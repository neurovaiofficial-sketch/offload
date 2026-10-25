//! Offload's login and pairing paths on the controlled side of a connection.

use super::*;
use crate::offload::server::{self as offload, LoginCheck};

impl Connection {
    /// A pairing request never turns into a session: it is answered here and the connection
    /// closes, whatever the owner decides.
    pub(super) async fn offload_pair_hook(&mut self, lr: &LoginRequest) -> Option<bool> {
        let Some(login_request::Union::OffloadPair(req)) = lr.union.as_ref() else {
            return None;
        };
        if !self.stream.is_secured() {
            self.send_login_error(crate::offload::MSG_INSECURE).await;
            return Some(false);
        }
        let requester = match offload::begin_pair(lr, req, &self.hash.salt, &self.hash.challenge)
        {
            Ok(r) => r,
            Err(msg) => {
                self.send_login_error(msg).await;
                sleep(1.).await;
                return Some(false);
            }
        };
        match offload::await_decision(&requester.id).await {
            Some(approved) => match offload::finish_pair(&approved) {
                Ok(()) => {
                    let mut res = LoginResponse::new();
                    res.set_offload_pair(OffloadPairResult {
                        name: crate::offload::my_name(),
                        platform: crate::offload::my_platform(),
                        ..Default::default()
                    });
                    let mut msg_out = Message::new();
                    msg_out.set_login_response(res);
                    self.send(msg_out).await;
                }
                Err(e) => self.send_login_error(e).await,
            },
            None => self.send_login_error(offload::MSG_DENIED).await,
        }
        // Let the answer leave before the connection is torn down.
        sleep(1.).await;
        Some(false)
    }

    /// Admits only devices paired with this one, by signature over this connection's challenge.
    /// Unless the owner allowed unattended access, the owner still accepts each session.
    pub(super) async fn offload_login_hook(&mut self, lr: &LoginRequest) -> Option<bool> {
        if !crate::offload::ENFORCED {
            return None;
        }
        let check = offload::verify_login(
            lr,
            &self.hash.salt,
            &self.hash.challenge,
            self.stream.is_secured(),
        );
        match check {
            LoginCheck::Trusted { name, unattended } => {
                if unattended {
                    if !self.send_logon_response_and_keep_alive().await {
                        return Some(false);
                    }
                    self.try_start_cm(lr.my_id.clone(), name, self.authorized);
                } else {
                    self.try_start_cm(lr.my_id.clone(), name, false);
                    self.send_login_error(crate::client::LOGIN_MSG_NO_PASSWORD_ACCESS)
                        .await;
                }
                Some(true)
            }
            LoginCheck::Rejected(msg) => {
                log::info!("offload: refused a login from {}: {}", lr.my_id, msg);
                self.send_login_error(msg).await;
                sleep(1.).await;
                Some(false)
            }
        }
    }
}

/// Ends the sessions of `peer_id`, or every session. Returns how many were told to close.
pub fn offload_disconnect(peer_id: Option<&str>) -> usize {
    let conns = AUTHED_CONNS.lock().unwrap();
    conns
        .iter()
        .filter(|c| peer_id.map_or(true, |id| c.session_key.peer_id == id))
        .filter(|c| c.sender.send(ipc::Data::Close).is_ok())
        .count()
}

pub fn offload_has_sessions() -> bool {
    !AUTHED_CONNS.lock().unwrap().is_empty()
}
