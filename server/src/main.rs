use std::env;

use axum::{routing::{get, post}, Router};
use dotenv::dotenv;
use tokio::net::TcpListener;
use tracing::info;

mod endpoints;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv().expect("Failed to init enviroment by .env file");
    let addr = env::var("ADDRESS").expect("Unable to find listening address in enviroment");
    let listener = TcpListener::bind(&addr).await.expect("Unable to bind to addr: {addr}");

    info!("Start listening on {addr}");
    axum::serve(listener, app()).await.expect("Failed to start server")
}

fn app() -> Router {
    Router::new()
    .route("/upload", post(endpoints::upload))
    .route("/download", get(endpoints::download))
}