//! The trusted-device store.
//!
//! Only public keys are stored: a paired device is authenticated by a signature with its own
//! private key, so nothing in this file lets anyone log in to anything. It is still
//! integrity-sensitive (adding an entry grants access), which is why it lives in the
//! service's own configuration directory and is changed only through the service process.

use crate::crypto;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{self, Write},
    path::Path,
};

pub const STORE_VERSION: u32 = 1;
pub const MAX_DEVICES: usize = 64;
const MAX_NAME_CHARS: usize = 64;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustedDevice {
    pub id: String,
    pub name: String,
    pub platform: String,
    /// Base64 Ed25519 public key pinned at pairing time.
    pub pk: String,
    pub added_at: i64,
    #[serde(default)]
    pub last_seen: i64,
}

impl TrustedDevice {
    pub fn new(id: &str, name: &str, platform: &str, pk: &[u8], now: i64) -> Option<Self> {
        if !crate::is_valid_device_id(id) || pk.len() != crypto::PUBLIC_KEY_LEN {
            return None;
        }
        Some(Self {
            id: id.to_owned(),
            name: sanitize_name(name, id),
            platform: sanitize_name(platform, ""),
            pk: crypto::b64_encode(pk),
            added_at: now,
            last_seen: 0,
        })
    }

    pub fn public_key(&self) -> Option<Vec<u8>> {
        crypto::b64_decode(&self.pk).filter(|k| k.len() == crypto::PUBLIC_KEY_LEN)
    }
}

