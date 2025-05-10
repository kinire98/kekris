use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::models::room_commands::server::CloseReason;

use crate::globals::PING_LIMIT_IN_SECONDS;

use tokio::io::{AsyncReadExt, AsyncWriteExt};

use tokio::net::TcpStream;

use crate::models::dummy_room::DummyPlayer;

use tokio::sync::mpsc::Sender;
use tokio::sync::{Mutex, MutexGuard, broadcast};

use crate::{
    globals::SIZE_FOR_KB, models::room_commands::client::ClientRoomNetCommands,
    models::room_commands::server::ServerRoomNetCommands,
};

use super::super::{FirstLevelCommands, Updates};

pub struct RoomPlayerListener {
    send_commands: Sender<FirstLevelCommands>,
    stream: Arc<Mutex<TcpStream>>,
    player: DummyPlayer,
    updates: broadcast::Receiver<Updates>,
    check_ping: bool,
    ping: u64,
    time_last_ping: u64,
    buffer: Vec<u8>,
}
impl RoomPlayerListener {
    pub fn new(
        send_commands: Sender<FirstLevelCommands>,
        stream: Arc<Mutex<TcpStream>>,
        player: DummyPlayer,
        updates: broadcast::Receiver<Updates>,
    ) -> Self {
        Self {
            send_commands,
            stream,
            player,
            updates,
            check_ping: false,
            ping: 0,
            time_last_ping: 0,
            buffer: vec![0; SIZE_FOR_KB],
        }
    }

    pub async fn listen_to_player_updates(&mut self) {
        loop {
            let lock = self.stream.clone();
            let mut socket = lock.lock().await;
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
                value = socket.read(&mut self.buffer) => {
                    let Ok(content) = value else {
                        continue;
                    };
                    if self.handle_client(content).await {
                        break;
                    }
                },
                value = self.updates.recv() => {
                    let command = self.handle_receive_update_error(value, &mut socket).await;
                    if command.is_none() {
                        break;
                    }
                    self.handle_updates(command.unwrap(), socket).await;
                },
                _ = tokio::time::sleep(Duration::from_secs(PING_LIMIT_IN_SECONDS)) => {}
            }
        }
    }
    async fn handle_client(&mut self, content: usize) -> bool {
        let Ok(command) = serde_json::from_slice::<ClientRoomNetCommands>(&self.buffer[..content])
        else {
            return false;
        };
        match command {
            ClientRoomNetCommands::RoomDiscover => (),
            ClientRoomNetCommands::JoinRoomRequest(_) => (),
            ClientRoomNetCommands::LeaveRoom(dummy_player) => {
                let _ = self
                    .send_commands
                    .send(FirstLevelCommands::PlayerDisconnected(dummy_player))
                    .await;
                return true;
            }
            ClientRoomNetCommands::PingResponse => {
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
    async fn handle_receive_update_error(
        &mut self,
        value: Result<Updates, broadcast::error::RecvError>,
        socket: &mut MutexGuard<'_, TcpStream>,
    ) -> Option<Updates> {
        match value {
            Ok(command) => Some(command),
            Err(error) => match error {
                broadcast::error::RecvError::Closed => {
                    let _ = socket
                        .write(
                            &serde_json::to_vec(&ServerRoomNetCommands::RoomClosed(
                                CloseReason::InnerError,
                            ))
                            .unwrap(),
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
                                let _ = socket
                                    .write(
                                        &serde_json::to_vec(&ServerRoomNetCommands::RoomClosed(
                                            CloseReason::InnerError,
                                        ))
                                        .unwrap(),
                                    )
                                    .await;
                                let _ =
                                    self.send_commands.send(FirstLevelCommands::FatalFail).await;
                                return None;
                            };
                            Some(command)
                        }
                        broadcast::error::RecvError::Closed => {
                            let _ = socket
                                .write(
                                    &serde_json::to_vec(&ServerRoomNetCommands::RoomClosed(
                                        CloseReason::InnerError,
                                    ))
                                    .unwrap(),
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
    async fn handle_updates(
        &mut self,
        update: Updates,
        mut socket: MutexGuard<'_, TcpStream>,
    ) -> bool {
        match update {
            Updates::PlayersUpdate(players) => {
                let players: Vec<DummyPlayer> =
                    players.iter().map(|player| player.into()).collect();
                let _ = socket
                    .write(
                        &serde_json::to_vec(&ServerRoomNetCommands::PlayersUpdate(players))
                            .expect("Reasonable"),
                    )
                    .await;
            }
            Updates::NameChanged(_) => todo!(),
            Updates::PlayerLimitChanged(_) => todo!(),
            Updates::RoomEnded => {
                let _ = socket
                    .write(
                        &serde_json::to_vec(&ServerRoomNetCommands::RoomClosed(
                            CloseReason::ClosedByHost,
                        ))
                        .expect("Reasonable"),
                    )
                    .await;
                return true;
            }
            Updates::SendPing(playing) => {
                if self.check_ping {
                    return false;
                }
                let result = socket
                    .write(
                        &serde_json::to_vec(&ServerRoomNetCommands::PingRequest(playing))
                            .expect("Reasonable to expect not to panic"),
                    )
                    .await;
                if result.is_ok() {
                    self.check_ping = true;
                    self.time_last_ping = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("Time went backwards 🗿🤙")
                        .as_secs();
                }
            }
            Updates::GameStarts(highest_ping) => {
                let _ = socket
                    .write(
                        &serde_json::to_vec(&ServerRoomNetCommands::GameStarts(
                            highest_ping - self.ping,
                        ))
                        .expect("Reasonable"),
                    )
                    .await;
                return true;
            }
        };
        false
    }
}
