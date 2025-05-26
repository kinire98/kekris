use std::{sync::Arc, time::Duration};

use tauri::{AppHandle, Emitter};
use tokio::{
    net::UdpSocket,
    sync::{Mutex, broadcast},
};

use crate::{
    globals::{LISTENING_DIRECTION_BROADCAST, LISTENING_RESPONSE_PORT_BROADCAST, SIZE_FOR_KB},
    models::{
        room_commands::client::ClientRoomNetCommands, room_commands::server::ServerRoomNetCommands,
        room_info::RoomInfo,
    },
};

const ERROR_OPENING_CONNECTION_EMIT: &str = "error_opening_connection";

/// Listens for broadcast requests for room discovery.
///
/// This function binds to a UDP socket and listens for incoming broadcast messages.
/// When a valid `RoomDiscover` command is received, it responds with the room's information.
///
/// # Arguments
///
/// * `info` - The `RoomInfo` to respond with.
/// * `app` - A Tauri `AppHandle` for emitting events.
/// * `stop_channel` - A channel for receiving a signal to stop listening.
/// * `players_num` - An `Arc<Mutex<u8>>` containing the number of players in the room.
pub fn listen_to_request(
    mut info: RoomInfo,
    app: AppHandle,
    mut stop_channel: broadcast::Receiver<bool>,
    players_num: Arc<Mutex<u8>>,
) {
    tokio::spawn(async move {
        let Ok(socket) = UdpSocket::bind(LISTENING_DIRECTION_BROADCAST).await else {
            finish_listening(&app);
            return;
        };
        socket
            .set_broadcast(true)
            .expect("Check for not broadcast allowance");
        let mut data = vec![0; SIZE_FOR_KB];
        loop {
            tokio::select! {
                players_amount = players_num.lock() => {
                    info.change_number_of_players(*players_amount);
                },
                socket_info = socket.recv_from(&mut data) => {
                    let Ok((valid_bytes, mut addr)) = socket_info else {
                        finish_listening(&app);
                        return;
                    };
                    let deserialized: Result<ClientRoomNetCommands, serde_json::Error> =
                        serde_json::from_slice(&data[..valid_bytes]);
                    let Ok(net_command) = deserialized else {
                        return;
                    };
                    addr.set_port(LISTENING_RESPONSE_PORT_BROADCAST);
                    if let ClientRoomNetCommands::RoomDiscover(_) = net_command {
                        let _ = socket
                            .send_to(
                                &serde_json::to_vec(&ServerRoomNetCommands::RoomDiscoverResponse(
                                    info.clone(),
                                ))
                                .expect("It's reasonable to expect that this shouldn't panic"),
                                addr,
                            )
                            .await;
                    }
                },
                _ = stop_channel.recv() => {
                    break;
                }
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    });
}

/// Emits an event to indicate that there was an error opening the connection.
///
/// # Arguments
///
/// * `app` - A Tauri `AppHandle` for emitting events.
fn finish_listening(app: &AppHandle) {
    let _ = app.emit(ERROR_OPENING_CONNECTION_EMIT, false);
}
