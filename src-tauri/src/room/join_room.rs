use tauri::AppHandle;
use tokio::net::{TcpSocket, TcpStream, tcp};

use crate::{
    globals::LISTENING_PORT_TCP_SERVER,
    models::{
        dummy_room::{DummyPlayer, DummyRoom},
        room_info::RoomInfo,
    },
};

pub async fn join_room(room: RoomInfo, player: DummyPlayer, app: AppHandle) -> Option<DummyRoom> {
    let tcp_socket = TcpStream::connect(format!("{}:{}", room.ip(), LISTENING_PORT_TCP_SERVER))
        .await
        .unwrap();
}
