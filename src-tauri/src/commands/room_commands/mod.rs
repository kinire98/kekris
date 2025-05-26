pub mod client;

pub mod server;

use std::sync::Arc;

use tokio::sync::{Mutex, OnceCell, broadcast, mpsc::Sender};

use crate::models::dummy_room::DummyRoom;

static END_SEARCH_CHANNEL: OnceCell<Arc<Mutex<Sender<bool>>>> = OnceCell::const_new();
static END_LISTEN_ROOM: OnceCell<Arc<Mutex<broadcast::Sender<bool>>>> = OnceCell::const_new();
static END_ROOM: OnceCell<Arc<Mutex<Sender<bool>>>> = OnceCell::const_new();

static ROOM_INFO: OnceCell<Arc<Mutex<Option<DummyRoom>>>> = OnceCell::const_new();

static END_LISTEN_ROOM_UPDATES: OnceCell<Arc<Mutex<broadcast::Sender<bool>>>> =
    OnceCell::const_new();

static SEND_ROOM_UPDATES: OnceCell<Arc<Mutex<Sender<crate::room::FirstLevelCommands>>>> =
    OnceCell::const_new();

#[tauri::command]
#[cfg(target_os = "macos")]
pub const fn can_host_room() -> bool {
    false
}
#[tauri::command]
#[cfg(not(target_os = "macos"))]
pub const fn can_host_room() -> bool {
    true
}