/// Names come from the other device and are shown in approval prompts, so they are reduced
/// to printable text without bidirectional overrides that could disguise them.
pub fn sanitize_name(name: &str, fallback: &str) -> String {
    let cleaned: String = name
        .chars()
        .filter(|c| {
            !c.is_control()
                && !matches!(*c, '\u{200B}'..='\u{200F}' | '\u{202A}'..='\u{202E}' | '\u{2066}'..='\u{2069}' | '\u{FEFF}')
        })
        .take(MAX_NAME_CHARS)
        .collect();
    let cleaned = cleaned.trim();
    if cleaned.is_empty() {
        fallback.to_owned()
    } else {
        cleaned.to_owned()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustStore {
    #[serde(default)]
    pub version: u32,
    /// "Allow my trusted devices to connect when I am away." Off unless the owner turns it on.
    #[serde(default)]
    pub unattended: bool,
    #[serde(default)]
    pub devices: Vec<TrustedDevice>,
}

impl Default for TrustStore {
    fn default() -> Self {
        Self {
            version: STORE_VERSION,
            unattended: false,
            devices: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub enum StoreError {
    Io(io::Error),
    Corrupt(String),
    Full,
}

impl std::fmt::Display for StoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StoreError::Io(e) => write!(f, "{e}"),
            StoreError::Corrupt(e) => write!(f, "trusted device list is unreadable: {e}"),
            StoreError::Full => write!(f, "too many linked devices (max {MAX_DEVICES})"),
        }
    }
}

impl std::error::Error for StoreError {}

impl From<io::Error> for StoreError {
    fn from(e: io::Error) -> Self {
        StoreError::Io(e)
    }
}

impl TrustStore {
    /// A missing file is an empty store. An unreadable one is an error: the caller must not
    /// guess, and must not overwrite it with an empty store either.
    pub fn load(path: &Path) -> Result<Self, StoreError> {
        let text = match fs::read_to_string(path) {
            Ok(t) => t,
            Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(Self::default()),
            Err(e) => return Err(e.into()),
        };
        let mut store: TrustStore =
            serde_json::from_str(&text).map_err(|e| StoreError::Corrupt(e.to_string()))?;
        // Entries that cannot be used for verification are dropped rather than trusted.
        store
            .devices
            .retain(|d| crate::is_valid_device_id(&d.id) && d.public_key().is_some());
        Ok(store)
    }

    /// Writes atomically: a crash leaves either the old or the new store, never a torn one.
    pub fn save(&self, path: &Path) -> Result<(), StoreError> {
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir)?;
        }
        let tmp = path.with_extension("tmp");
        {
            let mut f = fs::File::create(&tmp)?;
            let json = serde_json::to_vec_pretty(self)
                .map_err(|e| StoreError::Corrupt(e.to_string()))?;
            f.write_all(&json)?;
            f.sync_all()?;
        }
        fs::rename(&tmp, path)?;
        Ok(())
    }

    pub fn find(&self, id: &str) -> Option<&TrustedDevice> {
        self.devices.iter().find(|d| d.id == id)
    }

    pub fn pinned_key(&self, id: &str) -> Option<Vec<u8>> {
        self.find(id).and_then(TrustedDevice::public_key)
    }

    /// Adds a device, replacing any earlier pairing with the same id (re-pairing after a
    /// reinstall gives the device a new key).
    pub fn upsert(&mut self, device: TrustedDevice) -> Result<(), StoreError> {
        if let Some(existing) = self.devices.iter_mut().find(|d| d.id == device.id) {
            *existing = device;
            return Ok(());
        }
        if self.devices.len() >= MAX_DEVICES {
            return Err(StoreError::Full);
        }
        self.devices.push(device);
        Ok(())
    }

    pub fn remove(&mut self, id: &str) -> bool {
        let before = self.devices.len();
        self.devices.retain(|d| d.id != id);
        before != self.devices.len()
    }

    pub fn rename(&mut self, id: &str, name: &str) -> bool {
        match self.devices.iter_mut().find(|d| d.id == id) {
            Some(d) => {
                d.name = sanitize_name(name, &d.id.clone());
                true
            }
            None => false,
        }
    }

    pub fn touch(&mut self, id: &str, now: i64) {
        if let Some(d) = self.devices.iter_mut().find(|d| d.id == id) {
            d.last_seen = now;
        }
    }

    /// Removes every device and turns unattended access off.
    pub fn clear(&mut self) {
        self.devices.clear();
        self.unattended = false;
    }

    /// Is `pk` the key pinned for `id`?
    pub fn is_pinned(&self, id: &str, pk: &[u8]) -> bool {
        self.pinned_key(id)
            .map(|k| crypto::secure_eq(&k, pk))
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::test_keypair;

    fn device(id: &str) -> TrustedDevice {
        let (pk, _) = test_keypair();
        TrustedDevice::new(id, "Windows Desktop", "Windows", &pk, 100).unwrap()
    }

    #[test]
    fn store_round_trips_and_defaults_to_attended() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("Offload_trust.json");
        let empty = TrustStore::load(&path).unwrap();
        assert!(!empty.unattended);
        assert!(empty.devices.is_empty());

        let mut s = TrustStore::default();
        s.upsert(device("123456789")).unwrap();
        s.unattended = true;
        s.save(&path).unwrap();
        let back = TrustStore::load(&path).unwrap();
        assert_eq!(back, s);
    }

    #[test]
    fn a_corrupt_store_is_an_error_not_an_empty_store() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("Offload_trust.json");
        fs::write(&path, "{ not json").unwrap();
        assert!(matches!(TrustStore::load(&path), Err(StoreError::Corrupt(_))));
    }

    #[test]
    fn entries_with_unusable_keys_are_not_trusted() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("Offload_trust.json");
        fs::write(
            &path,
            r#"{"version":1,"unattended":false,"devices":[{"id":"1","name":"x","platform":"","pk":"AAAA","added_at":0}]}"#,
        )
        .unwrap();
        assert!(TrustStore::load(&path).unwrap().devices.is_empty());
    }

    #[test]
    fn repairing_replaces_the_pinned_key_and_revoking_removes_it() {
        let mut s = TrustStore::default();
        let first = device("123");
        s.upsert(first.clone()).unwrap();
        let second = device("123");
        s.upsert(second.clone()).unwrap();
        assert_eq!(s.devices.len(), 1);
        assert!(s.is_pinned("123", &second.public_key().unwrap()));
        assert!(!s.is_pinned("123", &first.public_key().unwrap()));
        assert!(s.remove("123"));
        assert!(!s.is_pinned("123", &second.public_key().unwrap()));
        assert!(!s.remove("123"));
    }

    #[test]
    fn clear_also_turns_off_unattended_access() {
        let mut s = TrustStore::default();
        s.upsert(device("1")).unwrap();
        s.unattended = true;
        s.clear();
        assert!(s.devices.is_empty());
        assert!(!s.unattended);
    }

    #[test]
    fn the_store_is_bounded() {
        let mut s = TrustStore::default();
        for i in 0..MAX_DEVICES {
            s.upsert(device(&i.to_string())).unwrap();
        }
        assert!(matches!(s.upsert(device("overflow")), Err(StoreError::Full)));
        // Replacing an existing entry is still allowed when full.
        assert!(s.upsert(device("0")).is_ok());
    }

    #[test]
    fn invalid_ids_and_keys_are_refused() {
        let (pk, _) = test_keypair();
        assert!(TrustedDevice::new("", "n", "p", &pk, 0).is_none());
        assert!(TrustedDevice::new("a b", "n", "p", &pk, 0).is_none());
        assert!(TrustedDevice::new("123", "n", "p", &pk[..31], 0).is_none());
    }

    #[test]
    fn names_are_sanitised_for_display() {
        assert_eq!(sanitize_name("Evil\u{202E}cod.exe\n", "x"), "Evilcod.exe");
        assert_eq!(sanitize_name("   ", "fallback"), "fallback");
        assert_eq!(sanitize_name(&"a".repeat(200), "x").len(), 64);
    }
}
