pub(crate) mod handlers;

use crate::handlers::{handle_client, Store};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let store: Store = Arc::new(Mutex::new(HashMap::new()));
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    loop {
        let (sock, _addr) = listener.accept().await.unwrap();
        let store = store.clone();
        tokio::spawn(async move {
            handle_client(sock, store).await;
        });
    }
}
