//! What the UI calls, as JSON in and JSON out (see `main_offload_call` in `flutter_ffi.rs`).

use super::{ipc, pair};
use hbb_common::{config::Config, tokio};
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
        "my_id": crate::ipc::get_id(),
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

#[tokio::main(flavor = "current_thread")]
async fn run_probe(id: String, mode: String, command: String, wait: u64, relay: bool) -> Value {
    super::probe::probe(&id, &mode, &command, wait, relay).await
}

fn wait_for<F: Fn(&Value) -> bool>(method: &str, done: F, secs: u64) -> Value {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(secs);
    loop {
        let v: Value = serde_json::from_str(&call(method, "{}")).unwrap_or_default();
        if done(&v) || std::time::Instant::now() >= deadline {
            return v;
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}

fn state(v: &Value) -> &str {
    v.get("status")
        .and_then(|s| s.get("state"))
        .and_then(Value::as_str)
        .unwrap_or_default()
}

/// `Offload --offload <method> [json]`: the same operations as the UI, for scripts and support.
/// Besides the UI methods: `link {"code"}` links and waits for the outcome, `pair_wait` waits
/// for the next pairing event, and `probe {"id","mode":"files"|"run","command","wait"}` tests
/// a connection to a linked computer (`"relay": true` forces the relayed path).
pub fn cli(args: &[String]) -> String {
    let method = args.first().map(String::as_str).unwrap_or("info");
    let json_args = args.get(1).map(String::as_str).unwrap_or("{}");
    let parsed: Value = serde_json::from_str(json_args).unwrap_or_else(|_| json!({}));
    let get = |k: &str| parsed.get(k).and_then(Value::as_str).unwrap_or_default().to_owned();
    match method {
        "link" => {
            call("link_start", json_args);
            wait_for(
                "link_status",
                |v| matches!(state(v), "linked" | "failed" | "idle"),
                200,
            )
            .to_string()
        }
        "pair_wait" => wait_for(
            "pair_status",
            |v| !matches!(state(v), "waiting"),
            parsed.get("wait").and_then(Value::as_u64).unwrap_or(300),
        )
        .to_string(),
        "probe" => run_probe(
            get("id"),
            get("mode"),
            get("command"),
            parsed.get("wait").and_then(Value::as_u64).unwrap_or(60),
            parsed.get("relay").and_then(Value::as_bool).unwrap_or(false),
        )
        .to_string(),
        _ => call(method, json_args),
    }
}
