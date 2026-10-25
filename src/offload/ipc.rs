//! Requests from the UI process to the process that accepts connections.
//!
//! On an installed system the service owns the trust store and the pending pairing code, so
//! the UI asks it over the existing local IPC channel (which RustDesk already restricts to the
//! app's own processes). When no service is reachable, store operations run in-process, but
//! a code can only be shown by a process that can accept the connection it invites.
//!
//! No request adds a device directly: devices enter the store only at the end of a pairing
//! exchange the service itself ran, so whatever can talk to this channel still cannot plant a
//! key without the other computer's owner approving it.

use super::{pair, server, store, MSG_UNATTENDED_LOCAL_ONLY};
use crate::ipc::{self as rd_ipc, Data};
use hbb_common::{config::Config, log};
use offload_core::{TrustStore, TrustedDevice};

pub const MSG_UNATTENDED_NEEDS_ADMIN: &str =
    "Turning on unattended access needs an administrator's approval on this computer.";
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
enum Request {
    Devices,
    PairStart,
    PairStatus,
    PairCancel,
    PairDecide { id: String, approve: bool },
    LinkStart { code: String },
    LinkStatus,
    LinkCancel,
    DeviceRemove { id: String },
    DeviceRename { id: String, name: String },
    DevicesClear,
    UnattendedSet { on: bool },
    Disconnect { id: Option<String> },
}

fn ok(v: Value) -> Value {
    let mut v = v;
    if let Some(m) = v.as_object_mut() {
        m.insert("ok".into(), true.into());
    }
    v
}

fn err(msg: impl ToString) -> Value {
    json!({ "ok": false, "error": msg.to_string() })
}

fn devices_json(s: &TrustStore) -> Value {
    json!({
        "unattended": s.unattended,
        "devices": s.devices,
        "my_id": Config::get_id(),
        "my_fingerprint": offload_core::crypto::fingerprint(&super::my_key_pair().1),
    })
}

fn has_remote_sessions() -> bool {
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    return crate::server::offload_has_sessions();
    #[cfg(any(target_os = "android", target_os = "ios"))]
    return false;
}

fn disconnect(id: Option<&str>) -> usize {
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    return crate::server::offload_disconnect(id);
    #[cfg(any(target_os = "android", target_os = "ios"))]
    return {
        let _ = id;
        0
    };
}

/// Serves one request in the process that owns the trust store. `local` is set when the UI
/// runs it in-process because no service answered; `privileged` when the caller runs with
/// administrator rights.
async fn serve(req: Request, local: bool, privileged: bool) -> Value {
    match req {
        Request::Devices => ok(devices_json(&store::snapshot())),
        Request::PairStart if local => err(
            "Offload is not running in the background on this computer yet. Finish setup, then try again.",
        ),
        Request::PairStart => match server::start_pairing().await {
            Ok(status) => ok(json!({ "status": status })),
            Err(e) => err(e),
        },
        Request::PairStatus => ok(json!({ "status": server::pair_status() })),
        Request::PairCancel => {
            server::cancel_pairing();
            ok(json!({}))
        }
        Request::PairDecide { id, approve } => {
            if server::decide(&id, approve) {
                ok(json!({}))
            } else {
                err("That request is no longer waiting for an answer.")
            }
        }
        Request::LinkStart { code } => {
            pair::start_link(code);
            ok(json!({ "status": pair::link_status() }))
        }
        Request::LinkStatus => ok(json!({ "status": pair::link_status() })),
        Request::LinkCancel => {
            pair::cancel_link();
            ok(json!({ "status": pair::link_status() }))
        }
        Request::DeviceRemove { id } => {
            let removed = store::update(|s| Ok(s.remove(&id)));
            let closed = disconnect(Some(&id));
            log::info!("offload: removed {id}, closed {closed} session(s)");
            match removed {
                Ok(_) => ok(json!({ "closed": closed })),
                Err(e) => err(e),
            }
        }
        Request::DeviceRename { id, name } => match store::update(|s| Ok(s.rename(&id, &name))) {
            Ok(true) => ok(json!({})),
            Ok(false) => err("no such device"),
            Err(e) => err(e),
        },
        Request::DevicesClear => {
            let res = store::update(|s| {
                s.clear();
                Ok(())
            })
            .or_else(|_| store::reset());
            let closed = disconnect(None);
            match res {
                Ok(()) => ok(json!({ "closed": closed })),
                Err(e) => err(e),
            }
        }
        Request::UnattendedSet { on } => {
            // Granting more access from inside a remote session would let the remote side
            // grant it to itself.
            if on && has_remote_sessions() {
                return err(MSG_UNATTENDED_LOCAL_ONLY);
            }
            // On Windows the service runs as LocalSystem: granting standing access to it must
            // take the same consent as any other administrative change (a UAC prompt).
            if on && cfg!(windows) && !privileged {
                return err(MSG_UNATTENDED_NEEDS_ADMIN);
            }
            match store::update(|s| {
                s.unattended = on;
                Ok(())
            }) {
                Ok(()) => ok(json!({ "unattended": on })),
                Err(e) => err(e),
            }
        }
        Request::Disconnect { id } => ok(json!({ "closed": disconnect(id.as_deref()) })),
    }
}

/// The IPC server side.
pub async fn handle(req: &str, privileged: bool) -> String {
    let resp = match serde_json::from_str::<Request>(req) {
        Ok(r) => serve(r, false, privileged).await,
        Err(e) => err(format!("bad request: {e}")),
    };
    resp.to_string()
}

fn timeout_ms(req: &Value) -> u64 {
    match req.get("op").and_then(Value::as_str) {
        // Waits for the pairing directory over the internet.
        Some("pair_start") => 20_000,
        Some("devices") | Some("pair_status") | Some("link_status") => 3_000,
        // Changes are flushed to disk before the service answers.
        _ => 10_000,
    }
}

/// Sends a request to the service, or serves it in-process if no service answers.
pub async fn call(req: Value) -> Value {
    match rd_ipc::connect(1000, "").await {
        Ok(mut c) => {
            if let Err(e) = c.send(&Data::Offload(req.to_string())).await {
                return err(format!("Offload background service: {e}"));
            }
            match c.next_timeout(timeout_ms(&req)).await {
                Ok(Some(Data::Offload(resp))) => {
                    serde_json::from_str(&resp).unwrap_or_else(|e| err(e))
                }
                Ok(_) => err("Offload background service sent an unexpected reply"),
                Err(e) => err(format!("Offload background service did not answer: {e}")),
            }
        }
        Err(_) => match serde_json::from_value::<Request>(req) {
            Ok(r) => serve(r, true, false).await,
            Err(e) => err(format!("bad request: {e}")),
        },
    }
}

/// The trust store as the service sees it.
pub async fn trust_snapshot() -> TrustStore {
    let resp = call(json!({ "op": "devices" })).await;
    let devices = resp
        .get("devices")
        .cloned()
        .and_then(|d| serde_json::from_value::<Vec<TrustedDevice>>(d).ok())
        .unwrap_or_default();
    TrustStore {
        unattended: resp.get("unattended").and_then(Value::as_bool).unwrap_or(false),
        devices,
        ..Default::default()
    }
}
