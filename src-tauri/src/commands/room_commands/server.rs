use std::sync::Arc;

use tauri::AppHandle;
use tokio::sync::{
    Mutex, broadcast,
    mpsc::{self},
};

use super::{END_LISTEN_ROOM, END_ROOM, SEND_ROOM_UPDATES};

use crate::{
    globals::SIZE_FOR_KB,
    models::dummy_room::DummyRoom,
    room::{FirstLevelCommands, Room},
};

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
    let (tx_command, rx_command) = mpsc::channel(SIZE_FOR_KB);
    if let Some(channel) = SEND_ROOM_UPDATES.get() {
        let mut lock = channel.lock().await;
        *lock = tx_command.clone();
    } else {
        SEND_ROOM_UPDATES
            .set(Arc::new(Mutex::new(tx_command.clone())))
            .unwrap();
    }
    let mut room = Room::new(name, app, rx_end, rx, player_name, tx_command, rx_command).await;
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
pub async fn start_online_game() {
    if let Some(channel) = SEND_ROOM_UPDATES.get() {
        let _ = channel
            .lock()
            .await
            .send(FirstLevelCommands::GameStarts)
            .await;
    }
}
