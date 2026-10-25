//! Offload: device pairing, trusted devices and device-signature logins on top of RustDesk.
//!
//! * The computer showing a code ("this computer's code") gets a slot from the pairing
//!   directory, adds a secret of its own and waits; see [`server`].
//! * The computer the code is typed into ("link a computer") resolves the slot, connects
//!   through the normal rendezvous/relay path and presents the code inside the encrypted
//!   session; see [`pair`].
//! * After the owner approves, both pin each other's Ed25519 identity key in the trust store
//!   ([`store`]). From then on a session logs in with a signature over the controlled side's
//!   fresh challenge ([`client`] signs, [`server`] verifies). Passwords are not accepted.

pub mod api;
pub mod client;
pub mod directory;
pub mod ipc;
pub mod pair;
pub mod server;
pub mod store;

use hbb_common::config::Config;

/// Every session goes through Offload's device-signature login; RustDesk password and
/// click-to-accept logins from unknown devices are refused.
pub const ENFORCED: bool = true;

pub const MSG_NOT_LINKED: &str =
    "This computer is not linked with yours. Link them with a code first.";
pub const MSG_KEY_CHANGED: &str = "This computer's identity has changed since you linked it (it may have been reinstalled). Remove it and link it again.";
pub const MSG_INSECURE: &str =
    "Could not set up an end-to-end encrypted connection to this computer, so Offload did not connect.";
pub const MSG_UNATTENDED_LOCAL_ONLY: &str =
    "For your security, this can only be changed while nobody is connected to this computer remotely.";

/// The session kinds a login signature is bound to. Both ends must derive the same string:
/// the controller from its connection type, the controlled side from the login request.
pub fn kind_of_conn_type(conn_type: hbb_common::rendezvous_proto::ConnType) -> &'static str {
    use hbb_common::rendezvous_proto::ConnType;
    match conn_type {
        ConnType::FILE_TRANSFER => "file",
        ConnType::PORT_FORWARD | ConnType::RDP => "forward",
        ConnType::VIEW_CAMERA => "camera",
        ConnType::TERMINAL => "terminal",
        _ => "desktop",
    }
}

pub fn kind_of_login(lr: &base::message_proto::LoginRequest) -> &'static str {
    use base::message_proto::login_request::Union;
    match lr.union.as_ref() {
        Some(Union::FileTransfer(_)) => "file",
        Some(Union::PortForward(_)) => "forward",
        Some(Union::ViewCamera(_)) => "camera",
        Some(Union::Terminal(_)) => "terminal",
        Some(Union::OffloadPair(_)) => "pair",
        _ => "desktop",
    }
}

pub fn my_name() -> String {
    let name = crate::common::hostname();
    if name.trim().is_empty() {
        crate::username()
    } else {
        name
    }
}

pub fn my_platform() -> String {
    hbb_common::whoami::platform().to_string()
}

/// This device's identity: (secret key, public key) as registered with the rendezvous server.
pub fn my_key_pair() -> (Vec<u8>, Vec<u8>) {
    Config::get_key_pair()
}

/// The rendezvous server's public key this build trusts (a custom key set in advanced
/// settings wins over the built-in one, exactly as for sessions).
pub async fn server_public_key() -> Option<Vec<u8>> {
    let key = crate::common::get_key(true).await;
    crate::common::get_rs_pk(&key).map(|pk| pk.0.to_vec())
}
