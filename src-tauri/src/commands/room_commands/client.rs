use std::sync::Arc;

use tauri::AppHandle;
use tokio::sync::{Mutex, broadcast, mpsc};

use super::{END_LISTEN_ROOM_UPDATES, END_SEARCH_CHANNEL, ROOM_INFO};
use crate::{
    globals::SIZE_FOR_KB,
    models::{
        dummy_room::{DummyPlayer, DummyRoom},
        room_info::RoomInfo,
    },
    room::client::{
        listen_to_room_updates::listen_to_room_updates, listen_to_rooms::listen_to_rooms,
    },
};

// const ROOM_NAME_EMIT: &str = "roomNameEmit";

#[tauri::command]
pub async fn stop_search() {
    if let Some(tx) = END_SEARCH_CHANNEL.get() {
        tx.lock().await.send(false).await.unwrap();
    }
    if let Some(tx) = END_LISTEN_ROOM_UPDATES.get() {
        tx.lock().await.send(false).unwrap();
    }
}

#[tauri::command]
pub async fn join_room(app: AppHandle, room: RoomInfo, player: DummyPlayer) {
    let player = DummyPlayer::fill(player);
    let (tx, rx) = broadcast::channel(SIZE_FOR_KB);
    let Some((room, stream)) =
        crate::room::client::join_room::join_room(room, player.clone(), &app).await
    else {
        return;
    };

    listen_to_room_updates(stream, app, rx, player).await;
    stop_search().await;
    if let Some(room_old) = ROOM_INFO.get() {
        let mut room_old = room_old.lock().await;
        *room_old = Some(room);
    } else {
        ROOM_INFO
            .set(Arc::new(Mutex::new(Some(room))))
            .expect("No reason to panic");
    }
    if let Some(lock) = END_LISTEN_ROOM_UPDATES.get() {
        let mut channel_old = lock.lock().await;
        *channel_old = tx;
    } else {
        END_LISTEN_ROOM_UPDATES
            .set(Arc::new(Mutex::new(tx)))
            .expect("No reason to panic");
    }
}

#[tauri::command]
pub async fn leave_room() {
    stop_search().await;
}

#[tauri::command]
pub async fn room_info() -> Option<DummyRoom> {
    if let Some(room) = ROOM_INFO.get() {
        return room.lock().await.clone();
    }
    None
}

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
