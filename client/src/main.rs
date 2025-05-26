use std::{
    path::Path, sync::Arc
};
use tokio_tungstenite::connect_async;
use tracing::info;
use transfer::TransferClient;

mod config;
mod transfer;
mod watcher_old;
mod watcher;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let config = config::load();
    info!("Found path: {}", config.path);
    let (ws_stream, _) = connect_async(&config.websocket_url)
        .await
        .expect("Failed to connect");

    info!("Connected to websocket");

    let transfer_client = Arc::new(TransferClient::new(config.clone(), ws_stream));
    transfer_client.listen_websocket();

    watcher_old::watch_dir(Path::new(config.path.as_str()), transfer_client).await;
}

