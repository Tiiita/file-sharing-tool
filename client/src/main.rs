use notify::{event::{self, ModifyKind}, Event, EventKind, Result, Watcher};
use tokio_tungstenite::connect_async;
use std::{any::Any, collections::HashMap, f32::{consts::E, INFINITY}, path::Path, sync::mpsc, time::Instant};
use tracing::{error, info, warn};
use transfer::TransferClient;

mod config;
mod transfer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let config = config::load();
    info!("Found path: {}", config.path);
    let (ws_stream, _) = connect_async(&config.websocket_url).await.expect("Failed to connect");
    info!("Connected to websocket");

    let transfer_client = TransferClient::new(config.clone(), ws_stream);
    transfer_client.listen_websocket();
    watch_dir(Path::new(config.path.as_str()), transfer_client).await;
}

async fn watch_dir(path: &Path, client: TransferClient) {
    let mut last_event = Vec::new();

    let (tx, rx) = mpsc::channel::<Result<Event>>();
    match notify::recommended_watcher(tx) {
        Ok(mut watcher) => {
            match watcher.watch(path, notify::RecursiveMode::Recursive) {
                Ok(_) => {
                    info!("Started watching directory")
                }
                Err(why) => {
                    error!("Failed to start watching dir: {why}")
                }
            }

            for res in rx {
                match res {
                    Ok(event) => {
                        handle_event(event, &client, &mut last_event).await;
                    }
                    Err(why) => error!("Error while watching: {}", why),
                }
            }
        }
        Err(why) => {
            error!("Failed to init watcher: {why}")
        }
    }
}

async fn handle_event(event: Event, client: &TransferClient, last_event: &mut Vec<Event>) {
    info!("{:?}", event.kind);
}


fn handle_group() {

}
