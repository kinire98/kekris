use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::models::room_commands::server::{CloseReason, RejectReason};

use crate::globals::{LISTENING_DIRECTION_TCP, PING_LIMIT_IN_SECONDS, UPDATES_IN_MILLIS};

use tokio::io::{AsyncReadExt, AsyncWriteExt};

use tokio::net::{TcpListener, TcpStream};

use crate::models::dummy_room::{DummyPlayer, DummyRoom};

use tokio::sync::mpsc::Sender;
use tokio::sync::{Mutex, broadcast};

use crate::{
    globals::SIZE_FOR_KB, models::room_commands::client::ClientRoomNetCommands,
    models::room_commands::server::ServerRoomNetCommands,
};

use super::super::{FirstLevelCommands, Updates};

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
                if number >= limit_players {
                    let _ = socket
                        .0
                        .write_all(
                            &serde_json::to_vec(&ServerRoomNetCommands::JoinRoomRequestRejected(
                                RejectReason::RoomFull,
                            ))
                            .expect("Won't panic in a reasonable amount of times"),
                        )
                        .await;
                    return;
                }
                if (socket
                    .0
                    .write(
                        &serde_json::to_vec(&ServerRoomNetCommands::JoinRoomRequestAccepted(room))
                            .expect("Reasonable to think it won't panic"),
                    )
                    .await)
                    .is_ok()
                {
                    let _ = sender_copy
                        .send(FirstLevelCommands::PlayerConnected((
                            dummy_player,
                            socket.0,
                        )))
                        .await;
                }
            });
        }
    });
}

pub fn listen_to_player_updates(
    send_commands: Sender<FirstLevelCommands>,
    stream: Arc<Mutex<TcpStream>>,
    player: DummyPlayer,
    mut game_starting: broadcast::Receiver<bool>,
    mut updates: broadcast::Receiver<Updates>,
) {
    tokio::spawn(async move {
        let mut buffer = vec![0; SIZE_FOR_KB];
        let mut time_last_ping = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards 🗿🤙")
            .as_secs();
        let mut check_ping = false;
        loop {
            let mut socket = stream.lock().await;
            if check_ping {
                let cur_time = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards 🗿🤙")
                    .as_secs();
                if cur_time - time_last_ping > PING_LIMIT_IN_SECONDS {
                    dbg!("here");
                    let _ = send_commands
                        .send(FirstLevelCommands::PlayerDisconnected(player.clone()))
                        .await;
                    let _ = socket
                        .write(
                            &serde_json::to_vec(&ServerRoomNetCommands::DisconnectedSignal)
                                .expect("Reasonable"),
                        )
                        .await;
                    break;
                }
            }

            tokio::select! {
                _value = game_starting.recv() => {
                    break;
                },
                value = socket.read(&mut buffer) => {
                    let Ok(content) = value else {
                        continue;
                    };
                    let Ok(command) = serde_json::from_slice::<ClientRoomNetCommands>(&buffer[..content]) else {
                        continue;
                    };
                    match command {
                        ClientRoomNetCommands::RoomDiscover => (),
                        ClientRoomNetCommands::JoinRoomRequest(_) => (),
                        ClientRoomNetCommands::LeaveRoom(dummy_player) => {
                            let _ = send_commands.send(FirstLevelCommands::PlayerDisconnected(dummy_player)).await;
                            break;
                        },
                        ClientRoomNetCommands::PingResponse => {
                            dbg!("send_ping_response_received", check_ping);
                            check_ping = false;
                            let cur_time = SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .expect("Time went backwards 🗿🤙")
                                .as_secs();
                            let _ = send_commands.send(FirstLevelCommands::PingReceived((player.clone(), cur_time - time_last_ping))).await;
                        },
                    }
                },
                value = updates.recv() => {
                    dbg!(&value);
                    let command = match value {
                        Ok(command) => command,
                        Err(error) => match error {
                            broadcast::error::RecvError::Closed => {
                                let _ = socket.write(&serde_json::to_vec(&ServerRoomNetCommands::RoomClosed(CloseReason::InnerError)).unwrap()).await;
                                let _ = send_commands.send(FirstLevelCommands::FatalFail).await;
                                break;
                            },
                            broadcast::error::RecvError::Lagged(_) => {
                                match updates.recv().await {
                                    Ok(command) => command,
                                    Err(error) => match error {
                                        broadcast::error::RecvError::Lagged(_) => {
                                            let Ok(command) = updates.recv().await else {
                                                let _ = socket.write(&serde_json::to_vec(&ServerRoomNetCommands::RoomClosed(CloseReason::InnerError)).unwrap()).await;
                                                let _ = send_commands.send(FirstLevelCommands::FatalFail).await;
                                                break;
                                            };
                                            command
                                        },
                                        broadcast::error::RecvError::Closed => {
                                            let _ = socket.write(&serde_json::to_vec(&ServerRoomNetCommands::RoomClosed(CloseReason::InnerError)).unwrap()).await;
                                            let _ = send_commands.send(FirstLevelCommands::FatalFail).await;
                                            break;
                                        },
                                    },
                                }
                            },

                        },
                    };
                    match command {
                        Updates::PlayersUpdate(players) => {
                            let players: Vec<DummyPlayer> = players.iter().map(|player| {
                                player.into()
                            }).collect();
                            let _ = socket.write(&serde_json::to_vec(&ServerRoomNetCommands::PlayersUpdate(players)).expect("Reasonable")).await;
                        },
                        Updates::NameChanged(_) => todo!(),
                        Updates::PlayerLimitChanged(_) => todo!(),
                        Updates::RoomEnded => {
                            let _ = socket.write(&serde_json::to_vec(&ServerRoomNetCommands::RoomClosed(CloseReason::ClosedByHost)).expect("Reasonable")).await;
                            break;
                        },
                        Updates::SendPing => {
                            dbg!("sent_ping", check_ping);
                            if check_ping {
                                continue;
                            }
                            let result = socket.write(&serde_json::to_vec(&ServerRoomNetCommands::PingRequest).expect("Reasonable to expect not to panic")).await;
                            if result.is_ok() {
                                check_ping = true;
                                time_last_ping = SystemTime::now()
                                    .duration_since(UNIX_EPOCH)
                                    .expect("Time went backwards 🗿🤙")
                                    .as_secs();
                            }
                        }
                    };
                },
                _ = tokio::time::sleep(Duration::from_secs(PING_LIMIT_IN_SECONDS)) => {}
            }
        }
    });
}
