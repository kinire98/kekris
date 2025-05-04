use std::time::Duration;

use tauri::{AppHandle, Emitter};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::broadcast,
};

use crate::{
    globals::SIZE_FOR_KB,
    models::{
        dummy_room::DummyPlayer,
        room_commands::{client::ClientRoomNetCommands, server::ServerRoomNetCommands},
    },
};

const PLAYERS_EMIT: &str = "playersEmit";

pub async fn listen_to_room_updates(
    mut stream: TcpStream,
    app: AppHandle,
    mut stop_channel: broadcast::Receiver<bool>,
    player: DummyPlayer,
) {
    tokio::spawn(async move {
        let mut buffer = vec![0; SIZE_FOR_KB];
        stream.write_all("alsdf".as_bytes()).await.unwrap();
        loop {
            tokio::select! {
                content = stream.read(&mut buffer) => {
                    if let Ok(content) = content {
                        if let Ok(command) = serde_json::from_slice::<ServerRoomNetCommands>(&buffer[..content]) {
                            match command {
                                ServerRoomNetCommands::RoomDiscoverResponse(_) => (),
                                ServerRoomNetCommands::JoinRoomRequestAccepted(_) => (),
                                ServerRoomNetCommands::JoinRoomRequestRejected(_) => (),
                                ServerRoomNetCommands::PlayersUpdate(dummy_players) => {
                                    dbg!("here");
                                    let _ = app.emit(PLAYERS_EMIT, dummy_players);
                                }
                            }
                        }
                    }
                },
                _ = stop_channel.recv() => {
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
        }
    });
}
