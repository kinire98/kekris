use std::time::Duration;

use tauri::{AppHandle, Emitter};

use crate::models::room_info::{RoomInfo, generate_random_info};

const ROOM_UPDATES_EVENT: &str = "room-updates";

#[tauri::command]
pub fn listen_for_rooms(app: AppHandle) -> Vec<RoomInfo> {
    tokio::spawn(async move {
        send_updates(app).await;
    });
    generate_random_info(0)
}

async fn send_updates(app: AppHandle) {
    let mut index = 1;
    loop {
        tokio::time::sleep(Duration::from_secs(10)).await;
        app.emit(ROOM_UPDATES_EVENT, generate_random_info(index))
            .unwrap();
        index += 1;
    }
}
