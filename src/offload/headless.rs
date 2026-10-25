//! A windowless [`Interface`] for connections Offload makes on its own (pairing, connection
//! tests): prompts are recorded instead of shown.

use crate::client::{Data, Interface, LoginConfigHandler};
use async_trait::async_trait;
use base::message_proto::*;
use hbb_common::{log, rendezvous_proto::ConnType, Stream};
use std::sync::{Arc, Mutex, RwLock};

#[derive(Clone)]
pub struct HeadlessUi {
    lc: Arc<RwLock<LoginConfigHandler>>,
    last_message: Arc<Mutex<String>>,
}

impl HeadlessUi {
    pub fn new(peer_id: &str, conn_type: ConnType, force_relay: bool) -> Self {
        let mut lc = LoginConfigHandler::default();
        lc.initialize(peer_id.to_owned(), conn_type, None, force_relay, None, None, None);
        Self {
            lc: Arc::new(RwLock::new(lc)),
            last_message: Default::default(),
        }
    }

    /// The text of the last prompt the connection tried to show, if any.
    pub fn last_message(&self) -> String {
        self.last_message.lock().unwrap().clone()
    }
}

#[async_trait]
impl Interface for HeadlessUi {
    fn send(&self, _data: Data) {}
    fn msgbox(&self, msgtype: &str, title: &str, text: &str, _link: &str) {
        log::info!("offload: {msgtype}: {title}: {text}");
        *self.last_message.lock().unwrap() = if text.is_empty() {
            title.to_owned()
        } else {
            text.to_owned()
        };
    }
    fn handle_login_error(&self, _err: &str) -> bool {
        false
    }
    fn handle_peer_info(&self, _pi: PeerInfo) {}
    fn set_multiple_windows_session(&self, _sessions: Vec<WindowsSession>) {}
    async fn handle_hash(&self, _pass: &str, _hash: Hash, _peer: &mut Stream) -> bool {
        false
    }
    async fn handle_login_from_ui(
        &self,
        _os_username: String,
        _os_password: String,
        _password: String,
        _remember: bool,
        _peer: &mut Stream,
    ) {
    }
    async fn handle_test_delay(&self, _t: TestDelay, _peer: &mut Stream) {}
    fn get_lch(&self) -> Arc<RwLock<LoginConfigHandler>> {
        self.lc.clone()
    }
}
