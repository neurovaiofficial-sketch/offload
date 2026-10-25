//! The controlled side's view of one pairing attempt.
//!
//! A code is good for one successful presentation, a few wrong guesses, and a few minutes.
//! The first correct presentation moves it to "awaiting approval"; from then on every other
//! request is refused, so one prompt can never be swapped for another while the owner reads it.

use crate::{code::PairingCode, trust::sanitize_name};
use serde::Serialize;

pub const CODE_TTL_SECS: i64 = 5 * 60;
pub const MAX_FAILED_ATTEMPTS: u32 = 3;
pub const APPROVAL_TIMEOUT_SECS: i64 = 2 * 60;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Requester {
    pub id: String,
    pub name: String,
    pub platform: String,
    #[serde(skip)]
    pub pk: Vec<u8>,
}

impl Requester {
    pub fn new(id: &str, name: &str, platform: &str, pk: Vec<u8>) -> Self {
        Self {
            id: id.to_owned(),
            name: sanitize_name(name, id),
            platform: sanitize_name(platform, ""),
            pk,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PairPhase {
    Waiting,
    AwaitingApproval { requester: Requester, since: i64 },
    Approved { requester: Requester },
    Denied,
    /// Too many wrong codes; this code is dead.
    Locked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttemptOutcome {
    /// The code matched; the owner is being asked.
    Accepted,
    WrongCode,
    /// The wrong guess that used up the last attempt; the code is now dead.
    Locked,
    Expired,
    /// The code was already used (or is being decided on).
    AlreadyUsed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum PairStatus {
    Idle,
    Waiting { code: String, expires_in: i64 },
    Approval { requester: Requester, expires_in: i64 },
    Approved { requester: Requester },
    Denied,
    Expired,
    Locked,
}

#[derive(Debug, Clone)]
pub struct PendingPair {
    code: PairingCode,
    expires_at: i64,
    failures: u32,
    phase: PairPhase,
    /// Lets the device delete its slot from the directory once it is done with it.
    pub slot_token: String,
}

impl PendingPair {
    pub fn new(code: PairingCode, slot_token: String, now: i64, ttl: i64) -> Self {
        Self {
            code,
            expires_at: now + ttl.clamp(30, CODE_TTL_SECS),
            failures: 0,
            phase: PairPhase::Waiting,
            slot_token,
        }
    }

    pub fn slot(&self) -> &str {
        self.code.slot()
    }

    pub fn phase(&self) -> &PairPhase {
        &self.phase
    }

    fn expire_if_due(&mut self, now: i64) {
        if let PairPhase::AwaitingApproval { since, .. } = self.phase {
            if now >= since + APPROVAL_TIMEOUT_SECS {
                self.phase = PairPhase::Denied;
            }
        }
    }

    pub fn is_expired(&self, now: i64) -> bool {
        matches!(self.phase, PairPhase::Waiting) && now >= self.expires_at
    }

    /// A pairing request presenting `presented` arrived from `requester`.
    pub fn attempt(
        &mut self,
        presented: &PairingCode,
        requester: Requester,
        now: i64,
    ) -> AttemptOutcome {
        self.expire_if_due(now);
        match self.phase {
            PairPhase::Waiting => {}
            PairPhase::Locked => return AttemptOutcome::Locked,
            _ => return AttemptOutcome::AlreadyUsed,
        }
        if now >= self.expires_at {
            return AttemptOutcome::Expired;
        }
        if !self.code.matches(presented) {
            self.failures += 1;
            if self.failures >= MAX_FAILED_ATTEMPTS {
                self.phase = PairPhase::Locked;
                return AttemptOutcome::Locked;
            }
            return AttemptOutcome::WrongCode;
        }
        self.phase = PairPhase::AwaitingApproval {
            requester,
            since: now,
        };
        AttemptOutcome::Accepted
    }

    /// The owner's answer. Only the request that is being shown can be decided, identified by
    /// the requester's id so a stale click cannot approve a different device.
    pub fn decide(&mut self, requester_id: &str, approve: bool, now: i64) -> bool {
        self.expire_if_due(now);
        let PairPhase::AwaitingApproval { requester, .. } = &self.phase else {
            return false;
        };
        if requester.id != requester_id {
            return false;
        }
        self.phase = if approve {
            PairPhase::Approved {
                requester: requester.clone(),
            }
        } else {
            PairPhase::Denied
        };
        true
    }

    /// Where the waiting connection stands: `Some(Some(r))` approved, `Some(None)` refused or
    /// timed out, `None` still waiting for the owner.
    pub fn decision(&mut self, now: i64) -> Option<Option<Requester>> {
        self.expire_if_due(now);
        match &self.phase {
            PairPhase::AwaitingApproval { .. } => None,
            PairPhase::Approved { requester } => Some(Some(requester.clone())),
            _ => Some(None),
        }
    }

    pub fn status(&mut self, now: i64) -> PairStatus {
        self.expire_if_due(now);
        match &self.phase {
            PairPhase::Waiting if now >= self.expires_at => PairStatus::Expired,
            PairPhase::Waiting => PairStatus::Waiting {
                code: self.code.display(),
                expires_in: self.expires_at - now,
            },
            PairPhase::AwaitingApproval { requester, since } => PairStatus::Approval {
                requester: requester.clone(),
                expires_in: since + APPROVAL_TIMEOUT_SECS - now,
            },
            PairPhase::Approved { requester } => PairStatus::Approved {
                requester: requester.clone(),
            },
            PairPhase::Denied => PairStatus::Denied,
            PairPhase::Locked => PairStatus::Locked,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn code(s: &str) -> PairingCode {
        PairingCode::parse(s).unwrap()
    }

    fn mac() -> Requester {
        Requester::new("987654321", "MacBook Pro", "Mac OS", vec![1; 32])
    }

    fn pending() -> PendingPair {
        PendingPair::new(code("K7QM-4ZP9"), "tok".into(), 1000, CODE_TTL_SECS)
    }

    #[test]
    fn the_right_code_asks_the_owner_and_approval_yields_the_requester() {
        let mut p = pending();
        assert_eq!(p.attempt(&code("K7QM-4ZP9"), mac(), 1010), AttemptOutcome::Accepted);
        assert_eq!(p.decision(1011), None);
        assert!(matches!(p.status(1011), PairStatus::Approval { .. }));
        assert!(p.decide("987654321", true, 1012));
        assert_eq!(p.decision(1013), Some(Some(mac())));
    }

    #[test]
    fn a_code_works_once() {
        let mut p = pending();
        assert_eq!(p.attempt(&code("K7QM-4ZP9"), mac(), 1010), AttemptOutcome::Accepted);
        let other = Requester::new("111", "Attacker", "Windows", vec![2; 32]);
        assert_eq!(p.attempt(&code("K7QM-4ZP9"), other, 1011), AttemptOutcome::AlreadyUsed);
        assert!(p.decide("987654321", false, 1012));
        assert_eq!(p.attempt(&code("K7QM-4ZP9"), mac(), 1013), AttemptOutcome::AlreadyUsed);
        assert_eq!(p.decision(1014), Some(None));
    }

    #[test]
    fn three_wrong_codes_kill_it() {
        let mut p = pending();
        assert_eq!(p.attempt(&code("K7QM-0000"), mac(), 1001), AttemptOutcome::WrongCode);
        assert_eq!(p.attempt(&code("K7QM-0001"), mac(), 1002), AttemptOutcome::WrongCode);
        assert_eq!(p.attempt(&code("K7QM-0002"), mac(), 1003), AttemptOutcome::Locked);
        assert_eq!(p.attempt(&code("K7QM-4ZP9"), mac(), 1004), AttemptOutcome::Locked);
        assert_eq!(p.status(1005), PairStatus::Locked);
    }

    #[test]
    fn codes_expire() {
        let mut p = pending();
        let late = 1000 + CODE_TTL_SECS;
        assert_eq!(p.attempt(&code("K7QM-4ZP9"), mac(), late), AttemptOutcome::Expired);
        assert_eq!(p.status(late), PairStatus::Expired);
        assert!(p.is_expired(late));
    }

    #[test]
    fn an_unanswered_prompt_times_out_as_a_refusal() {
        let mut p = pending();
        p.attempt(&code("K7QM-4ZP9"), mac(), 1010);
        let late = 1010 + APPROVAL_TIMEOUT_SECS;
        assert_eq!(p.decision(late), Some(None));
        assert!(!p.decide("987654321", true, late + 1));
    }

    #[test]
    fn only_the_shown_request_can_be_decided() {
        let mut p = pending();
        assert!(!p.decide("987654321", true, 1001));
        p.attempt(&code("K7QM-4ZP9"), mac(), 1010);
        assert!(!p.decide("someone-else", true, 1011));
        assert_eq!(p.decision(1012), None);
    }

    #[test]
    fn ttl_is_bounded() {
        let p = PendingPair::new(code("K7QM-4ZP9"), String::new(), 0, 10_000);
        assert!(!p.is_expired(CODE_TTL_SECS - 1));
        assert!(p.is_expired(CODE_TTL_SECS));
    }
}
