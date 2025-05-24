use figment::{providers::{Format, Json}, Figment};
use serde::{Deserialize, Serialize};
use tracing::info;

pub fn load() -> Config {
    let config = Figment::new().merge(Json::file("config.json"));

    let config: Config = config.extract().expect("Failed to load config..");

    info!("Loaded config");
    config
}


#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub path: String,
    pub websocket_url: String,
    pub download_url: String,
    pub upload_url: String,
    pub token: String,
}