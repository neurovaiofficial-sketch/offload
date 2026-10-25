//! The side that types a code: finds the computer showing it and asks to be linked.
//! Runs in the service, which owns the trust store the result is written to.

use super::{directory, headless::HeadlessUi, store};
use crate::client::Client;
use base::message_proto::*;
use hbb_common::{
    config::Config,
    get_time, log,
    protobuf::Message as _,
    rendezvous_proto::ConnType,
    tokio::{self, time::Duration},
    Stream,
};
use offload_core::{
    crypto::{self, PairContext},
    PairingCode, TrustedDevice,
};
use serde::Serialize;
use std::sync::Mutex;

/// Long enough for the owner of the other computer to walk over and approve.
const LINK_DEADLINE: Duration = Duration::from_secs(150);
/// Relays drop a connection after 30 s of silence; the other side is silent while its owner
/// decides, so this side keeps the line warm.
const KEEPALIVE: Duration = Duration::from_secs(10);

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum LinkStatus {
    Idle,
    Resolving,
    Connecting,
    Waiting,
    Linked {
        id: String,
        name: String,
        platform: String,
    },
    Failed {
        error: String,
    },
}

lazy_static::lazy_static! {
    static ref LINK: Mutex<(u64, LinkStatus)> = Mutex::new((0, LinkStatus::Idle));
}

fn set_status(generation: u64, status: LinkStatus) -> bool {
    let mut link = LINK.lock().unwrap();
    if link.0 != generation {
        return false;
    }
    link.1 = status;
    true
}

fn is_current(generation: u64) -> bool {
    LINK.lock().unwrap().0 == generation
}

pub fn link_status() -> LinkStatus {
    LINK.lock().unwrap().1.clone()
}

pub fn cancel_link() {
    let mut link = LINK.lock().unwrap();
    link.0 += 1;
    link.1 = LinkStatus::Idle;
}

/// Starts linking with the computer showing `code`; progress is read with [`link_status`].
pub fn start_link(code: String) {
    let generation = {
        let mut link = LINK.lock().unwrap();
        link.0 += 1;
        link.1 = LinkStatus::Resolving;
        link.0
    };
    std::thread::spawn(move || run_link(generation, code));
}

#[tokio::main(flavor = "current_thread")]
async fn run_link(generation: u64, code: String) {
    let status = match link(generation, &code).await {
        Ok(device) => LinkStatus::Linked {
            id: device.id,
            name: device.name,
            platform: device.platform,
        },
        Err(error) => {
            log::info!("offload: linking failed: {error}");
            LinkStatus::Failed { error }
        }
    };
    set_status(generation, status);
}

async fn link(generation: u64, code: &str) -> Result<TrustedDevice, String> {
    let code = PairingCode::parse(code)
        .map_err(|_| "That code doesn't look right. A code has 8 letters and digits, like K7QM-4ZP9.".to_owned())?;
    let record = directory::lookup_slot(code.slot())
        .await
        .map_err(|e| format!("Could not reach the pairing service: {e}"))?
        .ok_or_else(|| super::server::MSG_NO_CODE.to_owned())?;
    let server_pk = super::server_public_key()
        .await
        .ok_or_else(|| "This build has no server key configured.".to_owned())?;
    record
        .verify(code.slot(), &server_pk, offload_core::now())
        .map_err(|e| e.to_string())?;
    let peer_id = record.id;
    if peer_id == Config::get_id() {
        return Err("That is this computer's own code. Type it on the other computer.".to_owned());
    }
    if !set_status(generation, LinkStatus::Connecting) {
        return Err("cancelled".to_owned());
    }

    let ui = HeadlessUi::new(&peer_id, ConnType::DEFAULT_CONN, false);
    let key = crate::common::get_key(true).await;
    let ((mut stream, _direct, peer_pk, _kcp, _typ), _) =
        Client::start(&peer_id, &key, "", ConnType::DEFAULT_CONN, ui.clone())
            .await
            .map_err(|e| format!("Could not reach the other computer: {e}"))?;
    let peer_pk = match peer_pk {
        Some(pk) if stream.is_secured() => pk,
        _ => return Err(super::MSG_INSECURE.to_owned()),
    };

    let result = exchange(generation, &mut stream, &peer_id, &code).await?;
    let device = TrustedDevice::new(
        &peer_id,
        &result.name,
        &result.platform,
        &peer_pk,
        offload_core::now(),
    )
    .ok_or_else(|| "The other computer sent an invalid identity.".to_owned())?;
    let pinned = device.clone();
    store::update(|s| s.upsert(pinned).map_err(|e| e.to_string()))
        .map_err(|e| format!("Linked, but could not save the link on this computer: {e}"))?;
    log::info!("offload: linked with {peer_id}");
    Ok(device)
}

