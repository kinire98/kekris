use std::time::{SystemTime, UNIX_EPOCH};
use std::{sync::Arc, time::Duration};

use tauri::{AppHandle, Emitter};
use tokio::sync::Mutex;
use tokio::{
    net::TcpStream,
    sync::mpsc::{self, Receiver, Sender},
};

use crate::models::dummy_room::DummyPlayer;
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

/// Represents the client's online game instance.
pub struct ClientOnlineGame {
    /// The TCP socket for communication with the server.
    socket: Arc<tokio::sync::Mutex<TcpStream>>,
    /// A flag indicating whether the game is running.
    running: bool,
    /// Receiver for game responses from the local game.
    game_responses: Receiver<GameResponses>,
    /// Sender for second-level commands to the local game.
    tx_commands_second: Sender<SecondLevelCommands>,
    /// Tauri application handle.
    app: AppHandle,
    /// The strategy used by the client.
    strategy: Strategy,
    /// A flag indicating whether the game is currently being played.
    playing: Arc<Mutex<bool>>,
    /// Number of deaths the player has.
    deaths: u8,
    /// The player's own information.
    self_player: DummyPlayer,
    /// A flag indicating whether the player is dead.
    dead: bool,
}

impl ClientOnlineGame {
    /// Creates a new client online game instance.
    pub async fn new(
        socket: Arc<tokio::sync::Mutex<TcpStream>>,
        pieces_buffer: Vec<Piece>,
        game_options: GameOptions,
        app: AppHandle,
        delay: u64,
        playing: Arc<Mutex<bool>>,
        player: DummyPlayer,
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
            deaths: 0,
            self_player: player,
            dead: false,
        }
    }
    /// Sets up the channels for communication between different parts of the game.
    ///
    /// This function retrieves or initializes the global channels used for sending commands
    /// between different parts of the application, specifically for first-level commands,
    /// second-level commands, and game control commands. It uses `Arc<Mutex<T>>` to allow
    /// safe concurrent access to these channels.
    ///
    /// # Arguments
    ///
    /// * `tx_commands` - Sender for first-level commands.
    /// * `tx_commands_second` - Sender for second-level commands.
    /// * `tx_control` - Sender for game control commands.
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
    /// Starts the client online game.
    ///
    /// This function initiates the game loop, handling both network communication with the server
    /// and processing game responses from the local game instance. It manages the game state,
    /// including handling player death, game over conditions, and emitting events to the Tauri
    /// application for UI updates.
    pub async fn start(&mut self) {
        let mut lock = self.playing.lock().await;
        *lock = true;
        drop(lock);
        let mut time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards 🗿🤙")
            .as_secs();
        while self.running {
            let socket = self.socket.clone();
            if !self.dead {
                tokio::select! {
                    content = read_enum_from_server(&socket) => {
                        time = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .expect("Time went backwards 🗿🤙")
                            .as_secs();
                        if let Ok(content) = content  {
                            self.handle_network_content(content).await;
                        }
                    },
                    response = self.game_responses.recv() => {
                        time = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .expect("Time went backwards 🗿🤙")
                            .as_secs();
                        if let Some(response) = response  {
                            self.handle_game_responses(response).await;
                        };
                    },
                    _ = tokio::time::sleep(Duration::from_secs(1)) => {}
                }
            } else {
                tokio::select! {
                    content = read_enum_from_server(&socket) => {
                        time = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .expect("Time went backwards 🗿🤙")
                            .as_secs();
                        if let Ok(content) = content {
                            self.handle_network_content(content).await;
                        }
                    },
                    _ = tokio::time::sleep(Duration::from_secs(1)) => {}
                }
            }
            let cur_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards 🗿🤙")
                .as_secs();
            if cur_time - time >= 7 {
                self.running = false;
                let _ = self.app.emit(OTHER_PLAYER_WON_UNKNOWN, false);
            }
        }
        let mut lock = self.playing.lock().await;
        *lock = false;
        drop(lock);
    }
    /// Handles network content received from the server.
    ///
    /// This function processes commands received from the server, such as sending trash,
    /// synchronizing the piece queue, handling game over events, and updating the state
    /// of other players. It interacts with the local game instance by sending second-level
    /// commands and emits events to the Tauri application for UI updates.
    ///
    /// # Arguments
    ///
    /// * `content` - The server online game command received.
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
                let _ = self.tx_commands_second.send(SecondLevelCommands::Won).await;
                let _ = self.app.emit(
                    OTHER_PLAYER_WON,
                    WonSignal {
                        player: self.self_player.clone(),
                        is_hosting: false,
                    },
                );
                self.running = false;
            }
            ServerOnlineGameCommands::PlayerLost(dummy_player) => {
                self.deaths += 1;
                let _ = self.app.emit(OTHER_PLAYER_LOST, dummy_player);
            }
            ServerOnlineGameCommands::GameEnded(dummy_player) => {
                if dummy_player == self.self_player {
                    let _ = self.tx_commands_second.send(SecondLevelCommands::Won).await;
                }
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
    /// Handles game responses received from the local game.
    ///
    /// This function processes responses from the local game instance, such as board state updates,
    /// danger level updates, strategy updates, and trash sent events. It translates these responses
    /// into client online game commands and sends them to the server. It also handles the player's
    /// death event.
    ///
    /// # Arguments
    ///
    /// * `response` - The game response received from the local game.
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
            GameResponses::Lost => {
                self.dead = true;
                self.deaths += 1;
                send_enum_from_client(&self.socket, &ClientOnlineGameCommands::Lost(0))
                    .await
                    .unwrap();
                send_enum_from_client(&self.socket, &ClientOnlineGameCommands::Lost(0))
                    .await
                    .unwrap();
                None
            }
            GameResponses::Queue(_pieces) => {
                panic!("SHOULDN'T BE HERE");
            }
        };
        let Some(command) = command else {
            return;
        };
        send_enum_from_client(&self.socket, &command).await.unwrap();
    }
}
