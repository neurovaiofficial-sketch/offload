//! The controlled side: showing a pairing code, judging pairing requests, verifying logins.
//!
//! This runs in the process that accepts connections (the service on an installed system).

use super::store;
use base::message_proto::{LoginRequest, OffloadPairRequest};
use hbb_common::{config::Config, log, tokio};
use offload_core::{
    crypto::{self, AuthContext, PairContext},
    pairing::{AttemptOutcome, PairPhase, PairStatus, PendingPair, Requester, CODE_TTL_SECS},
    ratelimit::RateLimiter,
    PairingCode, TrustedDevice,
};
use std::{sync::Mutex, time::Duration};

pub const MSG_NO_CODE: &str =
    "That code is not active. Ask for a new code on the other computer and try again.";
pub const MSG_WRONG_CODE: &str = "That code is not right. Check it and try again.";
pub const MSG_LOCKED: &str =
    "Too many wrong codes. Show a new code on the other computer and try again.";
pub const MSG_EXPIRED: &str =
    "That code has expired. Show a new code on the other computer and try again.";
pub const MSG_USED: &str = "That code has already been used. Show a new code and try again.";
pub const MSG_DENIED: &str = "The other computer did not approve the link.";
const MSG_BUSY: &str = "Too many link attempts. Wait a minute and try again.";
const MSG_BAD_REQUEST: &str = "The link request was not valid.";

lazy_static::lazy_static! {
    static ref PENDING: Mutex<Option<PendingPair>> = Mutex::new(None);
    static ref DECIDED: tokio::sync::Notify = tokio::sync::Notify::new();
    // Pairing requests across all peers; each one can put a prompt in front of the owner.
    static ref ATTEMPTS: Mutex<RateLimiter> = Mutex::new(RateLimiter::new(60, 10, 1));
}

fn status_now() -> PairStatus {
    match PENDING.lock().unwrap().as_mut() {
        Some(p) => p.status(offload_core::now()),
        None => PairStatus::Idle,
    }
}

pub fn pair_status() -> PairStatus {
    status_now()
}

/// Shows a fresh code: asks the directory for a slot, adds a secret and starts waiting.
pub async fn start_pairing() -> Result<PairStatus, String> {
    let id = Config::get_id();
    if id.is_empty() || !Config::get_key_confirmed() {
        return Err(
            "This computer is not connected to the Offload service yet. Check its internet connection and try again."
                .to_owned(),
        );
    }
    let created = super::directory::create_slot(&id)
        .await
        .map_err(|e| format!("Could not reach the pairing service: {e}"))?;
    let server_pk = super::server_public_key()
        .await
        .ok_or_else(|| "This build has no server key configured.".to_owned())?;
    let now = offload_core::now();
    created
        .record
        .verify(&created.record.slot, &server_pk, now)
        .map_err(|e| e.to_string())?;
    if created.record.id != id {
        return Err("The pairing service answered for another computer.".to_owned());
    }
    let code = PairingCode::generate(&created.record.slot).map_err(|e| e.to_string())?;
    let ttl = (created.record.expires_at - now).min(CODE_TTL_SECS);
    let previous = PENDING.lock().unwrap().replace(PendingPair::new(
        code,
        created.token,
        now,
        ttl,
    ));
    if let Some(old) = previous {
        release_slot(old);
    }
    DECIDED.notify_waiters();
    Ok(status_now())
}

fn release_slot(p: PendingPair) {
    let (slot, token) = (p.slot().to_owned(), p.slot_token.clone());
    tokio::spawn(async move { super::directory::delete_slot(&slot, &token).await });
}

pub fn cancel_pairing() {
    let previous = PENDING.lock().unwrap().take();
    if let Some(old) = previous {
        release_slot(old);
    }
    DECIDED.notify_waiters();
}

/// The owner's answer to the approval prompt.
pub fn decide(requester_id: &str, approve: bool) -> bool {
    let ok = PENDING
        .lock()
        .unwrap()
        .as_mut()
        .map(|p| p.decide(requester_id, approve, offload_core::now()))
        .unwrap_or(false);
    if ok {
        DECIDED.notify_waiters();
    }
    ok
}

