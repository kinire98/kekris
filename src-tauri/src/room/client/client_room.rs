use std::{
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use tauri::{AppHandle, Emitter};
use tokio::{
    io::AsyncWriteExt,
    net::TcpStream,
    sync::{
        Mutex,
        broadcast::{self, error::RecvError},
    },
};

use crate::{
    game::game_types::client_online_game::ClientOnlineGame,
    globals::PING_LIMIT_IN_SECONDS,
    helpers::room_net_helpers::{read_enum_from_server, send_enum_from_client},
    models::{
        dummy_room::DummyPlayer,
        room_commands::{client::ClientRoomNetCommands, server::ServerRoomNetCommands},
    },
};

const PLAYERS_EMIT: &str = "playersEmit";
const ROOM_CLOSED_EMIT: &str = "roomClosed";
const LOST_CONNECTION_EMIT: &str = "connectionLost";
const GAME_STARTED_EMIT: &str = "gameStartedEmit";

pub struct ClientRoom {
    stream: Arc<Mutex<TcpStream>>,
    app: AppHandle,
    stop_channel: broadcast::Receiver<bool>,
    player: DummyPlayer,
    listening: bool,
    playing: Arc<Mutex<bool>>,
}

impl ClientRoom {
    pub fn new(
        stream: Arc<Mutex<TcpStream>>,
        app: AppHandle,
        stop_channel: broadcast::Receiver<bool>,
        player: DummyPlayer,
    ) -> Self {
        Self {
            stream,
            app,
            stop_channel,
            player,
            listening: true,
            playing: Arc::new(Mutex::new(false)),
        }
    }
    pub async fn listen(&mut self) {
        let mut time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards 🗿🤙")
            .as_secs();
        let lock = self.stream.clone();
        while self.listening {
            let lock_loop = self.playing.lock().await;
            dbg!(&lock_loop);
            if !*lock_loop {
                drop(lock_loop);
                dbg!("here");
                tokio::select! {
                    command = read_enum_from_server(&lock) => {
                        dbg!("here");
                        time = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .expect("Time went backwards 🗿🤙")
                            .as_secs();
                        if let Ok(content) = command {
                            dbg!("here");
                            self.handle_content(content).await;
                        } else {
                            dbg!(&command);
                            let command = read_enum_from_server(&lock).await;
                            if let Ok(command) = command {
                                dbg!("here");
                                self.handle_content(command).await;
                            } else {
                                dbg!(&command);
                                let command = read_enum_from_server(&lock).await;
                                dbg!(&command);
                                if let Ok(content) = command {
                                    dbg!("here");
                                    self.handle_content(content).await;
                                } else {
                                    dbg!(&command);
                                }
                            }
                        }
                        dbg!("here");
                    },
                    value = self.stop_channel.recv() => {
                        dbg!("here");
                        let break_loop = self.stop_listening(value).await;
                        if break_loop {
                            break;
                        }
                    },
                    _ = tokio::time::sleep(Duration::from_secs(1)) => {}
                }

                let cur_time = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards 🗿🤙")
                    .as_secs();
                if cur_time - time > PING_LIMIT_IN_SECONDS {
                    let _ = self.app.emit(LOST_CONNECTION_EMIT, false);
                    break;
                }
            } else {
                drop(lock_loop);
                tokio::time::sleep(Duration::from_millis(300)).await;
                time = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards 🗿🤙")
                    .as_secs()
            }
        }
    }
    async fn handle_content(&mut self, content: ServerRoomNetCommands) {
        match content {
            ServerRoomNetCommands::RoomDiscoverResponse(_) => (),
            ServerRoomNetCommands::JoinRoomRequestAccepted(_) => (),
            ServerRoomNetCommands::JoinRoomRequestRejected(_) => (),
            ServerRoomNetCommands::PlayersUpdate(dummy_players) => {
                let _ = self.app.emit(PLAYERS_EMIT, dummy_players);
            }
            ServerRoomNetCommands::RoomClosed(_) => {
                self.app.emit(ROOM_CLOSED_EMIT, false).unwrap();
                self.listening = false;
            }
            ServerRoomNetCommands::PingRequest(_) => {
                self.listening = !self.ping().await;
            }
            ServerRoomNetCommands::DisconnectedSignal(_) => {
                let _ = self.app.emit(LOST_CONNECTION_EMIT, false);
                self.listening = false;
            }
            ServerRoomNetCommands::GameStarts((delay, pieces, options, id)) => {
                let _ = self.app.emit(GAME_STARTED_EMIT, id);
                let mut game = ClientOnlineGame::new(
                    self.stream.clone(),
                    pieces,
                    options,
                    self.app.clone(),
                    delay,
                    self.playing.clone(),
                    self.player.clone(),
                )
                .await;
                *self.playing.lock().await = true;
                tokio::spawn(async move {
                    tokio::time::sleep(Duration::from_millis(delay)).await;
                    game.start().await;
                });
            }
        }
    }
    async fn ping(&mut self) -> bool {
        let lock = self.stream.clone();
        let result =
            send_enum_from_client(&lock, &ClientRoomNetCommands::PingResponse(false)).await;
        if result.is_err() {
            let error = result.unwrap_err();
            match error.kind() {
                std::io::ErrorKind::PermissionDenied
                | std::io::ErrorKind::ConnectionRefused
                | std::io::ErrorKind::ConnectionReset
                | std::io::ErrorKind::HostUnreachable
                | std::io::ErrorKind::NetworkUnreachable
                | std::io::ErrorKind::ConnectionAborted
                | std::io::ErrorKind::NotConnected
                | std::io::ErrorKind::AddrNotAvailable
                | std::io::ErrorKind::NetworkDown
                | std::io::ErrorKind::BrokenPipe
                | std::io::ErrorKind::WouldBlock
                | std::io::ErrorKind::TimedOut
                | std::io::ErrorKind::Interrupted
                | std::io::ErrorKind::UnexpectedEof => {
                    let _ = self.app.emit(LOST_CONNECTION_EMIT, false);
                    return true;
                }
                _ => (),
            }
        }
        false
    }
    async fn stop_listening(&mut self, value: Result<bool, RecvError>) -> bool {
        if let Ok(value_recv) = value {
            if value_recv {
                let lock = self.stream.clone();
                let mut lock = lock.lock().await;
                let result = lock
                    .write_all(
                        &serde_json::to_vec(&ClientRoomNetCommands::LeaveRoom(self.player.clone()))
                            .expect("Reasonable to expect not to panic"),
                    )
                    .await;
                let _ = lock.flush().await;
                let Ok(_) = result else {
                    let _ = lock
                        .write_all(
                            &serde_json::to_vec(&ClientRoomNetCommands::LeaveRoom(
                                self.player.clone(),
                            ))
                            .expect("Reasonable to expect not to panic"),
                        )
                        .await;
                    let _ = lock.flush().await;
                    return true;
                };
                return true;
            }
        }
        false
    }
}
