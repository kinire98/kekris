use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::helpers::room_net_helpers::{read_enum_from_client, send_enum_from_server};
use crate::models::room_commands::client::ClientRoomNetCommands;
use crate::models::room_commands::server::CloseReason;

use crate::globals::PING_LIMIT_IN_SECONDS;

use tokio::net::TcpStream;

use crate::models::dummy_room::DummyPlayer;

use tokio::sync::mpsc::Sender;
use tokio::sync::{Mutex, broadcast};

use crate::models::room_commands::server::ServerRoomNetCommands;

use super::super::{FirstLevelCommands, Updates};

/// `RoomPlayerListener` listens for updates and commands from a connected player in a room.
pub struct RoomPlayerListener {
    /// Sender for sending commands to the room.
    send_commands: Sender<FirstLevelCommands>,
    /// The TCP stream for communication with the player.
    stream: Arc<Mutex<TcpStream>>,
    /// The player's information.
    player: DummyPlayer,
    /// Receiver for updates from the room.
    updates: broadcast::Receiver<Updates>,
    /// Flag indicating if a ping request has been sent.
    check_ping: bool,
    /// The ping time of the player.
    ping: u64,
    /// The time of the last ping request.
    time_last_ping: u64,
    /// An atomic boolean indicating whether the player is currently playing a game.
    playing: Arc<Mutex<bool>>,
}
impl RoomPlayerListener {
    /// Creates a new `RoomPlayerListener` instance.
    ///
    /// # Arguments
    ///
    /// * `send_commands` - Sender for sending commands to the room.
    /// * `stream` - The TCP stream for communication with the player.
    /// * `player` - The player's information.
    /// * `updates` - Receiver for updates from the room.
    /// * `playing` - An atomic boolean indicating whether the player is currently playing a game.
    pub fn new(
        send_commands: Sender<FirstLevelCommands>,
        stream: Arc<Mutex<TcpStream>>,
        player: DummyPlayer,
        updates: broadcast::Receiver<Updates>,
        playing: Arc<Mutex<bool>>,
    ) -> Self {
        Self {
            send_commands,
            stream,
            player,
            updates,
            check_ping: false,
            ping: 0,
            time_last_ping: 0,
            playing,
        }
    }

