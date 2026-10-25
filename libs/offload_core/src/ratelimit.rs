//! Sliding-window rate limiting keyed by an arbitrary string (an IP address, a device id).

use std::collections::{HashMap, VecDeque};

pub struct RateLimiter {
    window_secs: i64,
    max_events: usize,
    max_keys: usize,
    events: HashMap<String, VecDeque<i64>>,
}

impl RateLimiter {
    /// At most `max_events` per `window_secs` per key. `max_keys` bounds memory; when it is
    /// reached, idle keys are pruned, and if none are idle new keys are refused (fail closed).
    pub fn new(window_secs: i64, max_events: usize, max_keys: usize) -> Self {
        Self {
            window_secs,
            max_events,
            max_keys,
            events: HashMap::new(),
        }
    }

    /// Records an event for `key` and says whether it is within the limit.
    pub fn allow(&mut self, key: &str, now: i64) -> bool {
        let horizon = now - self.window_secs;
        if !self.events.contains_key(key) && self.events.len() >= self.max_keys {
            self.prune(now);
            if self.events.len() >= self.max_keys {
                return false;
            }
        }
        let q = self.events.entry(key.to_owned()).or_default();
        while q.front().is_some_and(|&t| t <= horizon) {
            q.pop_front();
        }
        if q.len() >= self.max_events {
            return false;
        }
        q.push_back(now);
        true
    }

    pub fn prune(&mut self, now: i64) {
        let horizon = now - self.window_secs;
        self.events
            .retain(|_, q| q.back().is_some_and(|&t| t > horizon));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn limits_per_key_within_the_window() {
        let mut r = RateLimiter::new(60, 3, 100);
        assert!(r.allow("a", 0));
        assert!(r.allow("a", 1));
        assert!(r.allow("a", 2));
        assert!(!r.allow("a", 3));
        assert!(r.allow("b", 3));
        assert!(r.allow("a", 61));
    }

    #[test]
    fn memory_is_bounded_and_idle_keys_are_reclaimed() {
        let mut r = RateLimiter::new(10, 1, 2);
        assert!(r.allow("a", 0));
        assert!(r.allow("b", 0));
        assert!(!r.allow("c", 5));
        assert!(r.allow("c", 11));
    }
}
