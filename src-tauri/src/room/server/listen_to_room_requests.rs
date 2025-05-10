use std::sync::Arc;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    sync::{Mutex, mpsc::Sender},
};

use crate::{
    globals::{LISTENING_DIRECTION_TCP, SIZE_FOR_KB},
    models::{
        dummy_room::DummyRoom,
        room_commands::{
            client::ClientRoomNetCommands,
            server::{RejectReason, ServerRoomNetCommands},
        },
    },
    room::FirstLevelCommands,
};

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
