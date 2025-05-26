use std::{sync::Arc, time::Duration};

use tokio::{
    net::TcpStream,
    sync::{
        mpsc::{Receiver, Sender},
        Mutex,
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

/// Represents a remote game instance, handling communication with a remote player.
#[derive(Debug)]
pub struct RemoteGame {
    /// The TCP stream for communicating with the remote player.
    stream: Arc<Mutex<TcpStream>>,
    /// Receiver for commands from the online game.
    receiver: Receiver<OnlineToRemoteGameCommunication>,
    /// Sender for commands to the online game.
    sender: Sender<RemoteToOnlineGameCommunication>,
    /// The player associated with this remote game.
    player: DummyPlayer,
    /// The most recent player who sent trash to this player.
    most_recent_trash_received: Option<DummyPlayer>,
    /// Flag indicating if the player has lost.
    lost: bool,
    /// Flag indicating if the game is running.
    running: bool,
}
impl RemoteGame {
    /// Creates a new `RemoteGame` instance.
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

    /// Starts the remote game, handling incoming commands and network events.
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
                    },
                    _ = tokio::time::sleep(Duration::from_secs(1)) => {}
                }
            } else {
                tokio::select! {
                    value = self.receiver.recv() => {
                        let Some(command) = value else {
                            continue;
                        };
                        self.handle_command(command).await;
                    }
                    _ = tokio::time::sleep(Duration::from_secs(1)) => {}
                }
            }
        }
    }
    /// Handles commands received from the online game.
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
            OnlineToRemoteGameCommunication::Won => {
                self.running = false;
                Some(ServerOnlineGameCommands::Won(0))
            }
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
                for _ in 0..3 {
                    send_enum_from_server(
                        &self.stream,
                        &ServerOnlineGameCommands::GameEnded(dummy_player.clone()),
                    )
                    .await
                    .unwrap();
                }
                self.running = false;
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
    /// Handles network commands received from the remote player.
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
