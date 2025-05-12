use std::{
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use tauri::{AppHandle, Emitter};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::{
        Mutex,
        broadcast::{self, error::RecvError},
    },
};

use crate::{
    game::game_types::client_online_game::ClientOnlineGame,
    globals::{PING_LIMIT_IN_SECONDS, SIZE_FOR_KB},
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
}

impl ClientRoom {
    pub fn new(
        stream: TcpStream,
        app: AppHandle,
        stop_channel: broadcast::Receiver<bool>,
        player: DummyPlayer,
    ) -> Self {
        Self {
            stream: Arc::new(Mutex::new(stream)),
            app,
            stop_channel,
            player,
        }
    }
    pub async fn listen(&mut self) {
        let mut buffer = vec![0; SIZE_FOR_KB];
        let mut time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards 🗿🤙")
            .as_secs();
        let lock = self.stream.clone();
        loop {
            let mut stream = lock.lock().await;
            tokio::select! {
                content = stream.read(&mut buffer) => {
                    drop(stream);
                    time = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("Time went backwards 🗿🤙")
                        .as_secs();
                    self.handle_content(content, &buffer).await;
                },
                value = self.stop_channel.recv() => {
                    drop(stream);
                    let break_loop = self.stop_listening(value).await;
                    if break_loop {
                        break;
                    }
                },
                _ = tokio::time::sleep(Duration::from_secs(PING_LIMIT_IN_SECONDS)) => {}
            }
            let cur_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards 🗿🤙")
                .as_secs();
            if cur_time - time > PING_LIMIT_IN_SECONDS {
                let _ = self.app.emit(LOST_CONNECTION_EMIT, false);
                break;
            }
        }
    }
    async fn handle_content(
        &mut self,
        content: Result<usize, std::io::Error>,
        buffer: &[u8],
    ) -> bool {
        if let Ok(content) = content {
            if let Ok(command) = serde_json::from_slice::<ServerRoomNetCommands>(&buffer[..content])
            {
                match command {
                    ServerRoomNetCommands::RoomDiscoverResponse(_) => (),
                    ServerRoomNetCommands::JoinRoomRequestAccepted(_) => (),
                    ServerRoomNetCommands::JoinRoomRequestRejected(_) => (),
                    ServerRoomNetCommands::PlayersUpdate(dummy_players) => {
                        let _ = self.app.emit(PLAYERS_EMIT, dummy_players);
                    }
                    ServerRoomNetCommands::RoomClosed(_) => {
                        self.app.emit(ROOM_CLOSED_EMIT, false).unwrap();
                        return true;
                    }
                    ServerRoomNetCommands::PingRequest(_) => {
                        return self.ping().await;
                    }
                    ServerRoomNetCommands::DisconnectedSignal => {
                        let _ = self.app.emit(LOST_CONNECTION_EMIT, false);
                        return true;
                    }
                    ServerRoomNetCommands::GameStarts((delay, pieces, options)) => {
                        tokio::time::sleep(Duration::from_millis(delay)).await;
                        let _ = self.app.emit(GAME_STARTED_EMIT, true);
                        let mut game = ClientOnlineGame::new(
                            self.stream.clone(),
                            pieces,
                            options,
                            self.app.clone(),
                            delay,
                        )
                        .await;
                        tokio::spawn(async move {
                            game.start().await;
                        });
                    }
                }
            }
        }
        false
    }
    async fn ping(&mut self) -> bool {
        let lock = self.stream.clone();
        let mut stream = lock.lock().await;
        let result = stream
            .write(
                &serde_json::to_vec(&ClientRoomNetCommands::PingResponse)
                    .expect("Reasonable to expect not to panic"),
            )
            .await;
        drop(stream);
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
                let Ok(_) = lock
                    .lock()
                    .await
                    .write(
                        &serde_json::to_vec(&ClientRoomNetCommands::LeaveRoom(self.player.clone()))
                            .expect("Reasonable to expect not to panic"),
                    )
                    .await
                else {
                    let _ = lock
                        .lock()
                        .await
                        .write(
                            &serde_json::to_vec(&ClientRoomNetCommands::LeaveRoom(
                                self.player.clone(),
                            ))
                            .expect("Reasonable to expect not to panic"),
                        )
                        .await;
                    return true;
                };
                return true;
            }
        }
        false
    }
}
