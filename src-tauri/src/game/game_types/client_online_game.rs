use std::{sync::Arc, time::Duration};

use tauri::{AppHandle, Emitter};
use tokio::sync::Mutex;
use tokio::{
    net::TcpStream,
    sync::mpsc::{self, Receiver, Sender},
};

use crate::{
    commands::game_commands::{FIRST_LEVEL_CHANNEL, GAME_CONTROL_CHANNEL, SECOND_LEVEL_CHANNEL},
    game::{pieces::Piece, queue::remote_queue::RemoteQueue, strategy::Strategy},
    globals::SIZE_FOR_KB,
    helpers::game_net_helpers::{read_enum_from_server, send_enum_from_client},
    models::{
        game_commands::{FirstLevelCommands, SecondLevelCommands},
        game_options::GameOptions,
        game_responses::GameResponses,
        online_game_commands::{
            client::ClientOnlineGameCommands, server::ServerOnlineGameCommands,
        },
        other_player_state::OtherPlayerState,
        won_signal::WonSignal,
    },
};

const STATE_EMIT_OTHER_PLAYERS: &str = "stateEmitForOtherPlayers";
const OTHER_PLAYER_LOST: &str = "otherPlayerLostEmit";
const OTHER_PLAYER_WON: &str = "otherPlayerWon";
const OTHER_PLAYER_WON_UNKNOWN: &str = "otherPlayerWonUnknown";

use super::local_game::{GameControl, LocalGame};

pub struct ClientOnlineGame {
    socket: Arc<tokio::sync::Mutex<TcpStream>>,
    running: bool,
    game_responses: Receiver<GameResponses>,
    tx_commands_second: Sender<SecondLevelCommands>,
    app: AppHandle,
    strategy: Strategy,
    playing: Arc<Mutex<bool>>,
    received_first_game_command: bool,
}

