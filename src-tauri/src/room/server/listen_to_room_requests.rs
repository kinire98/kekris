use std::sync::Arc;

use tokio::{
    io::AsyncReadExt,
    net::TcpListener,
    sync::{Mutex, mpsc::Sender},
};

use crate::{
    globals::{LISTENING_DIRECTION_TCP, SIZE_FOR_KB},
    helpers::room_net_helpers::send_enum_from_server,
    models::{
        dummy_room::DummyRoom,
        room_commands::{
            client::ClientRoomNetCommands,
            server::{RejectReason, ServerRoomNetCommands},
        },
    },
    room::FirstLevelCommands,
};

/// Listens for incoming TCP connections to the room.
///
/// This function sets up a TCP listener to accept incoming connection requests.
/// For each incoming connection, it reads the request, validates it, and either
/// accepts or rejects the connection based on the room's current state.
///
/// # Arguments
///
/// * `send_commands` - A sender for sending commands to the main room logic.
/// * `room` - The `DummyRoom` representing the room.
/// * `receive_players` - An `Arc<Mutex<u8>>` containing the number of players currently in the room.
/// * `limit_players` - The maximum number of players allowed in the room.
pub fn listen_to_room_requests(
    send_commands: Sender<FirstLevelCommands>,
    room: DummyRoom,
    receive_players: Arc<Mutex<u8>>,
    limit_players: u8,
) {
    tokio::spawn(async move {
        let Ok(listener) = TcpListener::bind(LISTENING_DIRECTION_TCP).await else {
            send_commands
                .send(FirstLevelCommands::FatalFail)
                .await
                .expect("Irrelevant");
            return;
        };
        loop {
            let Ok(mut socket) = listener.accept().await else {
                continue;
            };
            let sender_copy = send_commands.clone();
            let receive_players = receive_players.clone();
            let room = room.clone();
            tokio::spawn(async move {
                let mut buffer = vec![0; SIZE_FOR_KB];
                let Ok(content) = socket.0.read(&mut buffer).await else {
                    return;
                };
                let Ok(command) =
                    serde_json::from_slice::<ClientRoomNetCommands>(&buffer[..content])
                else {
                    return;
                };
                let ClientRoomNetCommands::JoinRoomRequest(dummy_player) = command else {
                    return;
                };
                let number = *receive_players.lock().await;
                let socket = Arc::new(Mutex::new(socket.0));
                if number >= limit_players {
                    let _ = send_enum_from_server(
                        &socket,
                        &ServerRoomNetCommands::JoinRoomRequestRejected(RejectReason::RoomFull),
                    )
                    .await;
                    return;
                }
                if (send_enum_from_server(
                    &socket,
                    &ServerRoomNetCommands::JoinRoomRequestAccepted(room),
                )
                .await)
                    .is_ok()
                {
                    let _ = sender_copy
                        .send(FirstLevelCommands::PlayerConnected((dummy_player, socket)))
                        .await;
                }
            });
        }
    });
}
