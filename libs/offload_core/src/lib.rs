//! Offload device pairing and trust.
//!
//! Everything security-relevant that does not need the RustDesk runtime lives here so it can
//! be unit tested in isolation: pairing codes, the controlled side's pending-pair state
//! machine, the trusted-device store, the signed payloads that bind a device identity to one
//! login challenge, and the signed slot records returned by the pairing directory.
//!
//! Device identities are the Ed25519 key pairs RustDesk already keeps per device (the key the
//! rendezvous server certifies). Pairing pins the peer's public key on both sides; later
//! logins are authenticated by a detached signature over the controlled side's fresh login
//! challenge, never by the pairing code.

pub mod code;
pub mod crypto;
pub mod pairing;
pub mod ratelimit;
pub mod slot;
pub mod trust;

pub use code::{CodeError, PairingCode};
pub use pairing::{AttemptOutcome, PairPhase, PairStatus, PendingPair, Requester};
pub use slot::{SlotError, SlotRecord};
pub use trust::{TrustStore, TrustedDevice};

/// Seconds since the Unix epoch.
pub fn now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

/// RustDesk device ids are short alphanumeric strings; custom ids may add `-` and `_`.
pub fn is_valid_device_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 32
        && id
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
}
