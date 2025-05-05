use std::sync::Arc;

use tauri::AppHandle;
use tokio::sync::{
    Mutex, broadcast,
    mpsc::{self},
};

use super::{END_LISTEN_ROOM, END_ROOM};

use crate::{models::dummy_room::DummyRoom, room::Room};

// const ROOM_NAME_EMIT: &str = "roomNameEmit";

#[tauri::command]
pub async fn create_room(app: AppHandle, name: String, player_name: String) -> DummyRoom {
    let (tx, rx) = broadcast::channel(32);
    if let Some(channel) = END_LISTEN_ROOM.get() {
        let mut lock = channel.lock().await;
        *lock = tx;
    } else {
        END_LISTEN_ROOM.set(Arc::new(Mutex::new(tx))).unwrap();
    }
    let (tx_end, rx_end) = mpsc::channel(32);
    if let Some(channel) = END_ROOM.get() {
        let mut lock = channel.lock().await;
        *lock = tx_end;
    } else {
        END_ROOM.set(Arc::new(Mutex::new(tx_end))).unwrap();
    }
    let mut room = Room::new(name, app, rx_end, rx, player_name).await;
    let dummy_room = (&room).into();
    tokio::spawn(async move {
        room.room_start().await;
    });
    dummy_room
}

#[tauri::command]
pub async fn close_room() {
    if let Some(channel) = END_LISTEN_ROOM.get() {
        let _ = channel.lock().await.send(false);
    }
    if let Some(channel) = END_ROOM.get() {
        let _ = channel.lock().await.send(false).await;
    }
}

#[tauri::command]
pub async fn start_online_game() {}
