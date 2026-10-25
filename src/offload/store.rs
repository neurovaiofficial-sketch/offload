//! The trust store file, owned by the process that accepts connections.
//!
//! The service process reads and writes it directly; other processes go through [`super::ipc`].
//! Reads re-check the file's modification time, so a change made by another process (only
//! when the service was unreachable) is never shadowed by a stale cache.

use hbb_common::{config::Config, log};
use offload_core::{trust::StoreError, TrustStore};
use std::{path::PathBuf, sync::Mutex, time::SystemTime};

lazy_static::lazy_static! {
    static ref CACHE: Mutex<Option<(TrustStore, Option<SystemTime>)>> = Mutex::new(None);
}

pub fn path() -> PathBuf {
    Config::path(format!("{}_trust.json", crate::get_app_name()))
}

fn mtime() -> Option<SystemTime> {
    std::fs::metadata(path()).and_then(|m| m.modified()).ok()
}

/// The current store. An unreadable file yields an empty store, so it grants nothing.
pub fn snapshot() -> TrustStore {
    let current = mtime();
    let mut cache = CACHE.lock().unwrap();
    if let Some((store, seen)) = cache.as_ref() {
        if *seen == current {
            return store.clone();
        }
    }
    match TrustStore::load(&path()) {
        Ok(store) => {
            *cache = Some((store.clone(), current));
            store
        }
        Err(e) => {
            log::error!("offload: {e}");
            *cache = None;
            TrustStore::default()
        }
    }
}

/// Applies `f` to the store on disk and saves it. Refuses to touch an unreadable store, so a
/// corrupt file is never silently replaced; [`reset`] is the explicit way out.
pub fn update<R>(f: impl FnOnce(&mut TrustStore) -> Result<R, String>) -> Result<R, String> {
    let mut cache = CACHE.lock().unwrap();
    let mut store = TrustStore::load(&path()).map_err(|e| e.to_string())?;
    let out = f(&mut store)?;
    store.save(&path()).map_err(|e: StoreError| e.to_string())?;
    *cache = Some((store, mtime()));
    Ok(out)
}

/// Starts over with an empty store, keeping an unreadable file aside for inspection.
pub fn reset() -> Result<(), String> {
    let mut cache = CACHE.lock().unwrap();
    let p = path();
    if TrustStore::load(&p).is_err() {
        let aside = p.with_extension(format!("corrupt-{}", offload_core::now()));
        std::fs::rename(&p, &aside).map_err(|e| e.to_string())?;
        log::warn!("offload: moved unreadable trust store to {}", aside.display());
    }
    let store = TrustStore::default();
    store.save(&p).map_err(|e| e.to_string())?;
    *cache = Some((store, mtime()));
    Ok(())
}
