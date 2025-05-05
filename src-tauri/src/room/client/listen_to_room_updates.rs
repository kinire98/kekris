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
const ROOM_CLOSED_EMIT: &str = "roomClosed";

pub async fn listen_to_room_updates(
    mut stream: TcpStream,
    app: AppHandle,
    mut stop_channel: broadcast::Receiver<bool>,
    player: DummyPlayer,
) {
    tokio::spawn(async move {
        let mut buffer = vec![0; SIZE_FOR_KB];
        // stream.write_all("alsdf".as_bytes()).await.unwrap();
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
                                    let _  = app.emit(PLAYERS_EMIT, dummy_players);
                                }
                                ServerRoomNetCommands::RoomClosed => {
                                    let _ = app.emit(ROOM_CLOSED_EMIT, false);
                                    break;
                                },
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
                                println!("here");
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
            }
        }
    });
}
