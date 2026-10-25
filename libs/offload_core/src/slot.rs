//! Slot records from the pairing directory.
//!
//! The directory maps a four-symbol slot (the first half of a pairing code) to the id of the
//! device showing that code. Its answers are signed with the rendezvous server's Ed25519 key,
//! which every Offload build already pins, so a network attacker cannot redirect a pairing to
//! a device of their choosing. The other half of the code never reaches the directory.

use crate::{code, crypto};
use serde::{Deserialize, Serialize};

/// How far a client's clock may be ahead of the directory's before a fresh record looks stale.
pub const CLOCK_SKEW_SECS: i64 = 120;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SlotRecord {
    pub slot: String,
    pub id: String,
    pub expires_at: i64,
    /// Base64 detached Ed25519 signature over `crypto::slot_payload`.
    pub sig: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SlotError {
    BadSignature,
    Expired,
    Malformed,
}

impl std::fmt::Display for SlotError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SlotError::BadSignature => write!(f, "the pairing service's answer is not authentic"),
            SlotError::Expired => write!(f, "this code has expired"),
            SlotError::Malformed => write!(f, "the pairing service sent an invalid answer"),
        }
    }
}

impl std::error::Error for SlotError {}

impl SlotRecord {
    pub fn signed(
        slot: &str,
        id: &str,
        expires_at: i64,
        server_sk: &[u8],
    ) -> Result<Self, crypto::CryptoError> {
        let sig = crypto::sign(&crypto::slot_payload(slot, id, expires_at), server_sk)?;
        Ok(Self {
            slot: slot.to_owned(),
            id: id.to_owned(),
            expires_at,
            sig: crypto::b64_encode(&sig),
        })
    }

    /// Checks the record is authentic, well-formed, for the slot that was asked about, and
    /// still current.
    pub fn verify(&self, expected_slot: &str, server_pk: &[u8], now: i64) -> Result<(), SlotError> {
        if self.slot != expected_slot
            || !code::is_valid_slot(&self.slot)
            || !crate::is_valid_device_id(&self.id)
        {
            return Err(SlotError::Malformed);
        }
        let sig = crypto::b64_decode(&self.sig).ok_or(SlotError::Malformed)?;
        if !crypto::verify(
            &crypto::slot_payload(&self.slot, &self.id, self.expires_at),
            &sig,
            server_pk,
        ) {
            return Err(SlotError::BadSignature);
        }
        if now > self.expires_at + CLOCK_SKEW_SECS {
            return Err(SlotError::Expired);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSlotRequest {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSlotResponse {
    #[serde(flatten)]
    pub record: SlotRecord,
    /// Presented to delete the slot early, once pairing is done or cancelled.
    pub token: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::test_keypair;

    #[test]
    fn a_signed_record_verifies_and_tampering_is_detected() {
        let (pk, sk) = test_keypair();
        let r = SlotRecord::signed("K7QM", "123456789", 2000, &sk).unwrap();
        assert_eq!(r.verify("K7QM", &pk, 1000), Ok(()));

        let mut redirected = r.clone();
        redirected.id = "666666666".into();
        assert_eq!(redirected.verify("K7QM", &pk, 1000), Err(SlotError::BadSignature));

        let mut extended = r.clone();
        extended.expires_at = 9999;
        assert_eq!(extended.verify("K7QM", &pk, 1000), Err(SlotError::BadSignature));
    }

    #[test]
    fn a_record_from_another_key_or_for_another_slot_is_refused() {
        let (_, sk) = test_keypair();
        let (other_pk, _) = test_keypair();
        let r = SlotRecord::signed("K7QM", "123", 2000, &sk).unwrap();
        assert_eq!(r.verify("K7QM", &other_pk, 1000), Err(SlotError::BadSignature));
        let (pk, sk) = test_keypair();
        let r = SlotRecord::signed("K7QM", "123", 2000, &sk).unwrap();
        assert_eq!(r.verify("AAAA", &pk, 1000), Err(SlotError::Malformed));
    }

    #[test]
    fn stale_records_are_refused() {
        let (pk, sk) = test_keypair();
        let r = SlotRecord::signed("K7QM", "123", 1000, &sk).unwrap();
        assert_eq!(r.verify("K7QM", &pk, 1000 + CLOCK_SKEW_SECS), Ok(()));
        assert_eq!(r.verify("K7QM", &pk, 1001 + CLOCK_SKEW_SECS), Err(SlotError::Expired));
    }

    #[test]
    fn the_wire_format_is_flat_json() {
        let (_, sk) = test_keypair();
        let resp = CreateSlotResponse {
            record: SlotRecord::signed("K7QM", "123", 1000, &sk).unwrap(),
            token: "t".into(),
        };
        let v: serde_json::Value = serde_json::to_value(&resp).unwrap();
        assert_eq!(v["slot"], "K7QM");
        assert_eq!(v["token"], "t");
        let back: CreateSlotResponse = serde_json::from_value(v).unwrap();
        assert_eq!(back.record, resp.record);
    }
}