async fn exchange(
    generation: u64,
    stream: &mut Stream,
    peer_id: &str,
    code: &PairingCode,
) -> Result<OffloadPairResult, String> {
    let deadline = tokio::time::Instant::now() + LINK_DEADLINE;
    let mut keepalive = tokio::time::interval(KEEPALIVE);
    keepalive.tick().await;
    loop {
        if !is_current(generation) {
            return Err("cancelled".to_owned());
        }
        tokio::select! {
            _ = tokio::time::sleep_until(deadline) => {
                return Err("The other computer did not answer in time.".to_owned());
            }
            _ = keepalive.tick() => {
                let mut msg = Message::new();
                msg.set_test_delay(TestDelay {
                    time: get_time(),
                    from_client: true,
                    ..Default::default()
                });
                stream.send(&msg).await.map_err(|e| e.to_string())?;
            }
            res = stream.next() => {
                let bytes = match res {
                    Some(Ok(b)) => b,
                    Some(Err(e)) => return Err(format!("Connection lost: {e}")),
                    None => return Err("The other computer closed the connection.".to_owned()),
                };
                let Ok(msg) = Message::parse_from_bytes(&bytes) else {
                    continue;
                };
                match msg.union {
                    Some(message::Union::Hash(hash)) => {
                        let req = pair_request(peer_id, code, &hash)?;
                        stream.send(&req).await.map_err(|e| e.to_string())?;
                        set_status(generation, LinkStatus::Waiting);
                    }
                    Some(message::Union::LoginResponse(lr)) => match lr.union {
                        Some(login_response::Union::OffloadPair(res)) => return Ok(res),
                        Some(login_response::Union::Error(e)) => return Err(e),
                        _ => return Err("The other computer is not running Offload.".to_owned()),
                    },
                    Some(message::Union::Misc(m)) => {
                        if let Some(misc::Union::CloseReason(r)) = m.union {
                            return Err(r);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

fn pair_request(peer_id: &str, code: &PairingCode, hash: &Hash) -> Result<Message, String> {
    let (sk, pk) = super::my_key_pair();
    let my_id = Config::get_id();
    let name = super::my_name();
    let platform = super::my_platform();
    let canonical = code.canonical();
    let payload = crypto::pair_payload(&PairContext {
        target_id: peer_id,
        my_id: &my_id,
        salt: &hash.salt,
        challenge: &hash.challenge,
        code: &canonical,
        my_pk: &pk,
        my_name: &name,
        my_platform: &platform,
    });
    let sig = crypto::sign(&payload, &sk).map_err(|e| e.to_string())?;
    let mut lr = LoginRequest {
        username: peer_id.to_owned(),
        my_id,
        my_name: name.clone(),
        my_platform: platform.clone(),
        version: crate::VERSION.to_string(),
        session_id: u64::from_le_bytes(
            crypto::random_bytes(8)
                .try_into()
                .unwrap_or_default(),
        ),
        ..Default::default()
    };
    lr.set_offload_pair(OffloadPairRequest {
        code: canonical,
        pk: pk.into(),
        sig: sig.into(),
        name,
        platform,
        ..Default::default()
    });
    let mut msg = Message::new();
    msg.set_login_request(lr);
    Ok(msg)
}