    /// Listens for updates from the player and the room.
    pub async fn listen_to_player_updates(&mut self) {
        loop {
            let lock = self.playing.lock().await;
            if !*lock {
                drop(lock);
                let lock = self.stream.clone();
                if self.check_ping {
                    let cur_time = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("Time went backwards 🗿🤙")
                        .as_secs();
                    if cur_time - self.time_last_ping > PING_LIMIT_IN_SECONDS {
                        let _ = self
                            .send_commands
                            .send(FirstLevelCommands::PlayerDisconnected(self.player.clone()))
                            .await;
                        let _ = send_enum_from_server(
                            &lock,
                            &ServerRoomNetCommands::DisconnectedSignal(false),
                        )
                        .await;
                        break;
                    }
                }
                tokio::select! {
                    value = read_enum_from_client(&lock) => {
                        let Ok(content) = value else {
                            continue;
                        };
                        if self.handle_client(content).await {
                            break;
                        }
                    },
                    value = self.updates.recv() => {
                        let command = self.handle_receive_update_error(value, &lock).await;
                        if command.is_none() {
                            break;
                        }
                        self.handle_updates(command.unwrap(), &lock).await;
                    },
                    _ = tokio::time::sleep(Duration::from_secs(PING_LIMIT_IN_SECONDS)) => {}
                }
            } else {
                drop(lock);
                tokio::time::sleep(Duration::from_millis(300)).await;
                self.check_ping = false;
            }
        }
    }
    /// Handles commands received from the client.
    ///
    /// # Arguments
    ///
    /// * `content` - The command received from the client.
    async fn handle_client(&mut self, content: ClientRoomNetCommands) -> bool {
        match content {
            ClientRoomNetCommands::RoomDiscover(_) => (),
            ClientRoomNetCommands::JoinRoomRequest(_) => (),
            ClientRoomNetCommands::LeaveRoom(dummy_player) => {
                let _ = self
                    .send_commands
                    .send(FirstLevelCommands::PlayerDisconnected(dummy_player))
                    .await;
                return true;
            }
            ClientRoomNetCommands::PingResponse(_) => {
                self.check_ping = false;
                let cur_time = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards 🗿🤙")
                    .as_secs();
                self.ping = cur_time - self.time_last_ping;
                let _ = self
                    .send_commands
                    .send(FirstLevelCommands::PingReceived((
                        self.player.clone(),
                        self.ping,
                    )))
                    .await;
            }
        }
        false
    }
    /// Handles errors that occur while receiving updates.
    ///
    /// # Arguments
    ///
    /// * `value` - The result of receiving an update.
    /// * `socket` - The TCP stream for communication with the player.
    async fn handle_receive_update_error(
        &mut self,
        value: Result<Updates, broadcast::error::RecvError>,
        socket: &Arc<Mutex<TcpStream>>,
    ) -> Option<Updates> {
        match value {
            Ok(command) => Some(command),
            Err(error) => match error {
                broadcast::error::RecvError::Closed => {
                    let _ = send_enum_from_server(
                        socket,
                        &ServerRoomNetCommands::RoomClosed(CloseReason::InnerError),
                    )
                    .await;
                    let _ = self.send_commands.send(FirstLevelCommands::FatalFail).await;
                    None
                }
                broadcast::error::RecvError::Lagged(_) => match self.updates.recv().await {
                    Ok(command) => Some(command),
                    Err(error) => match error {
                        broadcast::error::RecvError::Lagged(_) => {
                            let Ok(command) = self.updates.recv().await else {
                                let _ = send_enum_from_server(
                                    socket,
                                    &ServerRoomNetCommands::RoomClosed(CloseReason::InnerError),
                                )
                                .await;

                                let _ =
                                    self.send_commands.send(FirstLevelCommands::FatalFail).await;
                                return None;
                            };
                            Some(command)
                        }
                        broadcast::error::RecvError::Closed => {
                            let _ = send_enum_from_server(
                                socket,
                                &ServerRoomNetCommands::RoomClosed(CloseReason::InnerError),
                            )
                            .await;

                            let _ = self.send_commands.send(FirstLevelCommands::FatalFail).await;
                            None
                        }
                    },
                },
            },
        }
    }
    /// Handles updates received from the room.
    ///
    /// # Arguments
    ///
    /// * `update` - The update received from the room.
    /// * `socket` - The TCP stream for communication with the player.
    async fn handle_updates(&mut self, update: Updates, socket: &Arc<Mutex<TcpStream>>) -> bool {
        match update {
            Updates::PlayersUpdate(players) => {
                let players: Vec<DummyPlayer> =
                    players.iter().map(|player| player.into()).collect();

                let error =
                    send_enum_from_server(socket, &ServerRoomNetCommands::PlayersUpdate(players))
                        .await;
                if error.is_err() {
                    match error.unwrap_err().kind() {
                        std::io::ErrorKind::BrokenPipe
                        | std::io::ErrorKind::UnexpectedEof
                        | std::io::ErrorKind::HostUnreachable
                        | std::io::ErrorKind::ConnectionReset => {
                            let _ = self
                                .send_commands
                                .send(FirstLevelCommands::PlayerDisconnected(self.player.clone()))
                                .await;
                            return true;
                        }
                        _ => (),
                    }
                }
            }
            Updates::NameChanged(_) => todo!(),
            Updates::PlayerLimitChanged(_) => todo!(),
            Updates::RoomEnded => {
                let _ = send_enum_from_server(
                    socket,
                    &ServerRoomNetCommands::RoomClosed(CloseReason::ClosedByHost),
                )
                .await;
                return true;
            }
            Updates::SendPing(playing) => {
                if self.check_ping {
                    return false;
                }
                let result =
                    send_enum_from_server(socket, &ServerRoomNetCommands::PingRequest(playing))
                        .await;
                if result.is_ok() {
                    self.check_ping = true;
                    self.time_last_ping = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("Time went backwards 🗿🤙")
                        .as_secs();
                } else {
                    match result.unwrap_err().kind() {
                        std::io::ErrorKind::BrokenPipe
                        | std::io::ErrorKind::UnexpectedEof
                        | std::io::ErrorKind::HostUnreachable
                        | std::io::ErrorKind::ConnectionReset => {
                            let _ = self
                                .send_commands
                                .send(FirstLevelCommands::PlayerDisconnected(self.player.clone()))
                                .await;
                            return true;
                        }
                        _ => (),
                    }
                }
            }
            Updates::GameStarts((highest_ping, options, pieces)) => {
                let ping = if self.ping > highest_ping {
                    self.ping
                } else {
                    highest_ping - self.ping
                };
                send_enum_from_server(
                    socket,
                    &ServerRoomNetCommands::GameStarts((ping, pieces, options, self.player.id())),
                )
                .await
                .unwrap();

                return true;
            }
        };
        false
    }
}
