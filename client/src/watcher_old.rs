use notify::{
    event::ModifyKind, Event, EventKind, RecursiveMode, Result, Watcher
};
use std::{
    collections::VecDeque, env::{consts::OS}, path::Path, process::exit, sync::Arc, time::Duration
};
use tokio::{sync::Mutex, time::sleep};
use tracing::{error, info};
use transfer::TransferClient;

use crate::transfer;

pub async fn watch_dir(path: &Path, client: Arc<TransferClient>) {
    let (tx_sync, rx_sync) = std::sync::mpsc::channel::<Result<Event>>();

    let mut watcher = match notify::recommended_watcher(tx_sync) {
        Ok(w) => w,
        Err(e) => {
            error!("Failed to create watcher: {e}");
            return;
        }
    };

    if let Err(e) = watcher.watch(path, RecursiveMode::Recursive) {
        error!("Failed to watch path: {e}");
        return;
    }

    info!("Started watching directory");

    let (tx, mut rx) = tokio::sync::mpsc::channel::<Event>(100);
    let buffer = Arc::new(Mutex::new(VecDeque::new()));
    let client_ref = Arc::clone(&client);

    tokio::spawn({
        let buffer = Arc::clone(&buffer);
        async move {
            let mut debounce_timer: Option<tokio::task::JoinHandle<()>> = None;

            while let Some(event) = rx.recv().await {
                buffer.lock().await.push_back(event);

                if let Some(timer) = debounce_timer.take() {
                    timer.abort();
                }

                let buffer = Arc::clone(&buffer);
                let client = Arc::clone(&client_ref);
                debounce_timer = Some(tokio::spawn(async move {
                    sleep(Duration::from_millis(100)).await;

                    let mut buf = buffer.lock().await;
                    let grouped = std::mem::take(&mut *buf);
                    drop(buf);

                    handle_event_group(grouped, client).await;
                }));
            }
        }
    });

    for res in rx_sync {
        match res {
            Ok(event) => {
                if let Err(e) = tx.try_send(event) {
                    error!("Failed to forward event: {e}");
                }
            }
            Err(e) => {
                error!("Error while watching: {e}");
            }
        }
    }
}

async fn handle_event_group(buffer: VecDeque<Event>, client: Arc<TransferClient>) {
    match OS.to_ascii_lowercase().as_str() {
        "windows" => { handle_event_group_win(buffer, client).await },
        "macos" => { handle_event_group_macos(buffer, client).await },
        _ => {
            error!("Unsupported operating system: '{OS}', exiting program");
            exit(1);
        },
    }
}

async fn handle_event_group_macos(buffer: VecDeque<Event>, client: Arc<TransferClient>) {
    let kinds = buffer.iter().map(|e| e.kind).collect::<VecDeque<EventKind>>();
    info!("{:?}", kinds);

    //Rename
    if kinds.iter().all(|kind| matches!(kind, EventKind::Modify(ModifyKind::Name(notify::event::RenameMode::Any))))
    && kinds.len() == 2 {
        let original_abs_path = &buffer.get(0).unwrap().paths.first().unwrap();
        let new_abs_path = &buffer.get(1).unwrap().paths.first().unwrap();
       client.rename(original_abs_path, new_abs_path).await;
       return;
    }


    //File moved in or out
    if kinds.iter().all(|kind| matches!(kind, EventKind::Modify(ModifyKind::Name(notify::event::RenameMode::Any))))
    && kinds.len() == 1 {
        todo!("Check file presence and delete() or upload() based on that");
    }

    //Delete
    if kinds.iter().any(|kind| matches!(kind, EventKind::Create(_)))
    && kinds.len() == 2 {
        client.delete(&buffer.iter().last().unwrap().paths.first().unwrap()).await;
        return;
    }

    //Create
    if kinds.iter().any(|kind| matches!(kind, EventKind::Create(_)))
    && kinds.len() == 1 {
        client.upload(&buffer.iter().last().unwrap().paths.first().unwrap()).await;
        return;
    }
}

async fn handle_event_group_win(buffer: VecDeque<Event>, client: Arc<TransferClient>) {

}