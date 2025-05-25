use axum::{extract::WebSocketUpgrade, response::IntoResponse};
use tracing::info;

use crate::websocket;

#[axum::debug_handler]
pub async fn upload() {

}

#[axum::debug_handler]
pub async fn download() {

}

#[axum::debug_handler]
pub async fn rename() {

}

#[axum::debug_handler]
pub async fn delete() {

}


#[axum::debug_handler]
pub async fn websocket(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(websocket::handle_socket)
}