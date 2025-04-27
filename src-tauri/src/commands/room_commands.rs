use std::sync::{Arc, mpsc::channel};

use tauri::AppHandle;
use tokio::sync::{
    Mutex, OnceCell,
    mpsc::{self, Sender},
};

use crate::{
    models::{
        dummy_room::{self, DummyPlayer, DummyRoom},
        room_info::RoomInfo,
    },
    room::{Room, listen_to_rooms::listen_to_rooms, player::Player},
};

const PLAYERS_EMIT: &str = "playersEmit";
const ROOM_NAME_EMIT: &str = "roomNameEmit";

static ROOM: OnceCell<Arc<Mutex<Room>>> = OnceCell::const_new();
static END_SEARCH_CHANNEL: OnceCell<Arc<Mutex<Sender<bool>>>> = OnceCell::const_new();

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
pub async fn join_room(room: RoomInfo, player: DummyPlayer) {
    stop_search().await
}

#[tauri::command]
pub fn create_room(app: AppHandle, name: String) -> DummyRoom {
    let room = Room::new(name, app);
    let dummy_room = room.as_ref().into();
    ROOM.set(Arc::new(Mutex::new(room)))
        .expect("Reasonable this will not panic");
    dummy_room
}

#[tauri::command]
pub fn leave_room() {}

#[tauri::command]
pub fn close_room() {}

#[tauri::command]
pub fn get_room_name() -> String {
    "Room".to_string()
}

#[tauri::command]
pub fn room_info(app: AppHandle) -> DummyRoom {
    let room = Room::new("test".to_string(), app);
    room.as_ref().into()
}
