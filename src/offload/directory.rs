//! Client for the pairing directory (`offload-server/pairing`).

use hbb_common::{bail, config::Config, ResultType};
use offload_core::{
    slot::{CreateSlotRequest, CreateSlotResponse},
    SlotRecord,
};
use std::time::Duration;

pub const DEFAULT_PORT: u16 = 21120;
const TIMEOUT: Duration = Duration::from_secs(10);

/// The directory runs next to the rendezvous server unless a build points elsewhere.
pub fn base_url() -> String {
    if let Some(url) = option_env!("OFFLOAD_PAIRING_URL") {
        if !url.is_empty() {
            return url.trim_end_matches('/').to_owned();
        }
    }
    format!("http://{}:{DEFAULT_PORT}", host_of(&Config::get_rendezvous_server()))
}

fn host_of(server: &str) -> String {
    let server = server.trim();
    if let Some(rest) = server.strip_prefix('[') {
        if let Some((host, _)) = rest.split_once(']') {
            return format!("[{host}]");
        }
    }
    match server.rsplit_once(':') {
        Some((host, port)) if !host.contains(':') && port.chars().all(|c| c.is_ascii_digit()) => {
            host.to_owned()
        }
        _ => server.to_owned(),
    }
}

async fn client(url: &str) -> reqwest::Client {
    crate::hbbs_http::create_http_client_async_with_url(url).await
}

fn error_text(status: reqwest::StatusCode, body: &str) -> String {
    serde_json::from_str::<serde_json::Value>(body)
        .ok()
        .and_then(|v| v.get("error").and_then(|e| e.as_str()).map(str::to_owned))
        .unwrap_or_else(|| status.to_string())
}

pub async fn create_slot(id: &str) -> ResultType<CreateSlotResponse> {
    let url = format!("{}/v1/slots", base_url());
    let resp = client(&url)
        .await
        .post(&url)
        .timeout(TIMEOUT)
        .json(&CreateSlotRequest { id: id.to_owned() })
        .send()
        .await?;
    let status = resp.status();
    let body = resp.text().await?;
    if !status.is_success() {
        bail!("pairing service: {}", error_text(status, &body));
    }
    Ok(serde_json::from_str(&body)?)
}

/// `Ok(None)` when no device is currently showing a code with this slot.
pub async fn lookup_slot(slot: &str) -> ResultType<Option<SlotRecord>> {
    let url = format!("{}/v1/slots/{slot}", base_url());
    let resp = client(&url).await.get(&url).timeout(TIMEOUT).send().await?;
    let status = resp.status();
    if status == reqwest::StatusCode::NOT_FOUND {
        return Ok(None);
    }
    let body = resp.text().await?;
    if !status.is_success() {
        bail!("pairing service: {}", error_text(status, &body));
    }
    Ok(Some(serde_json::from_str(&body)?))
}

pub async fn delete_slot(slot: &str, token: &str) {
    let url = format!("{}/v1/slots/{slot}", base_url());
    let res = client(&url)
        .await
        .delete(&url)
        .timeout(TIMEOUT)
        .header("x-offload-slot-token", token)
        .send()
        .await;
    if let Err(e) = res {
        hbb_common::log::debug!("offload: could not release pairing slot: {e}");
    }
}

#[cfg(test)]
mod tests {
    use super::host_of;

    #[test]
    fn the_directory_host_follows_the_rendezvous_server() {
        assert_eq!(host_of("rs.example.com:21116"), "rs.example.com");
        assert_eq!(host_of("rs.example.com"), "rs.example.com");
        assert_eq!(host_of("203.0.113.5:21116"), "203.0.113.5");
        assert_eq!(host_of("[2001:db8::1]:21116"), "[2001:db8::1]");
    }
}
