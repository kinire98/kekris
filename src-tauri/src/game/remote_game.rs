use std::{
    collections::{BinaryHeap, HashMap},
    sync::Arc,
};

use tokio::{io::AsyncWriteExt, net::TcpStream, sync::Mutex};

use crate::{
    models::{dummy_room::DummyPlayer, online_game_commands::server::ServerOnlineGameCommands},
    room::player::Player,
};

use super::board::danger_level::DangerLevel;

#[derive(Debug, Clone)]
pub struct RemoteGame {
    danger_level: DangerLevel,
    lines_received: HashMap<DummyPlayer, u32>,
    max_lines: BinaryHeap<u32>,
    reverse_search: HashMap<u32, DummyPlayer>,
    stream: Arc<Mutex<TcpStream>>,
}
impl RemoteGame {
    pub fn new(player: &Player) -> Self {
        Self {
            danger_level: DangerLevel::Empty,
            lines_received: HashMap::new(),
            max_lines: BinaryHeap::new(),
            reverse_search: HashMap::new(),
            stream: player.stream().unwrap(),
        }
    }
    pub fn danger_level(&self) -> DangerLevel {
        self.danger_level
    }
    pub fn lines_from(&mut self, player: DummyPlayer, lines: u32) {
        let stored_lines = *self.lines_received.get(&player).unwrap_or(&0);
        let total = stored_lines + lines;
        self.lines_received.insert(player.clone(), total);
        self.max_lines.push(total);
        self.send_lines_to_player(total);
    }
    pub fn highest_sender(&self) -> Option<DummyPlayer> {
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
}
