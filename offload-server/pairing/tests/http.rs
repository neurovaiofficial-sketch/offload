use axum::{
    body::Body,
    extract::connect_info::MockConnectInfo,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use offload_core::{crypto, slot::CreateSlotResponse, SlotRecord};
use offload_pairing::{router, Directory};
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tower::ServiceExt;

fn app() -> (axum::Router, Vec<u8>) {
    let (pk, sk) = crypto::generate_keypair();
    let dir = Arc::new(Mutex::new(Directory::new(sk)));
    let addr: SocketAddr = "203.0.113.7:5000".parse().unwrap();
    (
        router(dir, crypto::b64_encode(&pk)).layer(MockConnectInfo(addr)),
        pk,
    )
}

async fn body(resp: axum::response::Response) -> Vec<u8> {
    resp.into_body().collect().await.unwrap().to_bytes().to_vec()
}

#[tokio::test]
async fn a_device_registers_and_another_resolves_the_slot_over_http() {
    let (app, pk) = app();
    let resp = app
        .clone()
        .oneshot(
            Request::post("/v1/slots")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"id":"123456789"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);
    let created: CreateSlotResponse = serde_json::from_slice(&body(resp).await).unwrap();

    let resp = app
        .clone()
        .oneshot(
            Request::get(format!("/v1/slots/{}", created.record.slot.to_lowercase()))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let rec: SlotRecord = serde_json::from_slice(&body(resp).await).unwrap();
    assert_eq!(rec.id, "123456789");
    rec.verify(&created.record.slot, &pk, offload_core::now())
        .unwrap();

    let resp = app
        .clone()
        .oneshot(
            Request::delete(format!("/v1/slots/{}", created.record.slot))
                .header("x-offload-slot-token", &created.token)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let resp = app
        .oneshot(
            Request::get(format!("/v1/slots/{}", created.record.slot))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn oversized_and_malformed_requests_are_refused() {
    let (app, _) = app();
    let big = format!(r#"{{"id":"{}"}}"#, "1".repeat(4096));
    let resp = app
        .clone()
        .oneshot(
            Request::post("/v1/slots")
                .header("content-type", "application/json")
                .body(Body::from(big))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::PAYLOAD_TOO_LARGE);

    let resp = app
        .oneshot(
            Request::post("/v1/slots")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"id":"bad id"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}
