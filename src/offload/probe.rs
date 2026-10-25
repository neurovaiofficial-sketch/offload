//! "Test connection": logs in to a linked computer exactly as a session would and, depending
//! on the mode, lists its home folder or runs one command in a terminal there. Backs the
//! command line (`--offload probe ...`) and connection diagnostics.

use super::headless::HeadlessUi;
use crate::client::{Client, Interface, LOGIN_MSG_NO_PASSWORD_ACCESS};
use base::message_proto::*;
use hbb_common::{
    protobuf::Message as _,
    rendezvous_proto::ConnType,
    tokio::{self, time::Duration},
};
use serde_json::{json, Value};

const END_MARK: &str = "__OFFLOAD_END__";

fn strip_ansi(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\u{1b}' {
            if chars.peek() == Some(&'[') {
                chars.next();
                while let Some(&n) = chars.peek() {
                    chars.next();
                    if ('@'..='~').contains(&n) {
                        break;
                    }
                }
            } else if chars.peek() == Some(&']') {
                while let Some(n) = chars.next() {
                    if n == '\u{7}' {
                        break;
                    }
                }
            } else {
                chars.next();
            }
        } else if c != '\r' {
            out.push(c);
        }
    }
    out
}

/// `mode` is "files" (log in and list the home folder) or "run" (log in, open a terminal and
/// run `command`); `relay` forces the relayed path. Returns what happened as JSON.
pub async fn probe(peer_id: &str, mode: &str, command: &str, wait_secs: u64, relay: bool) -> Value {
    match probe_(peer_id, mode, command, wait_secs, relay).await {
        Ok(v) => v,
        Err(e) => json!({ "ok": false, "error": e }),
    }
}

async fn probe_(
    peer_id: &str,
    mode: &str,
    command: &str,
    wait_secs: u64,
    relay: bool,
) -> Result<Value, String> {
    let conn_type = match mode {
        "files" => ConnType::FILE_TRANSFER,
        "run" => ConnType::TERMINAL,
        _ => return Err(format!("unknown mode {mode}")),
    };
    let ui = HeadlessUi::new(peer_id, conn_type, relay);
    let key = crate::common::get_key(true).await;
    let started = std::time::Instant::now();
    let ((mut stream, direct, pk, _kcp, transport), _) =
        Client::start(peer_id, &key, "", conn_type, ui.clone())
            .await
            .map_err(|e| e.to_string())?;
    let lc = ui.get_lch();
    lc.write().unwrap().offload_peer_pk = if stream.is_secured() { pk } else { None };
    let deadline = tokio::time::Instant::now() + Duration::from_secs(wait_secs.max(5));
    let mut waited_for_approval = false;
    let mut peer = json!({});
    let mut output = String::new();
    loop {
        let res = tokio::time::timeout_at(deadline, stream.next())
            .await
            .map_err(|_| {
                if waited_for_approval {
                    "Nobody accepted the session on the other computer in time.".to_owned()
                } else {
                    "Timed out.".to_owned()
                }
            })?;
        let bytes = match res {
            Some(Ok(b)) => b,
            Some(Err(e)) => return Err(format!("Connection lost: {e}")),
            None => return Err("The other computer closed the connection.".to_owned()),
        };
        let Ok(msg) = Message::parse_from_bytes(&bytes) else {
            continue;
        };
        match msg.union {
            Some(message::Union::Hash(hash)) => {
                if !super::client::handle_hash(lc.clone(), hash, &ui, &mut stream).await {
                    return Err(ui.last_message());
                }
            }
            Some(message::Union::LoginResponse(lr)) => match lr.union {
                Some(login_response::Union::Error(e)) if e == LOGIN_MSG_NO_PASSWORD_ACCESS => {
                    waited_for_approval = true;
                }
                Some(login_response::Union::Error(e)) => return Err(e),
                Some(login_response::Union::PeerInfo(pi)) => {
                    peer = json!({
                        "hostname": pi.hostname,
                        "platform": pi.platform,
                        "username": pi.username,
                        "version": pi.version,
                    });
                    if mode == "run" {
                        let mut action = TerminalAction::new();
                        action.set_open(OpenTerminal {
                            terminal_id: 1,
                            rows: 40,
                            cols: 200,
                            ..Default::default()
                        });
                        let mut out = Message::new();
                        out.set_terminal_action(action);
                        stream.send(&out).await.map_err(|e| e.to_string())?;
                    }
                }
                _ => {}
            },
            Some(message::Union::FileResponse(fr)) => {
                if let Some(file_response::Union::Dir(dir)) = fr.union {
                    let names: Vec<String> = dir.entries.iter().take(50).map(|e| e.name.clone()).collect();
                    output = format!("{}: {}", dir.path, names.join(", "));
                    break;
                }
            }
            Some(message::Union::TerminalResponse(tr)) => match tr.union {
                Some(terminal_response::Union::Opened(o)) => {
                    if !o.success {
                        return Err(format!("Could not open a terminal: {}", o.message));
                    }
                    // The quotes keep the shell's echo of this line from containing the marker.
                    let line = format!("{command}; echo __OFF\"\"LOAD_END__\r");
                    let mut action = TerminalAction::new();
                    action.set_data(TerminalData {
                        terminal_id: 1,
                        data: line.into_bytes().into(),
                        ..Default::default()
                    });
                    let mut out = Message::new();
                    out.set_terminal_action(action);
                    stream.send(&out).await.map_err(|e| e.to_string())?;
                }
                Some(terminal_response::Union::Data(d)) => {
                    let data = if d.compressed {
                        hbb_common::compress::decompress(&d.data)
                    } else {
                        d.data.to_vec()
                    };
                    output.push_str(&String::from_utf8_lossy(&data));
                    if output.contains(END_MARK) {
                        let mut action = TerminalAction::new();
                        action.set_close(CloseTerminal {
                            terminal_id: 1,
                            ..Default::default()
                        });
                        let mut out = Message::new();
                        out.set_terminal_action(action);
                        stream.send(&out).await.ok();
                        break;
                    }
                }
                Some(terminal_response::Union::Error(e)) => return Err(e.message),
                _ => {}
            },
            Some(message::Union::Misc(m)) => {
                if let Some(misc::Union::CloseReason(r)) = m.union {
                    return Err(r);
                }
            }
            _ => {}
        }
    }
    Ok(json!({
        "ok": true,
        "peer": peer,
        "direct": direct,
        "transport": transport,
        "secured": stream.is_secured(),
        "needed_approval": waited_for_approval,
        "seconds": started.elapsed().as_secs_f32(),
        "output": strip_ansi(&output),
    }))
}

#[cfg(test)]
mod tests {
    #[test]
    fn terminal_colour_codes_are_removed() {
        assert_eq!(
            super::strip_ansi("\u{1b}[1;32mok\u{1b}[0m\r\n\u{1b}]0;title\u{7}x"),
            "ok\nx"
        );
    }
}
