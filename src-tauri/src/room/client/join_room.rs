use tauri::{AppHandle, Emitter};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::{
    globals::{LISTENING_PORT_TCP_SERVER, SIZE_FOR_KB},
    models::{
        dummy_room::{DummyPlayer, DummyRoom},
        room_commands::client::ClientRoomNetCommands,
        room_commands::server::ServerRoomNetCommands,
        room_info::RoomInfo,
    },
};
const CONNECTION_ERROR: &str = "connection_error";
const CONNECTION_REJECTED: &str = "connection_rejected";
// const PLAYERS_EMIT: &str = "playersEmit";

pub async fn join_room(
    room: RoomInfo,
    player: DummyPlayer,
    app: &AppHandle,
) -> Option<(DummyRoom, TcpStream)> {
    let Ok(mut tcp_socket) =
        TcpStream::connect(format!("{}:{}", room.ip(), LISTENING_PORT_TCP_SERVER)).await
    else {
        let _ = app.emit(CONNECTION_ERROR, false);
        return None;
    };
    let Ok(request) = serde_json::to_vec(&ClientRoomNetCommands::JoinRoomRequest(player)) else {
        return None;
    };

    let result = tcp_socket.write_all(&request).await;

    let Ok(_) = result else {
        let _ = app.emit(CONNECTION_ERROR, false);
        return None;
    };
    let _ = tcp_socket.flush().await;

    let mut buffer = vec![0; SIZE_FOR_KB];
    let Ok(response) = tcp_socket.read(&mut buffer).await else {
        let _ = app.emit(CONNECTION_ERROR, false);
        return None;
    };
    let Ok(command) = serde_json::from_slice::<ServerRoomNetCommands>(&buffer[..response]) else {
        return None;
    };
    if let ServerRoomNetCommands::JoinRoomRequestAccepted(dummy_room) = command {
        Some((dummy_room, tcp_socket))
    } else if let ServerRoomNetCommands::JoinRoomRequestRejected(reject_reason) = command {
        let _ = app.emit(CONNECTION_REJECTED, reject_reason);
        None
    } else {
        None
    }
}
