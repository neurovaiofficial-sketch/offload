//! Offload pairing directory.
//!
//! A device that wants to be linked asks for a slot (`POST /v1/slots`) and shows the slot
//! followed by a secret it generates itself. The device the user types the code into looks
//! the slot up (`GET /v1/slots/{slot}`), learns which device id to connect to, and presents the
//! whole code to that device over the end-to-end encrypted RustDesk session. The directory
//! never sees the secret, cannot approve anything, and signs its answers with the rendezvous
//! server's key so they cannot be forged in transit.

use axum::{
    extract::{ConnectInfo, DefaultBodyLimit, Path, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use offload_core::{
    code,
    crypto,
    ratelimit::RateLimiter,
    slot::{CreateSlotRequest, CreateSlotResponse, SlotRecord},
};
use serde::Serialize;
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
    time::Duration,
};

pub const SLOT_TTL_SECS: i64 = offload_core::pairing::CODE_TTL_SECS;
pub const MAX_ACTIVE_SLOTS: usize = 50_000;
pub const CREATES_PER_MINUTE: usize = 10;
pub const LOOKUPS_PER_MINUTE: usize = 30;
const TOKEN_HEADER: &str = "x-offload-slot-token";

struct Slot {
    record: SlotRecord,
    token: String,
}

pub struct Directory {
    sk: Vec<u8>,
    slots: HashMap<String, Slot>,
    by_device: HashMap<String, String>,
    creates: RateLimiter,
    lookups: RateLimiter,
}

impl Directory {
    pub fn new(server_sk: Vec<u8>) -> Self {
        Self {
            sk: server_sk,
            slots: HashMap::new(),
            by_device: HashMap::new(),
            creates: RateLimiter::new(60, CREATES_PER_MINUTE, 100_000),
            lookups: RateLimiter::new(60, LOOKUPS_PER_MINUTE, 100_000),
        }
    }

    pub fn active_slots(&self) -> usize {
        self.slots.len()
    }

    fn remove(&mut self, slot: &str) {
        if let Some(s) = self.slots.remove(slot) {
            if self.by_device.get(&s.record.id).map(String::as_str) == Some(slot) {
                self.by_device.remove(&s.record.id);
            }
        }
    }

    pub fn sweep(&mut self, now: i64) {
        let dead: Vec<String> = self
            .slots
            .iter()
            .filter(|(_, s)| s.record.expires_at <= now)
            .map(|(k, _)| k.clone())
            .collect();
        for k in dead {
            self.remove(&k);
        }
        self.creates.prune(now);
        self.lookups.prune(now);
    }

    pub fn create(&mut self, client: &str, id: &str, now: i64) -> Result<CreateSlotResponse, ApiError> {
        if !offload_core::is_valid_device_id(id) {
            return Err(ApiError::BadRequest("invalid device id"));
        }
        if !self.creates.allow(client, now) {
            return Err(ApiError::TooManyRequests);
        }
        self.sweep(now);
        // One code per device at a time: asking again replaces the previous code.
        if let Some(old) = self.by_device.get(id).cloned() {
            self.remove(&old);
        }
        if self.slots.len() >= MAX_ACTIVE_SLOTS {
            return Err(ApiError::Busy);
        }
        let mut slot = None;
        for _ in 0..32 {
            let s = code::random_symbols(code::SLOT_LEN);
            if !self.slots.contains_key(&s) {
                slot = Some(s);
                break;
            }
        }
        let slot = slot.ok_or(ApiError::Busy)?;
        let record = SlotRecord::signed(&slot, id, now + SLOT_TTL_SECS, &self.sk)
            .map_err(|_| ApiError::Internal)?;
        let token = hex(&crypto::random_bytes(16));
        self.slots.insert(
            slot.clone(),
            Slot {
                record: record.clone(),
                token: token.clone(),
            },
        );
        self.by_device.insert(id.to_owned(), slot);
        Ok(CreateSlotResponse { record, token })
    }

    pub fn lookup(&mut self, client: &str, slot: &str, now: i64) -> Result<SlotRecord, ApiError> {
        if !self.lookups.allow(client, now) {
            return Err(ApiError::TooManyRequests);
        }
        let slot = slot.to_ascii_uppercase();
        if !code::is_valid_slot(&slot) {
            return Err(ApiError::NotFound);
        }
        match self.slots.get(&slot) {
            Some(s) if s.record.expires_at > now => Ok(s.record.clone()),
            _ => Err(ApiError::NotFound),
        }
    }

    pub fn delete(&mut self, slot: &str, token: &str) -> Result<(), ApiError> {
        let ok = self
            .slots
            .get(slot)
            .map(|s| crypto::secure_eq(s.token.as_bytes(), token.as_bytes()))
            .unwrap_or(false);
        if !ok {
            return Err(ApiError::NotFound);
        }
        self.remove(slot);
        Ok(())
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[derive(Debug, PartialEq, Eq)]
pub enum ApiError {
    BadRequest(&'static str),
    NotFound,
    TooManyRequests,
    Busy,
    Internal,
}

#[derive(Serialize)]
struct ErrorBody {
    error: &'static str,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error) = match self {
            ApiError::BadRequest(m) => (StatusCode::BAD_REQUEST, m),
            ApiError::NotFound => (StatusCode::NOT_FOUND, "not found"),
            ApiError::TooManyRequests => (StatusCode::TOO_MANY_REQUESTS, "too many requests"),
            ApiError::Busy => (StatusCode::SERVICE_UNAVAILABLE, "busy, try again shortly"),
            ApiError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "internal error"),
        };
        (status, Json(ErrorBody { error })).into_response()
    }
}

pub type Shared = Arc<Mutex<Directory>>;

#[derive(Clone)]
pub struct AppState {
    pub dir: Shared,
    pub public_key: String,
}

fn client_key(addr: &SocketAddr) -> String {
    // IPv6 clients are limited per /64, the smallest prefix an end site is normally given.
    match addr.ip() {
        std::net::IpAddr::V4(v4) => v4.to_string(),
        std::net::IpAddr::V6(v6) => {
            if let Some(v4) = v6.to_ipv4_mapped() {
                return v4.to_string();
            }
            let s = v6.segments();
            format!("{:x}:{:x}:{:x}:{:x}::/64", s[0], s[1], s[2], s[3])
        }
    }
}

async fn create_slot(
    State(st): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(req): Json<CreateSlotRequest>,
) -> Result<(StatusCode, Json<CreateSlotResponse>), ApiError> {
    let res = st
        .dir
        .lock()
        .map_err(|_| ApiError::Internal)?
        .create(&client_key(&addr), &req.id, offload_core::now())?;
    Ok((StatusCode::CREATED, Json(res)))
}

async fn lookup_slot(
    State(st): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Path(slot): Path<String>,
) -> Result<Json<SlotRecord>, ApiError> {
    let rec = st
        .dir
        .lock()
        .map_err(|_| ApiError::Internal)?
        .lookup(&client_key(&addr), &slot, offload_core::now())?;
    Ok(Json(rec))
}

async fn delete_slot(
    State(st): State<AppState>,
    Path(slot): Path<String>,
    headers: HeaderMap,
) -> Result<StatusCode, ApiError> {
    let token = headers
        .get(TOKEN_HEADER)
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default();
    st.dir
        .lock()
        .map_err(|_| ApiError::Internal)?
        .delete(&slot.to_ascii_uppercase(), token)?;
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Serialize)]
struct Info {
    service: &'static str,
    version: &'static str,
    /// For operators checking a deployment; clients verify against the key built into them.
    public_key: String,
}

