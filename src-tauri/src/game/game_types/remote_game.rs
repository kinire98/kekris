use std::{
    collections::{BinaryHeap, HashMap},
    sync::Arc,
};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::{
        Mutex,
        mpsc::{Receiver, Sender},
    },
};

use crate::{
    globals::SIZE_FOR_KB,
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

use super::super::board::danger_level::DangerLevel;

#[derive(Debug)]
pub struct RemoteGame {
    danger_level: DangerLevel,
    lines_received: HashMap<DummyPlayer, u32>,
    max_lines: BinaryHeap<u32>,
    reverse_search: HashMap<u32, DummyPlayer>,
    stream: Arc<Mutex<TcpStream>>,
    receiver: Receiver<OnlineToRemoteGameCommunication>,
    sender: Sender<RemoteToOnlineGameCommunication>,
    buffer: Vec<u8>,
    player: DummyPlayer,
}
impl RemoteGame {
    pub fn new(
        player: &Player,
        receiver: Receiver<OnlineToRemoteGameCommunication>,
        sender: Sender<RemoteToOnlineGameCommunication>,
    ) -> Self {
        Self {
            danger_level: DangerLevel::Empty,
            lines_received: HashMap::new(),
            max_lines: BinaryHeap::new(),
            reverse_search: HashMap::new(),
            stream: player.stream().unwrap(),
            sender,
            receiver,
            buffer: vec![0; SIZE_FOR_KB],
            player: player.into(),
        }
    }
    fn lines_from(&mut self, player: DummyPlayer, lines: u32) {
        let stored_lines = *self.lines_received.get(&player).unwrap_or(&0);
        let total = stored_lines + lines;
        self.lines_received.insert(player.clone(), total);
        self.max_lines.push(total);
        self.reverse_search.insert(total, player);
        self.send_lines_to_player(total);
    }
    fn highest_sender(&self) -> Option<DummyPlayer> {
        let value = self.max_lines.peek()?;
        let value = self.reverse_search.get(value)?;
        Some(value.clone())
    }

    fn send_lines_to_player(&self, lines: u32) {
        let stream = self.stream.clone();
        tokio::spawn(async move {
            let mut socket = stream.lock().await;
            let result = socket
                .write(&serde_json::to_vec(&ServerOnlineGameCommands::TrashSent(lines)).unwrap())
                .await;
            if result.is_ok() {
                return;
            }
            let _ = socket
                .write(&serde_json::to_vec(&ServerOnlineGameCommands::TrashSent(lines)).unwrap())
                .await;
        });
    }
    pub async fn start_game(&mut self) {
        let lock = self.stream.clone();
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
                self.lines_from(dummy_player, amount);
                Some(ServerOnlineGameCommands::TrashSent(amount))
            }
            OnlineToRemoteGameCommunication::Queue(pieces) => {
                Some(ServerOnlineGameCommands::Queue(pieces))
            }
            OnlineToRemoteGameCommunication::DangerLevelRequest => {
                let _ = self
                    .sender
                    .send(RemoteToOnlineGameCommunication::DangerLevelResponse(
                        self.player.clone(),
                        self.danger_level,
                    ))
                    .await;
                None
            }
            OnlineToRemoteGameCommunication::HighestReceivedPlayerRequest => {
                let _ = self
                    .sender
                    .send(RemoteToOnlineGameCommunication::HighestReceivedPlayer(
                        self.player.clone(),
                        self.highest_sender(),
                    ))
                    .await;
                None
            }
            OnlineToRemoteGameCommunication::Won => Some(ServerOnlineGameCommands::Won),
            OnlineToRemoteGameCommunication::PlayerLost(dummy_player) => {
                Some(ServerOnlineGameCommands::PlayerLost(dummy_player))
            }
            OnlineToRemoteGameCommunication::GameEnded(dummy_player) => {
                Some(ServerOnlineGameCommands::GameEnded(dummy_player))
            }
            OnlineToRemoteGameCommunication::State(dummy_player, state) => {
                Some(ServerOnlineGameCommands::State(dummy_player, state))
            }
        };
        let Some(command) = command else {
            return;
        };
        let mut socket = self.stream.lock().await;
        let _ = socket.write(&serde_json::to_vec(&command).unwrap()).await;
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
                self.danger_level = danger_level;
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
