use std::path::{Path, PathBuf};

use reqwest::{multipart, Client};
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tracing::{error, info};

use crate::config::Config;

pub type WSStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
pub struct TransferClient {
    config: Config,
    path: PathBuf,
    reqwest: Client,
    ws_stream: WSStream,
}

impl TransferClient {
    pub fn new(config: Config, ws_stream: WSStream) -> Self {
        Self {
            path: PathBuf::from(config.path.as_str()),
            config,
            ws_stream,
            reqwest: Client::new(),
        }
        
    }

    pub async fn upload(&self, abs_path: &Path) {
        let relative_path = self.rel_path(abs_path);
        match tokio::fs::read(abs_path).await {
            Ok(bytes) => {
                let file_name = match relative_path.file_name().and_then(|n| n.to_str()) {
                    Some(name) => name,
                    None => {
                        error!("Failed to get valid UTF-8 file name from path");
                        return;
                    }
                };
    
                let form = multipart::Form::new()
                    .text("file_path", relative_path.to_string_lossy().into_owned())
                    .part("file", multipart::Part::bytes(bytes).file_name(file_name.to_string()));
                
                match self.reqwest
                    .post(&self.config.upload_url)
                    .multipart(form)
                    .bearer_auth(&self.config.token)
                    .send()
                    .await
                {
                    Ok(response) => {
                        if !response.status().is_success() {
                            error!("Upload failed with status: {}", response.status());
                        }
                    }
                    Err(why) => {
                        error!("Upload request error: {}", why);
                    }
                }
            }
            Err(why) => {
                error!("Failed to read file {}: {}", relative_path.display(), why);
            }
        }
    }

    pub async fn rename(&self, original_abs_path: &Path, new_abs_path: &Path) {
        info!("Trying rename");
    }

    pub async fn delete(&self, abs_file_path: &Path) {
        info!("Trying delete");
    }

    async fn download(&self, abs_file_path: &Path) {
        
    }

    fn rel_path<'a>(&self, abs_path: &'a Path) -> &'a Path {
        abs_path.strip_prefix(self.path.clone()).unwrap()
    }

    // Starts listening for incoming file changes communcated via websocket in new thread. 
    // Triggers download() to use http to sync file again.
    pub fn listen_websocket(&self) {
        tokio::spawn(async move {
            
        });
    }
}