async fn info(State(st): State<AppState>) -> Json<Info> {
    Json(Info {
        service: "offload-pairing",
        version: env!("CARGO_PKG_VERSION"),
        public_key: st.public_key.clone(),
    })
}

pub fn router(dir: Shared, public_key: String) -> Router {
    Router::new()
        .route("/healthz", get(|| async { "ok" }))
        .route("/v1/info", get(info))
        .route("/v1/slots", post(create_slot))
        .route("/v1/slots/:slot", get(lookup_slot).delete(delete_slot))
        .layer(DefaultBodyLimit::max(1024))
        .layer(tower_http::timeout::TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(10),
        ))
        .with_state(AppState { dir, public_key })
}

/// Reads a key in the rendezvous server's `id_ed25519` format: base64 of the 64-byte
/// libsodium secret key.
pub fn parse_server_key(contents: &str) -> Option<Vec<u8>> {
    crypto::b64_decode(contents.trim()).filter(|k| k.len() == crypto::SECRET_KEY_LEN)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dir() -> (Directory, Vec<u8>) {
        let (pk, sk) = crypto::generate_keypair();
        (Directory::new(sk), pk)
    }

    #[test]
    fn created_slots_resolve_with_a_verifiable_signature() {
        let (mut d, pk) = dir();
        let created = d.create("1.2.3.4", "123456789", 1000).unwrap();
        let rec = d.lookup("5.6.7.8", &created.record.slot, 1001).unwrap();
        assert_eq!(rec.id, "123456789");
        assert_eq!(rec.verify(&created.record.slot, &pk, 1001), Ok(()));
    }

    #[test]
    fn one_code_per_device() {
        let (mut d, _) = dir();
        d.create("1.2.3.4", "123", 1000).unwrap();
        let second = d.create("1.2.3.4", "123", 1001).unwrap();
        assert_eq!(d.active_slots(), 1);
        assert_eq!(d.lookup("x", &second.record.slot, 1002).unwrap().id, "123");
    }

    #[test]
    fn slots_expire() {
        let (mut d, _) = dir();
        let c = d.create("1.2.3.4", "123", 1000).unwrap();
        assert_eq!(
            d.lookup("x", &c.record.slot, 1000 + SLOT_TTL_SECS),
            Err(ApiError::NotFound)
        );
        d.sweep(1000 + SLOT_TTL_SECS);
        assert_eq!(d.active_slots(), 0);
    }

    #[test]
    fn deleting_needs_the_token() {
        let (mut d, _) = dir();
        let c = d.create("1.2.3.4", "123", 1000).unwrap();
        assert_eq!(d.delete(&c.record.slot, "wrong"), Err(ApiError::NotFound));
        assert_eq!(d.delete(&c.record.slot, &c.token), Ok(()));
        assert_eq!(d.lookup("x", &c.record.slot, 1001), Err(ApiError::NotFound));
    }

    #[test]
    fn creation_and_lookup_are_rate_limited_per_client() {
        let (mut d, _) = dir();
        for i in 0..CREATES_PER_MINUTE {
            d.create("1.2.3.4", &format!("dev{i}"), 1000).unwrap();
        }
        assert_eq!(d.create("1.2.3.4", "devX", 1000).err(), Some(ApiError::TooManyRequests));
        assert!(d.create("9.9.9.9", "devX", 1000).is_ok());
        for _ in 0..LOOKUPS_PER_MINUTE {
            let _ = d.lookup("5.5.5.5", "AAAA", 1000);
        }
        assert_eq!(d.lookup("5.5.5.5", "AAAA", 1000), Err(ApiError::TooManyRequests));
    }

    #[test]
    fn invalid_input_is_refused() {
        let (mut d, _) = dir();
        assert!(matches!(d.create("c", "not an id", 1000), Err(ApiError::BadRequest(_))));
        assert_eq!(d.lookup("c", "../../etc", 1000), Err(ApiError::NotFound));
    }

    #[test]
    fn ipv6_clients_share_a_limit_per_64() {
        let a: SocketAddr = "[2001:db8:1:2:aaaa::1]:5".parse().unwrap();
        let b: SocketAddr = "[2001:db8:1:2:bbbb::9]:6".parse().unwrap();
        assert_eq!(client_key(&a), client_key(&b));
        let m: SocketAddr = "[::ffff:1.2.3.4]:5".parse().unwrap();
        assert_eq!(client_key(&m), "1.2.3.4");
    }

    #[test]
    fn server_keys_in_hbbs_format_parse() {
        let (_, sk) = crypto::generate_keypair();
        assert_eq!(parse_server_key(&format!("{}\n", crypto::b64_encode(&sk))), Some(sk));
        assert_eq!(parse_server_key("short"), None);
    }
}
