use std::{collections::HashMap, sync::Arc, time::Duration};

use tauri::AppHandle;
use tokio::sync::{
    Mutex,
    mpsc::{self, Sender},
};

use crate::{
    commands::game_commands::{FIRST_LEVEL_CHANNEL, GAME_CONTROL_CHANNEL, SECOND_LEVEL_CHANNEL},
    globals::SIZE_FOR_KB,
    models::{
        dummy_room::DummyPlayer,
        game_commands::{FirstLevelCommands, SecondLevelCommands},
        game_options::GameOptions,
    },
    room::player::Player,
};

use super::{
    local_game::{GameControl, LocalGame},
    queue::local_queue::LocalQueue,
    remote_game::RemoteGame,
};

pub struct OnlineGame {
    players: Vec<Player>,
    playing: Arc<Mutex<bool>>,
    remote_games: HashMap<DummyPlayer, RemoteGame>,
}
impl OnlineGame {
    pub async fn new(
        players: Vec<Player>,
        playing: Arc<Mutex<bool>>,
        app: AppHandle,
        delay: u64,
    ) -> Self {
        let mut options = GameOptions::default();
        options.multi_player(players.len() as u8);
        let (tx_commands, rx_commands) = mpsc::channel(SIZE_FOR_KB);
        let (tx_commands_second, rx_commands_second) = mpsc::channel(SIZE_FOR_KB);
        let (tx_control, rx_control) = mpsc::channel(SIZE_FOR_KB);
        let local_queue = LocalQueue::default();
        Self::set_channels(tx_commands, tx_commands_second, tx_control).await;
        let mut local_game = LocalGame::new(
            options,
            app,
            rx_commands,
            Some(rx_commands_second),
            rx_control,
            local_queue,
        );
        let mut remote_games = HashMap::new();
        players.iter().for_each(|player| {
            remote_games.insert(player.into(), RemoteGame::new(player));
        });
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(delay)).await;
            local_game.start_game().await;
        });
        OnlineGame {
            players,
            playing,
            remote_games,
        }
    }
    async fn set_channels(
        tx_commands: Sender<FirstLevelCommands>,
        tx_commands_second: Sender<SecondLevelCommands>,
        tx_control: Sender<GameControl>,
    ) {
        if let Some(channel) = FIRST_LEVEL_CHANNEL.get() {
            let mut locked = channel.lock().await;
            *locked = tx_commands;
        } else {
            FIRST_LEVEL_CHANNEL
                .set(Arc::new(Mutex::new(tx_commands)))
                .unwrap();
        }
        if let Some(channel) = SECOND_LEVEL_CHANNEL.get() {
            let mut locked = channel.lock().await;
            *locked = tx_commands_second;
        } else {
            SECOND_LEVEL_CHANNEL
                .set(Arc::new(Mutex::new(tx_commands_second)))
                .unwrap();
        }
        if let Some(channel) = GAME_CONTROL_CHANNEL.get() {
            let mut locked = channel.lock().await;
            *locked = tx_control;
        } else {
            GAME_CONTROL_CHANNEL
                .set(Arc::new(Mutex::new(tx_control)))
                .unwrap();
        }
    }
    pub async fn start(&mut self) {
        let mut value = self.playing.lock().await;
        *value = true;
        drop(value);
        let mut i = 0;
        loop {
            if i == 3 {
                break;
            }
            i += 1;
        }
        let mut value = self.playing.lock().await;
        *value = false;
    }
}
