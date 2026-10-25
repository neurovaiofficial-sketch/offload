//! Signed payloads and the libsodium (Ed25519) primitives behind them.
//!
//! Every payload starts with a domain tag ending in NUL, followed by length-prefixed fields,
//! so no payload of one kind can be read as another, and no field boundary is ambiguous.
//! The tags start with `O` (0x4F): as protobuf that is field 9 with the invalid wire type 7,
//! so a signature made here can never pass for one over RustDesk's protobuf `IdPk` message,
//! which the same device key signs during the connection handshake.

use sodiumoxide::crypto::sign;
use std::sync::Once;

pub const AUTH_DOMAIN: &[u8] = b"OFFLOAD/AUTH/1\0";
pub const PAIR_DOMAIN: &[u8] = b"OFFLOAD/PAIR/1\0";
pub const SLOT_DOMAIN: &[u8] = b"OFFLOAD/SLOT/1\0";

pub const PUBLIC_KEY_LEN: usize = sign::PUBLICKEYBYTES;
pub const SECRET_KEY_LEN: usize = sign::SECRETKEYBYTES;
pub const SIGNATURE_LEN: usize = sign::SIGNATUREBYTES;

static INIT: Once = Once::new();

/// libsodium must be initialised once before use; it is idempotent and thread-safe.
pub fn init() {
    INIT.call_once(|| {
        if sodiumoxide::init().is_err() {
            // libsodium only fails to initialise when it cannot obtain entropy; every
            // operation below would then be unsafe, so refuse to continue.
            panic!("libsodium failed to initialise");
        }
    });
}

fn encode(domain: &[u8], fields: &[&[u8]]) -> Vec<u8> {
    let len = domain.len() + fields.iter().map(|f| 4 + f.len()).sum::<usize>();
    let mut out = Vec::with_capacity(len);
    out.extend_from_slice(domain);
    for f in fields {
        out.extend_from_slice(&(f.len() as u32).to_be_bytes());
        out.extend_from_slice(f);
    }
    out
}

/// What a controlling device signs to log in to a device it is paired with.
pub struct AuthContext<'a> {
    /// The id of the device being logged in to.
    pub target_id: &'a str,
    /// The id of the device logging in.
    pub my_id: &'a str,
    /// Salt and challenge from the controlled side's `Hash` message; the challenge is fresh
    /// for every connection, which is what makes a captured signature useless later.
    pub salt: &'a str,
    pub challenge: &'a str,
    /// The kind of session requested ("desktop", "file", "terminal", ...).
    pub conn_kind: &'a str,
    pub my_pk: &'a [u8],
}

pub fn auth_payload(c: &AuthContext) -> Vec<u8> {
    encode(
        AUTH_DOMAIN,
        &[
            c.target_id.as_bytes(),
            c.my_id.as_bytes(),
            c.salt.as_bytes(),
            c.challenge.as_bytes(),
            c.conn_kind.as_bytes(),
            c.my_pk,
        ],
    )
}

/// What a device signs when it asks to pair: it proves possession of the key it wants pinned.
pub struct PairContext<'a> {
    pub target_id: &'a str,
    pub my_id: &'a str,
    pub salt: &'a str,
    pub challenge: &'a str,
    /// The full normalised pairing code as typed by the user.
    pub code: &'a str,
    pub my_pk: &'a [u8],
    pub my_name: &'a str,
    pub my_platform: &'a str,
}

pub fn pair_payload(c: &PairContext) -> Vec<u8> {
    encode(
        PAIR_DOMAIN,
        &[
            c.target_id.as_bytes(),
            c.my_id.as_bytes(),
            c.salt.as_bytes(),
            c.challenge.as_bytes(),
            c.code.as_bytes(),
            c.my_pk,
            c.my_name.as_bytes(),
            c.my_platform.as_bytes(),
        ],
    )
}

pub fn slot_payload(slot: &str, id: &str, expires_at: i64) -> Vec<u8> {
    encode(
        SLOT_DOMAIN,
        &[slot.as_bytes(), id.as_bytes(), expires_at.to_string().as_bytes()],
    )
}

#[derive(Debug, PartialEq, Eq)]
pub enum CryptoError {
    BadSecretKey,
}

impl std::fmt::Display for CryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CryptoError::BadSecretKey => write!(f, "malformed Ed25519 secret key"),
        }
    }
}

impl std::error::Error for CryptoError {}

/// Detached Ed25519 signature with a libsodium-format secret key (seed || public key).
pub fn sign(payload: &[u8], secret_key: &[u8]) -> Result<Vec<u8>, CryptoError> {
    init();
    let sk = sign::SecretKey::from_slice(secret_key).ok_or(CryptoError::BadSecretKey)?;
    Ok(sign::sign_detached(payload, &sk).to_bytes().to_vec())
}

