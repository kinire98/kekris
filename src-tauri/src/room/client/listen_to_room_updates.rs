use std::time::{Duration, SystemTime, UNIX_EPOCH};

use tauri::{AppHandle, Emitter};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::broadcast,
};

use crate::{
    globals::{PING_LIMIT_IN_SECONDS, SIZE_FOR_KB},
    models::{
        dummy_room::DummyPlayer,
        room_commands::{client::ClientRoomNetCommands, server::ServerRoomNetCommands},
    },
};

const PLAYERS_EMIT: &str = "playersEmit";
const ROOM_CLOSED_EMIT: &str = "roomClosed";
const LOST_CONNECTION_EMIT: &str = "connectionLost";

pub async fn listen_to_room_updates(
    mut stream: TcpStream,
    app: AppHandle,
    mut stop_channel: broadcast::Receiver<bool>,
    player: DummyPlayer,
) {
    tokio::spawn(async move {
        let mut buffer = vec![0; SIZE_FOR_KB];
        let mut time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards 🗿🤙")
            .as_secs();
        loop {
            tokio::select! {
                content = stream.read(&mut buffer) => {
                    if let Ok(content) = content {
                        if let Ok(command) = serde_json::from_slice::<ServerRoomNetCommands>(&buffer[..content]) {
                            time = SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .expect("Time went backwards 🗿🤙")
                                .as_secs();
                            match command {
                                ServerRoomNetCommands::RoomDiscoverResponse(_) => (),
                                ServerRoomNetCommands::JoinRoomRequestAccepted(_) => (),
                                ServerRoomNetCommands::JoinRoomRequestRejected(_) => (),
                                ServerRoomNetCommands::PlayersUpdate(dummy_players) => {
                                    let _ = app.emit(PLAYERS_EMIT,dummy_players);
                                }
                                ServerRoomNetCommands::RoomClosed(_) => {
                                    app.emit(ROOM_CLOSED_EMIT,false).unwrap();
                                    break;
                                },
                                ServerRoomNetCommands::PingRequest => {
                                    dbg!("ping_request_received");
                                    let result = stream
                                    .write(&serde_json::to_vec(&ClientRoomNetCommands::PingResponse)
                                    .expect("Reasonable to expect not to panic")).await;
                                    dbg!(&result);
                                    if result.is_err() {
                                        let error = result.unwrap_err();
                                        match error.kind() {
                                            std::io::ErrorKind::PermissionDenied |
                                            std::io::ErrorKind::ConnectionRefused |
                                            std::io::ErrorKind::ConnectionReset |
                                            std::io::ErrorKind::HostUnreachable |
                                            std::io::ErrorKind::NetworkUnreachable |
                                            std::io::ErrorKind::ConnectionAborted |
                                            std::io::ErrorKind::NotConnected |
                                            std::io::ErrorKind::AddrNotAvailable |
                                            std::io::ErrorKind::NetworkDown |
                                            std::io::ErrorKind::BrokenPipe |
                                            std::io::ErrorKind::WouldBlock |
                                            std::io::ErrorKind::TimedOut |
                                            std::io::ErrorKind::Interrupted |
                                            std::io::ErrorKind::UnexpectedEof  => {
                                                let _ = app.emit(LOST_CONNECTION_EMIT, false);
                                                break;
                                            },
                                            _ => (),
                                        }
                                    }
                                    dbg!("ping_response_sent");
                                },
                                ServerRoomNetCommands::DisconnectedSignal => {
                                    let _ = app.emit(LOST_CONNECTION_EMIT, false);
                                    break;
                                }
                            }
                        }
                    }
                },
                value = stop_channel.recv() => {
                    if let Ok(value_recv) = value{
                        if value_recv {
                            let Ok(_) = stream
                                .write(
                                    &serde_json::to_vec(&ClientRoomNetCommands::LeaveRoom(player.clone()))
                                        .expect("Reasonable to expect not to panic"),
                                )
                                .await
                            else {
                                let _ = stream
                                    .write(
                                        &serde_json::to_vec(&ClientRoomNetCommands::LeaveRoom(player.clone()))
                                            .expect("Reasonable to expect not to panic"),
                                    )
                                    .await;
                                break;
                            };
                            break;
                        }
                    }
                },
                _ = tokio::time::sleep(Duration::from_secs(PING_LIMIT_IN_SECONDS)) => {}
            }
            let cur_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards 🗿🤙")
                .as_secs();
            if cur_time - time > PING_LIMIT_IN_SECONDS {
                let _ = app.emit(LOST_CONNECTION_EMIT, false);
                break;
            }
        }
    });
}
