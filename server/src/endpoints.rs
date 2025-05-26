use axum::{extract::WebSocketUpgrade, response::IntoResponse};
use tracing::info;

use crate::websocket;

#[axum::debug_handler]
pub async fn upload() {
    info!("Received upload request");
}

#[axum::debug_handler]
pub async fn download() {
    info!("Received download request");
}

#[axum::debug_handler]
pub async fn rename() {
    info!("Received rename request");
}

#[axum::debug_handler]
pub async fn delete() {
    info!("Received delete request");

}


#[axum::debug_handler]
pub async fn websocket(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(websocket::handle_socket)
}