/// Checks a pairing request and, when the code is right, puts it in front of the owner.
pub fn begin_pair(
    lr: &LoginRequest,
    req: &OffloadPairRequest,
    salt: &str,
    challenge: &str,
) -> Result<Requester, &'static str> {
    let now = offload_core::now();
    if !ATTEMPTS.lock().unwrap().allow("pair", now) {
        return Err(MSG_BUSY);
    }
    let Ok(presented) = PairingCode::parse(&req.code) else {
        return Err(MSG_WRONG_CODE);
    };
    if !offload_core::is_valid_device_id(&lr.my_id)
        || req.pk.len() != crypto::PUBLIC_KEY_LEN
        || lr.my_id == Config::get_id()
    {
        return Err(MSG_BAD_REQUEST);
    }
    let target_id = Config::get_id();
    let canonical = presented.canonical();
    let payload = crypto::pair_payload(&PairContext {
        target_id: &target_id,
        my_id: &lr.my_id,
        salt,
        challenge,
        code: &canonical,
        my_pk: &req.pk,
        my_name: &req.name,
        my_platform: &req.platform,
    });
    if !crypto::verify(&payload, &req.sig, &req.pk) {
        return Err(MSG_BAD_REQUEST);
    }
    let requester = Requester::new(&lr.my_id, &req.name, &req.platform, req.pk.to_vec());
    let mut pending = PENDING.lock().unwrap();
    let Some(p) = pending.as_mut() else {
        return Err(MSG_NO_CODE);
    };
    let outcome = p.attempt(&presented, requester.clone(), now);
    drop(pending);
    match outcome {
        AttemptOutcome::Accepted => {
            log::info!("offload: link request from {} awaiting approval", requester.id);
            DECIDED.notify_waiters();
            Ok(requester)
        }
        AttemptOutcome::WrongCode => Err(MSG_WRONG_CODE),
        AttemptOutcome::Locked => {
            log::warn!("offload: pairing code locked after repeated wrong attempts");
            Err(MSG_LOCKED)
        }
        AttemptOutcome::Expired => Err(MSG_EXPIRED),
        AttemptOutcome::AlreadyUsed => Err(MSG_USED),
    }
}

/// Waits for the owner to answer the prompt for `requester_id`; `None` means refused,
/// timed out, or the code was cancelled.
pub async fn await_decision(requester_id: &str) -> Option<Requester> {
    loop {
        let notified = DECIDED.notified();
        {
            let mut pending = PENDING.lock().unwrap();
            let p = pending.as_mut()?;
            let ours = match p.phase() {
                PairPhase::AwaitingApproval { requester, .. } | PairPhase::Approved { requester } => {
                    requester.id == requester_id
                }
                _ => false,
            };
            if !ours {
                return None;
            }
            if let Some(decision) = p.decision(offload_core::now()) {
                return decision;
            }
        }
        // The prompt times out on its own; waking periodically notices that.
        let _ = tokio::time::timeout(Duration::from_secs(1), notified).await;
    }
}

/// Pins the approved requester and retires the code.
pub fn finish_pair(requester: &Requester) -> Result<(), String> {
    let device = TrustedDevice::new(
        &requester.id,
        &requester.name,
        &requester.platform,
        &requester.pk,
        offload_core::now(),
    )
    .ok_or_else(|| MSG_BAD_REQUEST.to_owned())?;
    store::update(|s| s.upsert(device).map_err(|e| e.to_string()))?;
    log::info!("offload: linked with {}", requester.id);
    let slot = PENDING.lock().unwrap().as_ref().map(|p| (p.slot().to_owned(), p.slot_token.clone()));
    if let Some((slot, token)) = slot {
        tokio::spawn(async move { super::directory::delete_slot(&slot, &token).await });
    }
    Ok(())
}

pub enum LoginCheck {
    Trusted { name: String, unattended: bool },
    Rejected(&'static str),
}

/// Is this login signed by a device we are paired with, for this very challenge?
pub fn verify_login(lr: &LoginRequest, salt: &str, challenge: &str, secured: bool) -> LoginCheck {
    let Some(auth) = lr.offload_auth.as_ref() else {
        return LoginCheck::Rejected(super::MSG_NOT_LINKED);
    };
    if !secured {
        return LoginCheck::Rejected(super::MSG_INSECURE);
    }
    let trust = store::snapshot();
    let Some(device) = trust.find(&lr.my_id) else {
        return LoginCheck::Rejected(super::MSG_NOT_LINKED);
    };
    let Some(pk) = device.public_key() else {
        return LoginCheck::Rejected(super::MSG_NOT_LINKED);
    };
    if !crypto::secure_eq(&pk, &auth.pk) {
        return LoginCheck::Rejected(super::MSG_KEY_CHANGED);
    }
    let target_id = Config::get_id();
    let payload = crypto::auth_payload(&AuthContext {
        target_id: &target_id,
        my_id: &lr.my_id,
        salt,
        challenge,
        conn_kind: super::kind_of_login(lr),
        my_pk: &pk,
    });
    if !crypto::verify(&payload, &auth.sig, &pk) {
        return LoginCheck::Rejected(super::MSG_NOT_LINKED);
    }
    let id = lr.my_id.clone();
    std::thread::spawn(move || {
        if let Err(e) = store::update(|s| {
            s.touch(&id, offload_core::now());
            Ok(())
        }) {
            log::debug!("offload: could not record last connection: {e}");
        }
    });
    LoginCheck::Trusted {
        name: device.name.clone(),
        unattended: trust.unattended,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn statuses_serialise_for_the_ui() {
        let s = serde_json::to_value(PairStatus::Idle).unwrap();
        assert_eq!(s["state"], "idle");
    }
}
