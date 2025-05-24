use tracing::info;

#[axum::debug_handler]
pub async fn upload() {
    info!("Uploaded from: .");
}

#[axum::debug_handler]
pub async fn download() {
    info!("Downloaded from: .");
}