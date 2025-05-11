use std::{
    collections::{BinaryHeap, HashMap},
    sync::Arc,
};

use color_eyre::owo_colors::colors::css::Sienna;
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
                    let Some(command) = value else {
                        continue;
                    };
                    self.handle_command(command).await;
                }
                content = socket.read(&mut self.buffer) => {
                    let Ok(content) = content else {
                        continue;
                    };
                    self.handle_network(content).await;
                }
            }
        }
    }
    async fn handle_command(&mut self, command: OnlineToRemoteGameCommunication) {
        match command {
            OnlineToRemoteGameCommunication::TrashReceived(dummy_player, _) => todo!(),
            OnlineToRemoteGameCommunication::Queue(pieces) => todo!(),
            OnlineToRemoteGameCommunication::BoardStateRequest => todo!(),
            OnlineToRemoteGameCommunication::DangerLevelRequest => todo!(),
            OnlineToRemoteGameCommunication::HighestReceivedPlayerRequest => todo!(),
            OnlineToRemoteGameCommunication::Won => todo!(),
            OnlineToRemoteGameCommunication::PlayerLost(dummy_player) => todo!(),
        }
    }
    async fn handle_network(&mut self, content: usize) {
        let Ok(command) =
            serde_json::from_slice::<ClientOnlineGameCommands>(&self.buffer[..content])
        else {
            return;
        };
        match command {
            ClientOnlineGameCommands::TrashSent(strategy, _) => todo!(),
            ClientOnlineGameCommands::BoardState(_) => todo!(),
            ClientOnlineGameCommands::DangerLevel(danger_level) => todo!(),
            ClientOnlineGameCommands::Lost => todo!(),
            ClientOnlineGameCommands::QueueRequest => todo!(),
        }
    }
}
