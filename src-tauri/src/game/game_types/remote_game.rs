use std::{sync::Arc, time::Duration};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::{
        Mutex,
        mpsc::{Receiver, Sender},
    },
};

use crate::{
    globals::{DELAY_FOR_COLISIONS, SIZE_FOR_KB},
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
    buffer: Vec<u8>,
    player: DummyPlayer,
    most_recent_trash_received: Option<DummyPlayer>,
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
            buffer: vec![0; SIZE_FOR_KB],
            player: player.into(),
            most_recent_trash_received: None,
        }
    }

    pub async fn start_game(&mut self) {
        let lock = self.stream.clone();
        tokio::time::sleep(Duration::from_millis(DELAY_FOR_COLISIONS)).await;
        loop {
            let mut socket = lock.lock().await;
            tokio::select! {
                value = self.receiver.recv() => {
                    drop(socket);
                    let Some(command) = value else {
                        continue;
                    };
                    self.handle_command(command).await;
                }
                content = socket.read(&mut self.buffer) => {
                    drop(socket);
                    let Ok(content) = content else {
                        continue;
                    };
                    self.handle_network(content).await;
                }
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
            OnlineToRemoteGameCommunication::Won => Some(ServerOnlineGameCommands::Won),
            OnlineToRemoteGameCommunication::PlayerLost(dummy_player) => {
                if self.player == dummy_player {
                    None
                } else {
                    Some(ServerOnlineGameCommands::PlayerLost(dummy_player))
                }
            }
            OnlineToRemoteGameCommunication::GameEnded(dummy_player) => {
                Some(ServerOnlineGameCommands::GameEnded(dummy_player))
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
        let mut socket = self.stream.lock().await;
        let _ = socket
            .write_all(&serde_json::to_vec(&command).unwrap())
            .await;
    }
    async fn handle_network(&mut self, content: usize) {
        let Ok(command) =
            serde_json::from_slice::<ClientOnlineGameCommands>(&self.buffer[..content])
        else {
            return;
        };
        match command {
            ClientOnlineGameCommands::TrashSent(strategy, amount) => {
                let _ = self
                    .sender
                    .send(RemoteToOnlineGameCommunication::TrashSent(
                        self.player.clone(),
                        strategy,
                        amount,
                    ))
                    .await;
            }
            ClientOnlineGameCommands::BoardState(state) => {
                let _ = self
                    .sender
                    .send(RemoteToOnlineGameCommunication::BoardState(
                        self.player.clone(),
                        state,
                    ))
                    .await;
            }
            ClientOnlineGameCommands::DangerLevel(danger_level) => {
                let _ = self
                    .sender
                    .send(RemoteToOnlineGameCommunication::DangerLevel(
                        self.player.clone(),
                        danger_level,
                    ))
                    .await;
            }
            ClientOnlineGameCommands::Lost => {
                let _ = self
                    .sender
                    .send(RemoteToOnlineGameCommunication::Lost(self.player.clone()))
                    .await;
            }
            ClientOnlineGameCommands::QueueRequest => {
                let _ = self
                    .sender
                    .send(RemoteToOnlineGameCommunication::QueueRequest)
                    .await;
            }
        }
    }
}
