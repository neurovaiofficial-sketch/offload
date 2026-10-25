//! What the UI calls, as JSON in and JSON out (see `main_offload_call` in `flutter_ffi.rs`).

use super::{ipc, pair};
use hbb_common::config::Config;
use serde_json::{json, Value};

#[tokio::main(flavor = "current_thread")]
async fn service(req: Value) -> Value {
    ipc::call(req).await
}

fn ok_with(v: Value) -> Value {
    json!({ "ok": true, "status": v })
}

fn info() -> Value {
    json!({
        "ok": true,
        "my_id": Config::get_id(),
        "name": super::my_name(),
        "platform": super::my_platform(),
        "version": crate::VERSION,
        "server": Config::get_rendezvous_server(),
        "pairing": super::directory::base_url(),
        "key_configured": !hbb_common::config::RS_PUB_KEY.is_empty() || !Config::get_option("key").is_empty(),
    })
}

/// `method` names an operation; `args` is a JSON object with its parameters.
pub fn call(method: &str, args: &str) -> String {
    let args: Value = serde_json::from_str(args).unwrap_or_else(|_| json!({}));
    let mut req = match args {
        Value::Object(m) => Value::Object(m),
        _ => json!({}),
    };
    let resp = match method {
        "info" => info(),
        "link_start" => {
            let code = req.get("code").and_then(Value::as_str).unwrap_or_default();
            pair::start_link(code.to_owned());
            ok_with(json!(pair::link_status()))
        }
        "link_status" => ok_with(json!(pair::link_status())),
        "link_cancel" => {
            pair::cancel_link();
            ok_with(json!(pair::link_status()))
        }
        "devices" | "pair_start" | "pair_status" | "pair_cancel" | "pair_decide"
        | "device_remove" | "device_rename" | "devices_clear" | "unattended_set"
        | "disconnect" => {
            if let Some(m) = req.as_object_mut() {
                m.insert("op".into(), method.into());
            }
            service(req)
        }
        _ => json!({ "ok": false, "error": format!("unknown method {method}") }),
    };
    resp.to_string()
}
