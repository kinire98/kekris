use std::sync::Arc;
use std::time::Duration;

use crate::models::room_commands::server::RejectReason;

use crate::globals::LISTENING_DIRECTION_TCP;

use tokio::io::{AsyncReadExt, AsyncWriteExt};

use tokio::net::TcpListener;

use crate::models::dummy_room::{DummyPlayer, DummyRoom};

use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::{Mutex, broadcast};

use crate::{
    globals::SIZE_FOR_KB, models::room_commands::client::ClientRoomNetCommands,
    models::room_commands::server::ServerRoomNetCommands,
};

use super::super::{FirstLevelCommands, Updates};

pub fn listen_to_room_requests(
    send_commands: Sender<FirstLevelCommands>,
    receive_updates: Receiver<Updates>,
    stop_listening: broadcast::Receiver<bool>,
    room: DummyRoom,
    receive_players: Arc<Mutex<u8>>,
    limit_players: u8,
) {
    send_updates(stop_listening, receive_updates);
    self::receive_updates(send_commands, room, receive_players, limit_players);
}

fn receive_updates(
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
                if let ClientRoomNetCommands::JoinRoomRequest(dummy_player) = command {
                    let number = *receive_players.lock().await;
                    if number >= limit_players {
                        let _ = socket
                            .0
                            .write_all(
                                &serde_json::to_vec(
                                    &ServerRoomNetCommands::JoinRoomRequestRejected(
                                        RejectReason::RoomFull,
                                    ),
                                )
                                .expect("Won't panic in a reasonable amount of times"),
                            )
                            .await;
                        return;
                    }
                    if (socket
                        .0
                        .write(
                            &serde_json::to_vec(&ServerRoomNetCommands::JoinRoomRequestAccepted(
                                room,
                            ))
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
                }
            });
        }
    });
}
fn send_updates(
    mut stop_listening: broadcast::Receiver<bool>,
    mut receive_updates: Receiver<Updates>,
) {
    tokio::spawn(async move {
        loop {
            if stop_listening.try_recv().is_ok() {
                break;
            }
            if let Ok(update) = receive_updates.try_recv() {
                match update {
                    Updates::PlayersUpdate(players) => {
                        async {
                            let dummys: Vec<DummyPlayer> =
                                players.iter().clone().map(|player| player.into()).collect();
                            for mut player in players {
                                let stream = player.stream();
                                if stream.is_some() {
                                    stream
                                        .unwrap()
                                        .lock()
                                        .await
                                        .write_all(
                                            &serde_json::to_vec(
                                                &ServerRoomNetCommands::PlayersUpdate(
                                                    dummys.clone(),
                                                ),
                                            )
                                            .expect("Reasonable"),
                                        )
                                        .await
                                        .unwrap();
                                }
                            }
                        }
                        .await
                    }
                    Updates::NameChanged(_) => todo!(),
                    Updates::PlayerLimitChanged(_) => todo!(),
                }
            };
            tokio::time::sleep(Duration::from_micros(16_666)).await;
        }
    });
}
