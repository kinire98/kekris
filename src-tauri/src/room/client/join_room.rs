use std::sync::Arc;

use tauri::{AppHandle, Emitter};
use tokio::{io::AsyncWriteExt, net::TcpStream, sync::Mutex};

use crate::{
    globals::LISTENING_PORT_TCP_SERVER,
    helpers::room_net_helpers::read_enum_from_server,
    models::{
        dummy_room::{DummyPlayer, DummyRoom},
        room_commands::{client::ClientRoomNetCommands, server::ServerRoomNetCommands},
        room_info::RoomInfo,
    },
};
const CONNECTION_ERROR: &str = "connection_error";
const CONNECTION_REJECTED: &str = "connection_rejected";

/// Attempts to join a room at the specified `RoomInfo`.
///
/// Establishes a TCP connection, sends a join request, and processes the server's response.
///
/// # Arguments
///
/// * `room` - The `RoomInfo` of the room to join.
/// * `player` - The `DummyPlayer` representing the player joining the room.
/// * `app` - A reference to the Tauri `AppHandle` for emitting events.
///
/// # Returns
///
/// An `Option` containing a tuple of `DummyRoom` and `Arc<Mutex<TcpStream>>` if the join request is accepted,
/// or `None` if the connection fails or the join request is rejected.
pub async fn join_room(
    room: RoomInfo,
    player: DummyPlayer,
    app: &AppHandle,
) -> Option<(DummyRoom, Arc<Mutex<TcpStream>>)> {
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

    let stream = Arc::new(Mutex::new(tcp_socket));
    let Ok(command) = read_enum_from_server(&stream).await else {
        let _ = app.emit(CONNECTION_ERROR, false);
        return None;
    };
    if let ServerRoomNetCommands::JoinRoomRequestAccepted(dummy_room) = command {
        Some((dummy_room, stream))
    } else if let ServerRoomNetCommands::JoinRoomRequestRejected(reject_reason) = command {
        let _ = app.emit(CONNECTION_REJECTED, reject_reason);
        None
    } else {
        None
    }
}
