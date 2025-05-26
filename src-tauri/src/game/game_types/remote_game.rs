use std::sync::Arc;

use tokio::{
    net::TcpStream,
    sync::{
        Mutex,
        mpsc::{Receiver, Sender},
    },
};

use crate::{
    helpers::game_net_helpers::{read_enum_from_client, send_enum_from_server},
    models::{
        dummy_room::DummyPlayer,
        online_game_commands::{
            client::ClientOnlineGameCommands, server::ServerOnlineGameCommands,
        },
        online_remote_game_communication::{
            OnlineToRemoteGameCommunication, RemoteToOnlineGameCommunication,
        },
    },
    room::player::Player,
};

#[derive(Debug)]
pub struct RemoteGame {
    stream: Arc<Mutex<TcpStream>>,
    receiver: Receiver<OnlineToRemoteGameCommunication>,
    sender: Sender<RemoteToOnlineGameCommunication>,
    player: DummyPlayer,
    most_recent_trash_received: Option<DummyPlayer>,
    lost: bool,
    running: bool,
}
impl RemoteGame {
    pub fn new(
        player: &Player,
        receiver: Receiver<OnlineToRemoteGameCommunication>,
        sender: Sender<RemoteToOnlineGameCommunication>,
    ) -> Self {
        Self {
            stream: player.stream().unwrap(),
            sender,
            receiver,
            player: player.into(),
            most_recent_trash_received: None,
            lost: false,
            running: true,
        }
    }

    pub async fn start_game(&mut self) {
        let lock = self.stream.clone();
        while self.running {
            if !self.lost {
                tokio::select! {
                    value = self.receiver.recv() => {
                        let Some(command) = value else {
                            continue;
                        };
                        self.handle_command(command).await;
                    }
                    result = read_enum_from_client(&lock) => {
                        let Ok(content) = result else {
                            continue;
                        };
                        self.handle_network(content).await;
                    }
                }
            } else {
                let value = self.receiver.recv().await;
                let Some(command) = value else {
                    continue;
                };
                self.handle_command(command).await;
            }
        }
    }
    async fn handle_command(&mut self, command: OnlineToRemoteGameCommunication) {
        let command: Option<ServerOnlineGameCommands> = match command {
            OnlineToRemoteGameCommunication::TrashReceived(dummy_player, amount) => {
                self.most_recent_trash_received = Some(dummy_player);
                Some(ServerOnlineGameCommands::TrashSent(amount))
            }
            OnlineToRemoteGameCommunication::Queue(pieces) => {
                Some(ServerOnlineGameCommands::Queue(pieces))
            }

            OnlineToRemoteGameCommunication::MostRecentReceivedPlayerRequest => {
                let _ = self
                    .sender
                    .send(RemoteToOnlineGameCommunication::HighestReceivedPlayer(
                        self.player.clone(),
                        self.most_recent_trash_received.clone(),
                    ))
                    .await;
                None
            }
            OnlineToRemoteGameCommunication::Won => Some(ServerOnlineGameCommands::Won(0)),
            OnlineToRemoteGameCommunication::PlayerLost(dummy_player) => {
                if self.lost {
                    None
                } else {
                    Some(ServerOnlineGameCommands::PlayerLost(dummy_player))
                }
            }
            OnlineToRemoteGameCommunication::GameEnded(dummy_player) => {
                dbg!("here");

                // None
                send_enum_from_server(
                    &self.stream,
                    &ServerOnlineGameCommands::GameEnded(dummy_player.clone()),
                )
                .await
                .unwrap();
                send_enum_from_server(
                    &self.stream,
                    &ServerOnlineGameCommands::GameEnded(dummy_player),
                )
                .await
                .unwrap();
                None
            }

            OnlineToRemoteGameCommunication::State(dummy_player, state) => {
                if dummy_player == self.player {
                    None
                } else {
                    Some(ServerOnlineGameCommands::State(dummy_player, state))
                }
            }
        };
        let Some(command) = command else {
            return;
        };
        send_enum_from_server(&self.stream, &command).await.unwrap();
    }
    async fn handle_network(&mut self, command: ClientOnlineGameCommands) {
        let message = match command {
            ClientOnlineGameCommands::TrashSent(strategy, amount) => Some(
                RemoteToOnlineGameCommunication::TrashSent(self.player.clone(), strategy, amount),
            ),
            ClientOnlineGameCommands::BoardState(state) => Some(
                RemoteToOnlineGameCommunication::BoardState(self.player.clone(), state),
            ),
            ClientOnlineGameCommands::DangerLevel(danger_level) => Some(
                RemoteToOnlineGameCommunication::DangerLevel(self.player.clone(), danger_level),
            ),
            ClientOnlineGameCommands::Lost(_) => {
                dbg!("here");
                if self.lost {
                    None
                } else {
                    self.lost = true;
                    Some(RemoteToOnlineGameCommunication::Lost(self.player.clone()))
                }
            }
            ClientOnlineGameCommands::QueueRequest(_) => {
                Some(RemoteToOnlineGameCommunication::QueueRequest)
            }
        };
        let Some(message) = message else {
            return;
        };
        self.sender.send(message).await.unwrap();
    }
}