impl ClientOnlineGame {
    pub async fn new(
        socket: Arc<tokio::sync::Mutex<TcpStream>>,
        pieces_buffer: Vec<Piece>,
        game_options: GameOptions,
        app: AppHandle,
        delay: u64,
        playing: Arc<Mutex<bool>>,
    ) -> Self {
        let (tx_needs_pieces, _) = std::sync::mpsc::channel();
        let queue = RemoteQueue::new(pieces_buffer, tx_needs_pieces);
        let (tx_first_level, rx_first_level) = mpsc::channel(SIZE_FOR_KB);
        let (tx_second_level, rx_second_level) = mpsc::channel(SIZE_FOR_KB);
        let (tx_control, rx_control) = mpsc::channel(SIZE_FOR_KB);
        let (tx_responses, rx_responses) = mpsc::channel(SIZE_FOR_KB);
        let mut local_game = LocalGame::new(
            game_options,
            app.clone(),
            rx_first_level,
            Some(rx_second_level),
            rx_control,
            Some(tx_responses),
            queue,
        );
        Self::set_channels(tx_first_level, tx_second_level.clone(), tx_control).await;
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(delay)).await;
            local_game.start_game().await;
        });
        Self {
            socket,
            running: true,
            tx_commands_second: tx_second_level,
            game_responses: rx_responses,
            app,
            strategy: Strategy::Random,
            playing,
            received_first_game_command: false,
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
                .set(Arc::new(tokio::sync::Mutex::new(tx_commands)))
                .unwrap();
        }
        if let Some(channel) = SECOND_LEVEL_CHANNEL.get() {
            let mut locked = channel.lock().await;
            *locked = tx_commands_second;
        } else {
            SECOND_LEVEL_CHANNEL
                .set(Arc::new(tokio::sync::Mutex::new(tx_commands_second)))
                .unwrap();
        }
        if let Some(channel) = GAME_CONTROL_CHANNEL.get() {
            let mut locked = channel.lock().await;
            *locked = tx_control;
        } else {
            GAME_CONTROL_CHANNEL
                .set(Arc::new(tokio::sync::Mutex::new(tx_control)))
                .unwrap();
        }
    }
    pub async fn start(&mut self) {
        let mut lock = self.playing.lock().await;
        *lock = true;
        drop(lock);
        let mut limit = 50;
        while self.running {
            let socket = self.socket.clone();
            tokio::select! {
                content = read_enum_from_server(&socket) => {
                    limit = 50;
                    if let Ok(content) = content  {
                        self.handle_network_content(content).await;
                        self.received_first_game_command = true;
                    } else {
                        self.handle_error(content.unwrap_err());
                    };
                },
                response = self.game_responses.recv() => {
                    limit = 50;
                    if let Some(response) = response  {
                        self.handle_game_responses(response).await;
                    };
                },
                _ = tokio::time::sleep(Duration::from_millis(100)) => {
                    limit -= 1;
                    if limit == 0 {
                        self.return_to_room_error();
                    }
                }

            }
        }
        let mut lock = self.playing.lock().await;
        *lock = false;
        drop(lock);
    }
    async fn handle_network_content(&mut self, content: ServerOnlineGameCommands) {
        match content {
            ServerOnlineGameCommands::TrashSent(amount) => {
                let _ = self
                    .tx_commands_second
                    .send(SecondLevelCommands::TrashReceived(amount))
                    .await;
            }
            ServerOnlineGameCommands::Queue(pieces) => {
                let result = self
                    .tx_commands_second
                    .send(SecondLevelCommands::QueueSync(pieces.clone()))
                    .await;
                if result.is_err() {
                    let _ = self
                        .tx_commands_second
                        .send(SecondLevelCommands::QueueSync(pieces))
                        .await;
                }
            }
            ServerOnlineGameCommands::Won(_) => {
                dbg!("here");
                let _ = self.tx_commands_second.send(SecondLevelCommands::Won).await;
            }
            ServerOnlineGameCommands::PlayerLost(dummy_player) => {
                let _ = self.app.emit(OTHER_PLAYER_LOST, dummy_player);
            }
            ServerOnlineGameCommands::GameEnded(dummy_player) => {
                dbg!("here");
                let _ = self.app.emit(
                    OTHER_PLAYER_WON,
                    WonSignal {
                        player: dummy_player,
                        is_hosting: false,
                    },
                );
                self.running = false;
            }
            ServerOnlineGameCommands::State(dummy_player, state) => {
                let _ = self.app.emit(
                    STATE_EMIT_OTHER_PLAYERS,
                    OtherPlayerState {
                        player: dummy_player,
                        state,
                    },
                );
            }
        }
    }
    async fn handle_game_responses(&mut self, response: GameResponses) {
        let command: Option<ClientOnlineGameCommands> = match response {
            GameResponses::BoardState(state) => Some(ClientOnlineGameCommands::BoardState(state)),
            GameResponses::DangerLevel(danger_level) => {
                Some(ClientOnlineGameCommands::DangerLevel(danger_level))
            }
            GameResponses::Strategy(strategy) => {
                self.strategy = strategy;
                None
            }
            GameResponses::TrashSent(amount) => {
                Some(ClientOnlineGameCommands::TrashSent(self.strategy, amount))
            }
            GameResponses::Lost => Some(ClientOnlineGameCommands::Lost(0)),
            GameResponses::Queue(_pieces) => {
                panic!("SHOULDN'T BE HERE");
            }
        };
        let Some(command) = command else {
            return;
        };
        send_enum_from_client(&self.socket, &command).await.unwrap();
    }
    fn handle_error(&mut self, error: Box<dyn std::error::Error + Send + Sync>) {
        if let Some(e) = error.downcast_ref::<serde_json::Error>() {
            if e.is_data() && self.received_first_game_command {
                self.return_to_room_error();
            }
        }
    }
    fn return_to_room_error(&mut self) {
        self.running = false;
        let _ = self.app.emit(OTHER_PLAYER_WON_UNKNOWN, false);
    }
}