pub fn verify(payload: &[u8], signature: &[u8], public_key: &[u8]) -> bool {
    init();
    let Some(pk) = sign::PublicKey::from_slice(public_key) else {
        return false;
    };
    let Ok(sig) = sign::Signature::try_from(signature) else {
        return false;
    };
    sign::verify_detached(&sig, payload, &pk)
}

/// The public half of a libsodium secret key.
pub fn public_key_of(secret_key: &[u8]) -> Option<Vec<u8>> {
    if secret_key.len() != SECRET_KEY_LEN {
        return None;
    }
    Some(secret_key[SECRET_KEY_LEN - PUBLIC_KEY_LEN..].to_vec())
}

/// Constant-time equality for secrets of equal length.
pub fn secure_eq(a: &[u8], b: &[u8]) -> bool {
    init();
    a.len() == b.len() && sodiumoxide::utils::memcmp(a, b)
}

/// A uniformly random index below `bound`, from the OS CSPRNG.
pub fn random_below(bound: u32) -> u32 {
    init();
    sodiumoxide::randombytes::randombytes_uniform(bound)
}

pub fn random_bytes(n: usize) -> Vec<u8> {
    init();
    sodiumoxide::randombytes::randombytes(n)
}

pub fn b64_encode(bytes: &[u8]) -> String {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD.encode(bytes)
}

pub fn b64_decode(s: &str) -> Option<Vec<u8>> {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD.decode(s.trim()).ok()
}

/// A fresh (public key, libsodium secret key) pair.
pub fn generate_keypair() -> (Vec<u8>, Vec<u8>) {
    init();
    let (pk, sk) = sign::gen_keypair();
    (pk.0.to_vec(), sk.0.to_vec())
}

#[cfg(test)]
pub(crate) fn test_keypair() -> (Vec<u8>, Vec<u8>) {
    generate_keypair()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn auth<'a>(challenge: &'a str, kind: &'a str, pk: &'a [u8]) -> AuthContext<'a> {
        AuthContext {
            target_id: "123456789",
            my_id: "987654321",
            salt: "salt",
            challenge,
            conn_kind: kind,
            my_pk: pk,
        }
    }

    #[test]
    fn a_login_signature_verifies_only_for_its_own_challenge_and_session_kind() {
        let (pk, sk) = test_keypair();
        let payload = auth_payload(&auth("c1", "desktop", &pk));
        let sig = sign(&payload, &sk).unwrap();
        assert!(verify(&payload, &sig, &pk));
        assert!(!verify(&auth_payload(&auth("c2", "desktop", &pk)), &sig, &pk));
        assert!(!verify(&auth_payload(&auth("c1", "file", &pk)), &sig, &pk));
    }

    #[test]
    fn a_signature_does_not_verify_under_another_key() {
        let (pk, sk) = test_keypair();
        let (other_pk, _) = test_keypair();
        let payload = auth_payload(&auth("c", "desktop", &pk));
        let sig = sign(&payload, &sk).unwrap();
        assert!(!verify(&payload, &sig, &other_pk));
    }

    #[test]
    fn field_boundaries_are_unambiguous() {
        let a = slot_payload("AB", "C", 1);
        let b = slot_payload("A", "BC", 1);
        assert_ne!(a, b);
    }

    #[test]
    fn domains_never_collide() {
        let (pk, _) = test_keypair();
        let p = PairContext {
            target_id: "1",
            my_id: "2",
            salt: "s",
            challenge: "c",
            code: "ABCD1234",
            my_pk: &pk,
            my_name: "n",
            my_platform: "Windows",
        };
        let a = auth_payload(&auth("c", "desktop", &pk));
        assert_ne!(&pair_payload(&p)[..PAIR_DOMAIN.len()], &a[..AUTH_DOMAIN.len()]);
        assert_eq!(a[0], b'O');
    }

    #[test]
    fn malformed_keys_and_signatures_are_rejected_not_panicking() {
        assert_eq!(sign(b"x", &[0u8; 10]), Err(CryptoError::BadSecretKey));
        let (pk, sk) = test_keypair();
        let sig = sign(b"x", &sk).unwrap();
        assert!(!verify(b"x", &sig[..10], &pk));
        assert!(!verify(b"x", &sig, &pk[..5]));
        assert_eq!(public_key_of(&sk).unwrap(), pk);
    }

    #[test]
    fn secure_eq_requires_equal_length_and_content() {
        assert!(secure_eq(b"abcd", b"abcd"));
        assert!(!secure_eq(b"abcd", b"abce"));
        assert!(!secure_eq(b"abc", b"abcd"));
    }
}
