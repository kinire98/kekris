use std::sync::Arc;

use tauri::AppHandle;
use tokio::sync::{
    Mutex, OnceCell,
    mpsc::{self, Sender},
};

use crate::{
    models::{
        dummy_room::{DummyPlayer, DummyRoom},
        room_info::RoomInfo,
    },
    room::{Room, listen_to_rooms::listen_to_rooms},
};

const PLAYERS_EMIT: &str = "playersEmit";
const ROOM_NAME_EMIT: &str = "roomNameEmit";

static END_SEARCH_CHANNEL: OnceCell<Arc<Mutex<Sender<bool>>>> = OnceCell::const_new();
static END_LISTEN_ROOM: OnceCell<Arc<Mutex<Sender<bool>>>> = OnceCell::const_new();
static END_ROOM: OnceCell<Arc<Mutex<Sender<bool>>>> = OnceCell::const_new();

#[tauri::command]
pub async fn listen_for_rooms(app: AppHandle) {
    let (tx, rx) = mpsc::channel(32);
    tokio::spawn(async move {
        listen_to_rooms(app, rx).await;
    });
    if let Some(tx_rec) = END_SEARCH_CHANNEL.get() {
        let mut locked = tx_rec.lock().await;
        *locked = tx;
    } else {
        END_SEARCH_CHANNEL.set(Arc::new(Mutex::new(tx))).unwrap();
    }
}

#[tauri::command]
pub async fn stop_search() {
    if let Some(tx) = END_SEARCH_CHANNEL.get() {
        tx.lock().await.send(false).await.unwrap();
    }
}
#[tauri::command]
pub async fn join_room(app: AppHandle, room: RoomInfo, player: DummyPlayer) {
    stop_search().await;
    crate::room::join_room::join_room(room, player, app).await;
}

#[tauri::command]
pub async fn create_room(app: AppHandle, name: String) -> DummyRoom {
    let (tx, rx) = mpsc::channel(32);
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
    let mut room = Room::new(name, app, rx_end, rx);
    let dummy_room = room.as_ref().into();
    tokio::spawn(async move {
        room.room_start().await;
    });
    dummy_room
}

#[tauri::command]
pub fn leave_room() {}

#[tauri::command]
pub async fn close_room() {
    if let Some(channel) = END_LISTEN_ROOM.get() {
        let _ = channel.lock().await.send(false).await;
    }
    if let Some(channel) = END_ROOM.get() {
        let _ = channel.lock().await.send(false).await;
    }
}

#[tauri::command]
pub fn get_room_name() -> String {
    "Room".to_string()
}

#[tauri::command]
pub async fn room_info(app: AppHandle) -> DummyRoom {
    let (_tx, rx) = mpsc::channel(32);

    let (_tx_end, rx_end) = mpsc::channel(32);

    let room = Room::new("test".to_string(), app, rx_end, rx);
    room.as_ref().into()
}
