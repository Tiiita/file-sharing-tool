use axum::{extract::WebSocketUpgrade, response::IntoResponse};
use tracing::info;

use crate::websocket;

#[axum::debug_handler]
pub async fn upload() {
    info!("Uploaded from: .");
}

#[axum::debug_handler]
pub async fn download() {
    info!("Downloaded from: .");
}

#[axum::debug_handler]
pub async fn websocket(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(websocket::handle_socket)
}