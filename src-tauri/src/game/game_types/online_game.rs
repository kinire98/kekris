use std::{collections::HashMap, sync::Arc, time::Duration};

use tauri::AppHandle;
use tokio::sync::{
    Mutex,
    mpsc::{self, Receiver, Sender},
};

use crate::{
    commands::game_commands::{FIRST_LEVEL_CHANNEL, GAME_CONTROL_CHANNEL, SECOND_LEVEL_CHANNEL},
    globals::SIZE_FOR_KB,
    models::{
        dummy_room::DummyPlayer,
        game_commands::{FirstLevelCommands, SecondLevelCommands},
        game_options::GameOptions,
        game_responses::GameResponses,
        online_remote_game_communication::{
            OnlineToRemoteGameCommunication, RemoteToOnlineGameCommunication,
        },
    },
    room::player::Player,
};

use super::{
    super::queue::local_queue::LocalQueue,
    local_game::{GameControl, LocalGame},
    remote_game::RemoteGame,
};

pub struct OnlineGame {
    players: Vec<Player>,
    playing: Arc<Mutex<bool>>,
    remote_games: HashMap<DummyPlayer, Sender<OnlineToRemoteGameCommunication>>,
    tx_commands_second: Sender<SecondLevelCommands>,
    rx_remote_commands: Receiver<RemoteToOnlineGameCommunication>,
    game_responses: Receiver<GameResponses>,
    game_runnning: bool,
}
impl OnlineGame {
    pub async fn new(
        players: Vec<Player>,
        playing: Arc<Mutex<bool>>,
        app: AppHandle,
        delay: u64,
        queue: LocalQueue,
    ) -> Self {
        let mut options = GameOptions::default();
        options.multi_player(players.len() as u8);
        let (tx_commands, rx_commands) = mpsc::channel(SIZE_FOR_KB);
        let (tx_commands_second, rx_commands_second) = mpsc::channel(SIZE_FOR_KB);
        let (tx_control, rx_control) = mpsc::channel(SIZE_FOR_KB);
        let (tx_responses, rx_responses) = mpsc::channel(SIZE_FOR_KB);
        Self::set_channels(tx_commands, tx_commands_second.clone(), tx_control).await;
        let mut local_game = LocalGame::new(
            options,
            app,
            rx_commands,
            Some(rx_commands_second),
            rx_control,
            Some(tx_responses),
            queue,
        );
        let mut remote_games: HashMap<DummyPlayer, Sender<OnlineToRemoteGameCommunication>> =
            HashMap::new();
        let (tx_remote_to_online, rx_remote_to_online) = mpsc::channel(SIZE_FOR_KB);
        players.iter().for_each(|player| {
            let (tx_online_to_remote, rx_online_to_remote) = mpsc::channel(SIZE_FOR_KB);
            let mut game =
                RemoteGame::new(player, rx_online_to_remote, tx_remote_to_online.clone());
            remote_games.insert(player.into(), tx_online_to_remote);
            tokio::spawn(async move {
                game.start_game().await;
            });
        });
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(delay)).await;
            local_game.start_game().await;
        });
        OnlineGame {
            players,
            playing,
            remote_games,
            tx_commands_second,
            rx_remote_commands: rx_remote_to_online,
            game_responses: rx_responses,
            game_runnning: true,
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
        {
            // Block for dropping value
            let mut value = self.playing.lock().await;
            *value = true;
        }
        while self.game_runnning {
            tokio::select! {
                response = self.game_responses.recv() => {
                    let Some(response) = response else {
                        continue;
                    };
                    self.handle_game_responses(response).await;
                },
                command = self.rx_remote_commands.recv() => {
                    let Some(command) = command else {
                        continue;
                    };
                    self.handle_commands(command).await;
                }
            }
        }
        let mut value = self.playing.lock().await;
        *value = false;
    }
    async fn handle_game_responses(&mut self, response: GameResponses) {
        match response {
            GameResponses::BoardState(_) => todo!(),
            GameResponses::DangerLevel(danger_level) => todo!(),
            GameResponses::Strategy(strategy) => todo!(),
            GameResponses::TrashSent(_) => todo!(),
            GameResponses::Lost => todo!(),
        }
    }
    async fn handle_commands(&mut self, command: RemoteToOnlineGameCommunication) {
        match command {
            RemoteToOnlineGameCommunication::TrashSent(dummy_player, strategy, _) => todo!(),
            RemoteToOnlineGameCommunication::BoardStateResponse(dummy_player, _) => todo!(),
            RemoteToOnlineGameCommunication::DangerLevelResponse(dummy_player, danger_level) => {
                todo!()
            }
            RemoteToOnlineGameCommunication::HighestReceivedPlayer(dummy_player, dummy_player1) => {
                todo!()
            }
            RemoteToOnlineGameCommunication::Lost(dummy_player) => todo!(),
            RemoteToOnlineGameCommunication::QueueRequest => todo!(),
        }
    }
}
