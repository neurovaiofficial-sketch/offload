use offload_pairing::{parse_server_key, router, Directory};
use std::{
    net::SocketAddr,
    path::PathBuf,
    sync::{Arc, Mutex},
    time::Duration,
};

/// The rendezvous server (hbbs) creates its key on first start; give it time to appear.
const KEY_WAIT: Duration = Duration::from_secs(60);

fn env_or(name: &str, default: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| default.to_owned())
}

async fn load_key(path: &PathBuf) -> Result<Vec<u8>, String> {
    let deadline = tokio::time::Instant::now() + KEY_WAIT;
    loop {
        match std::fs::read_to_string(path) {
            Ok(s) => {
                return parse_server_key(&s)
                    .ok_or_else(|| format!("{} is not an hbbs id_ed25519 key", path.display()))
            }
            Err(e) if tokio::time::Instant::now() >= deadline => {
                return Err(format!("cannot read {}: {e}", path.display()))
            }
            Err(_) => tokio::time::sleep(Duration::from_secs(1)).await,
        }
    }
}

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let key_file = PathBuf::from(env_or("OFFLOAD_KEY_FILE", "/data/id_ed25519"));
    let listen: SocketAddr = match env_or("OFFLOAD_PAIRING_LISTEN", "0.0.0.0:21120").parse() {
        Ok(a) => a,
        Err(e) => {
            log::error!("OFFLOAD_PAIRING_LISTEN is not an address: {e}");
            std::process::exit(2);
        }
    };
    let sk = match load_key(&key_file).await {
        Ok(k) => k,
        Err(e) => {
            log::error!("{e}");
            std::process::exit(1);
        }
    };
    let public_key = offload_core::crypto::public_key_of(&sk)
        .map(|pk| offload_core::crypto::b64_encode(&pk))
        .unwrap_or_default();
    log::info!("signing with the rendezvous key {public_key}");

    let dir = Arc::new(Mutex::new(Directory::new(sk)));
    let sweeper = dir.clone();
    tokio::spawn(async move {
        let mut tick = tokio::time::interval(Duration::from_secs(30));
        loop {
            tick.tick().await;
            if let Ok(mut d) = sweeper.lock() {
                d.sweep(offload_core::now());
            }
        }
    });

    let app = router(dir, public_key).into_make_service_with_connect_info::<SocketAddr>();
    let listener = match tokio::net::TcpListener::bind(listen).await {
        Ok(l) => l,
        Err(e) => {
            log::error!("cannot listen on {listen}: {e}");
            std::process::exit(1);
        }
    };
    log::info!("offload-pairing listening on {listen}");
    let shutdown = async {
        let _ = tokio::signal::ctrl_c().await;
    };
    if let Err(e) = axum::serve(listener, app)
        .with_graceful_shutdown(shutdown)
        .await
    {
        log::error!("server error: {e}");
        std::process::exit(1);
    }
}
