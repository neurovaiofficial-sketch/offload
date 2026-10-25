//! The controlling side of a session with a paired device.

use super::ipc;
use crate::client::{Interface, LoginConfigHandler};
use base::message_proto::{Hash, OffloadAuth};
use hbb_common::{config::Config, log, Stream};
use offload_core::crypto::{self, AuthContext};
use std::sync::{Arc, RwLock};

/// Answers the controlled side's login challenge with this device's signature.
///
/// The session only proceeds when the channel was end-to-end secured with exactly the key
/// pinned for this peer at pairing time, so neither the rendezvous server nor a relay can
/// stand in for it, and the signature is useless to anyone but that peer for this challenge.
pub async fn handle_hash(
    lc: Arc<RwLock<LoginConfigHandler>>,
    hash: Hash,
    interface: &impl Interface,
    peer: &mut Stream,
) -> bool {
    let (peer_id, conn_type, secured_pk) = {
        let lc = lc.read().unwrap();
        (lc.get_id().to_owned(), lc.conn_type, lc.offload_peer_pk.clone())
    };
    let trust = ipc::trust_snapshot().await;
    let Some(pinned) = trust.pinned_key(&peer_id) else {
        interface.msgbox("error", "Not linked", super::MSG_NOT_LINKED, "");
        return false;
    };
    let Some(secured_pk) = secured_pk else {
        interface.msgbox("error", "Connection Error", super::MSG_INSECURE, "");
        return false;
    };
    if !crypto::secure_eq(&pinned, &secured_pk) {
        log::warn!("offload: {peer_id} presented a key other than the one pinned at pairing");
        interface.msgbox("error", "Identity changed", super::MSG_KEY_CHANGED, "");
        return false;
    }
    let (sk, pk) = super::my_key_pair();
    let my_id = Config::get_id();
    let payload = crypto::auth_payload(&AuthContext {
        target_id: &peer_id,
        my_id: &my_id,
        salt: &hash.salt,
        challenge: &hash.challenge,
        conn_kind: super::kind_of_conn_type(conn_type),
        my_pk: &pk,
    });
    let sig = match crypto::sign(&payload, &sk) {
        Ok(s) => s,
        Err(e) => {
            interface.msgbox("error", "Connection Error", &e.to_string(), "");
            return false;
        }
    };
    lc.write().unwrap().offload_auth = Some(OffloadAuth {
        pk: pk.into(),
        sig: sig.into(),
        ..Default::default()
    });
    let msg = lc
        .read()
        .unwrap()
        .create_login_msg(String::new(), String::new(), Vec::new());
    if let Err(e) = peer.send(&msg).await {
        log::error!("offload: failed to send login: {e}");
        return false;
    }
    true
}
