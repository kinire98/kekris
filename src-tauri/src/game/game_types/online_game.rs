use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
    time::Duration,
};

use futures_util::StreamExt;
use rand::{Rng, seq::IteratorRandom};
use tauri::{AppHandle, Emitter};
use tokio::sync::{
    Mutex,
    mpsc::{self, Receiver, Sender},
};
use tokio_stream::{self as stream};

use crate::{
    commands::game_commands::{FIRST_LEVEL_CHANNEL, GAME_CONTROL_CHANNEL, SECOND_LEVEL_CHANNEL},
    game::{pieces::Piece, strategy::Strategy},
    globals::SIZE_FOR_KB,
    models::{
        dummy_room::DummyPlayer,
        game_commands::{FirstLevelCommands, SecondLevelCommands},
        game_options::GameOptions,
        game_responses::GameResponses,
        online_remote_game_communication::{
            OnlineToRemoteGameCommunication, RemoteToOnlineGameCommunication,
        },
        other_player_state::OtherPlayerState,
        won_signal::WonSignal,
    },
    room::player::Player,
};

use super::{
    super::queue::local_queue::LocalQueue,
    danger_tracker::DangerTracker,
    local_game::{GameControl, LocalGame},
    remote_game::RemoteGame,
};

const STATE_EMIT_OTHER_PLAYERS: &str = "stateEmitForOtherPlayers";
const OTHER_PLAYER_LOST: &str = "otherPlayerLostEmit";
const OTHER_PLAYER_WON: &str = "otherPlayerWon";

const GAME_STARTED_EMIT: &str = "gameStartedEmit";

/// `OnlineGame` manages a multiplayer online game session.
///
/// It handles communication between the local game, remote players, and the UI.
pub struct OnlineGame {
    /// The list of players in the game.
    players: Vec<Player>,
    /// An atomic boolean indicating whether the game is currently being played.
    playing: Arc<Mutex<bool>>,
    /// A map of remote players to their command senders.
    remote_games: HashMap<DummyPlayer, Sender<OnlineToRemoteGameCommunication>>,
    /// Sender for second-level commands to the local game.
    tx_commands_second: Sender<SecondLevelCommands>,
    /// Receiver for commands from remote games.
    rx_remote_commands: Receiver<RemoteToOnlineGameCommunication>,
    /// Receiver for game responses from the local game.
    game_responses: Receiver<GameResponses>,
    /// A boolean indicating whether the game is running.
    game_runnning: bool,
    /// The local player's information.
    self_player: DummyPlayer,
    /// The local player's strategy.
    self_player_strategy: Strategy,
    /// A map of players to the number of lines they are waiting to payback.
    waiting_for_payback_lines: HashMap<DummyPlayer, u32>,
    /// Tracks the danger levels of all players in the game.
    danger_levels: DangerTracker,
    /// Tauri application handle for emitting events.
    app: AppHandle,
    /// A set of players who have lost the game.
    players_lost: HashSet<DummyPlayer>,
    /// A map of players to the number of even lines they have.
    even_lines: HashMap<DummyPlayer, u32>,
    /// A boolean indicating whether the local player has lost.
    self_lost: bool,
}
impl OnlineGame {
    /// Creates a new `OnlineGame` instance.
    ///
    /// # Arguments
    ///
    /// * `players` - The list of players in the game.
    /// * `playing` - An atomic boolean indicating whether the game is currently being played.
    /// * `app` - Tauri application handle for emitting events.
    /// * `delay` - A delay in milliseconds before the game starts.
    /// * `queue` - The queue implementation to use.
    /// * `local_player` - The local player's information.
    pub async fn new(
        players: Vec<Player>,
        playing: Arc<Mutex<bool>>,
        app: AppHandle,
        delay: u64,
        queue: LocalQueue,
        local_player: DummyPlayer,
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
            app.clone(),
            rx_commands,
            Some(rx_commands_second),
            rx_control,
            Some(tx_responses),
            queue,
        );
        let mut remote_games: HashMap<DummyPlayer, Sender<OnlineToRemoteGameCommunication>> =
            HashMap::new();
        let (tx_remote_to_online, rx_remote_to_online) = mpsc::channel(SIZE_FOR_KB);
        let mut even_lines = HashMap::new();
        players.iter().for_each(|player| {
            if player.stream().is_none() {
                return;
            }
            let (tx_online_to_remote, rx_online_to_remote) = mpsc::channel(SIZE_FOR_KB);
            let mut game =
                RemoteGame::new(player, rx_online_to_remote, tx_remote_to_online.clone());
            let dummy: DummyPlayer = player.into();
            remote_games.insert(dummy.clone(), tx_online_to_remote);
            even_lines.insert(dummy, 0);
            tokio::spawn(async move {
                game.start_game().await;
            });
        });
        even_lines.insert(local_player.clone(), 0);
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(delay)).await;
            local_game.start_game().await;
        });
        let dummys: Vec<DummyPlayer> = players
            .iter()
            .filter(|player| player.stream().is_some())
            .map(|player| player.into())
            .collect();
        OnlineGame {
            players,
            playing,
            remote_games,
            tx_commands_second,
            rx_remote_commands: rx_remote_to_online,
            game_responses: rx_responses,
            game_runnning: true,
            self_player: local_player,
            self_player_strategy: Strategy::Random,
            waiting_for_payback_lines: HashMap::new(),
            danger_levels: DangerTracker::new(dummys),
            app,
            players_lost: HashSet::new(),
            even_lines,
            self_lost: false,
        }
    }
    /// Sets up the channels for communication between different parts of the game.
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
    /// Starts the online game.
    pub async fn start(&mut self) {
        self.app
            .emit(GAME_STARTED_EMIT, self.self_player.id())
            .unwrap();
        let mut value = self.playing.lock().await;
        *value = true;
        drop(value);
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
    /// Handles game responses from the local game.
    ///
    /// # Arguments
    ///
    /// * `response` - The game response to handle.
    async fn handle_game_responses(&mut self, response: GameResponses) {
        match response {
            GameResponses::BoardState(state) => {
                self.send_state(self.self_player.clone(), state).await;
            }
            GameResponses::DangerLevel(danger_level) => {
                self.danger_levels
                    .insert(self.self_player.clone(), danger_level);
            }
            GameResponses::Strategy(strategy) => {
                self.self_player_strategy = strategy;
            }
            GameResponses::TrashSent(lines) => {
                self.trash_received(self.self_player.clone(), self.self_player_strategy, lines)
                    .await;
            }
            GameResponses::Lost => {
                self.self_lost = true;
                self.lost(self.self_player.clone()).await;
                self.lost_checks(self.self_player.clone()).await;
            }
            GameResponses::Queue(pieces) => {
                self.queue_emit(pieces).await;
            }
        }
    }
    /// Handles commands from remote games.
    ///
    /// # Arguments
    ///
    /// * `command` - The remote game command to handle.
    async fn handle_commands(&mut self, command: RemoteToOnlineGameCommunication) {
        match command {
            RemoteToOnlineGameCommunication::TrashSent(dummy_player, strategy, received) => {
                self.trash_received(dummy_player, strategy, received).await;
            }
            RemoteToOnlineGameCommunication::BoardState(dummy_player, state) => {
                self.other_player_state_emit(dummy_player.clone(), state.clone());
                self.send_state(dummy_player, state).await;
            }
            RemoteToOnlineGameCommunication::DangerLevel(dummy_player, danger_level) => {
                self.danger_levels.insert(dummy_player, danger_level);
            }
            RemoteToOnlineGameCommunication::HighestReceivedPlayer(dummy_player, dummy_player1) => {
                self.most_recent_received_from_player(dummy_player, dummy_player1)
                    .await;
            }
            RemoteToOnlineGameCommunication::Lost(dummy_player) => {
                self.other_player_lost(dummy_player.clone());
                self.lost(dummy_player.clone()).await;
                self.lost_checks(dummy_player).await;
            }
            RemoteToOnlineGameCommunication::QueueRequest => {
                let _ = self
                    .tx_commands_second
                    .send(SecondLevelCommands::AskForQueue)
                    .await;
            }
        }
    }
    /// Sends the board state to all remote games.
    ///
    /// # Arguments
    ///
    /// * `player` - The player who's board state is being sent.
    /// * `state` - The board state to send.
    async fn send_state(&mut self, player: DummyPlayer, state: String) {
        stream::iter(self.remote_games.values().cloned())
            .for_each_concurrent(self.players.len(), |tx| {
                let value = state.clone();
                let player = player.clone();
                async move {
                    let _ = tx
                        .send(OnlineToRemoteGameCommunication::State(player, value))
                        .await;
                }
            })
            .await;
    }
    /// Notifies all remote games that a player has lost.
    ///
    /// # Arguments
    ///
    /// * `player` - The player who lost.
    async fn lost(&mut self, player: DummyPlayer) {
        stream::iter(self.remote_games.values().cloned())
            .for_each_concurrent(self.players.len(), |tx| {
                let player = player.clone();
                async move {
                    let _ = tx
                        .send(OnlineToRemoteGameCommunication::PlayerLost(player))
                        .await;
                }
            })
            .await;
    }
    /// Sends the piece queue to all remote games.
    ///
    /// # Arguments
    ///
    /// * `pieces` - The piece queue to send.
    async fn queue_emit(&mut self, pieces: Vec<Piece>) {
        stream::iter(self.remote_games.values().cloned())
            .for_each_concurrent(self.players.len(), |tx| {
                let value = pieces.clone();
                async move {
                    let _ = tx.send(OnlineToRemoteGameCommunication::Queue(value)).await;
                }
            })
            .await;
    }
    /// Handles trash received from a player.
    ///
    /// # Arguments
    ///
    /// * `dummy_player` - The player who sent the trash.
    /// * `strategy` - The strategy to use for distributing the trash.
    /// * `received` - The amount of trash received.
    async fn trash_received(
        &mut self,
        dummy_player: DummyPlayer,
        strategy: Strategy,
        received: u32,
    ) {
        if self.players.len() == 1 {
            if dummy_player == self.self_player {
                let _ = self
                    .remote_games
                    .iter()
                    .nth(0)
                    .unwrap()
                    .1
                    .send(OnlineToRemoteGameCommunication::TrashReceived(
                        dummy_player,
                        received,
                    ))
                    .await;
            } else {
                let _ = self
                    .tx_commands_second
                    .send(SecondLevelCommands::TrashReceived(received))
                    .await;
            }
            return;
        }
        if self.players_lost.len() == self.players.len() - 1 && !self.self_lost {
            if dummy_player == self.self_player {
                let _ = self
                    .remote_games
                    .get(&self.get_remaining_player())
                    .unwrap()
                    .send(OnlineToRemoteGameCommunication::TrashReceived(
                        dummy_player,
                        received,
                    ))
                    .await;
            } else {
                let _ = self
                    .tx_commands_second
                    .send(SecondLevelCommands::TrashReceived(received))
                    .await;
            }
            return;
        }
        if self.players_lost.len() == self.players.len() - 1 && self.self_lost {
            let remaining = self.get_remaining_remote_players();
            if dummy_player == remaining[0] {
                let _ = self
                    .remote_games
                    .get(&remaining[1])
                    .unwrap()
                    .send(OnlineToRemoteGameCommunication::TrashReceived(
                        dummy_player,
                        received,
                    ))
                    .await;
            } else {
                let _ = self
                    .remote_games
                    .get(&remaining[0])
                    .unwrap()
                    .send(OnlineToRemoteGameCommunication::TrashReceived(
                        dummy_player,
                        received,
                    ))
                    .await;
            }
            return;
        }
        match strategy {
            Strategy::Elimination => self.elimination_lines(dummy_player, received).await,
            Strategy::Even => self.even_lines(dummy_player, received).await,
            Strategy::PayBack => self.payback_lines(dummy_player, received).await,
            Strategy::Random => self.random_lines(dummy_player, received).await,
        }
    }
    /// Gets the remaining player in the game.
    fn get_remaining_player(&self) -> DummyPlayer {
        self.players
            .iter()
            .map(|player| {
                let player: DummyPlayer = player.into();
                player
            })
            .find(|player| !self.players_lost.contains(player))
            .unwrap()
    }
    /// Gets the remaining remote players in the game.
    fn get_remaining_remote_players(&self) -> Vec<DummyPlayer> {
        self.players
            .iter()
            .map(|player| {
                let player: DummyPlayer = player.into();
                player
            })
            .filter(|player| !self.players_lost.contains(player))
            .collect()
    }
    /// Distributes lines using the elimination strategy.
    ///
    /// # Arguments
    ///
    /// * `dummy_player` - The player who sent the lines.
    /// * `received` - The number of lines received.
    async fn elimination_lines(&mut self, dummy_player: DummyPlayer, received: u32) {
        let more_endangered_players = self.danger_levels.get_highest();
        let random_endangered_player = more_endangered_players
            .iter()
            .nth(rand::rng().random_range(0..more_endangered_players.len()))
            .cloned()
            .unwrap();
        if random_endangered_player == dummy_player {
            self.random_lines(dummy_player, received).await;
            return;
        }
        let _ = self
            .remote_games
            .get(&random_endangered_player)
            .unwrap()
            .send(OnlineToRemoteGameCommunication::TrashReceived(
                dummy_player.clone(),
                received,
            ))
            .await;
        self.store_lines(random_endangered_player, received);
    }
    /// Distributes lines using the even strategy.
    ///
    /// # Arguments
    ///
    /// * `dummy_player` - The player who sent the lines.
    /// * `received` - The number of lines received.
    async fn even_lines(&mut self, dummy_player: DummyPlayer, received: u32) {
        let player = self
            .even_lines
            .iter()
            .min_by(|a, b| {
                if a.0 == &dummy_player {
                    1.cmp(&0)
                } else {
                    a.1.cmp(b.1)
                }
            })
            .map(|(k, _v)| k)
            .cloned()
            .unwrap();
        if player == self.self_player {
            let _ = self
                .tx_commands_second
                .send(SecondLevelCommands::TrashReceived(received))
                .await;
        } else {
            let _ = self
                .remote_games
                .get(&player)
                .unwrap()
                .send(OnlineToRemoteGameCommunication::TrashReceived(
                    dummy_player,
                    received,
                ))
                .await;
        }
        self.store_lines(player, received);
    }
    /// Distributes lines using the payback strategy.
    ///
    /// # Arguments
    ///
    /// * `dummy_player` - The player who sent the lines.
    /// * `received` - The number of lines received.
    async fn payback_lines(&mut self, dummy_player: DummyPlayer, received: u32) {
        let lines = self
            .waiting_for_payback_lines
            .get(&dummy_player)
            .unwrap_or(&0);
        self.waiting_for_payback_lines
            .insert(dummy_player.clone(), lines + received);
        let _ = self
            .remote_games
            .get(&dummy_player)
            .unwrap()
            .send(OnlineToRemoteGameCommunication::MostRecentReceivedPlayerRequest)
            .await;
    }
    /// Distributes lines using the random strategy.
    ///
    /// # Arguments
    ///
    /// * `dummy_player` - The player who sent the lines.
    /// * `received` - The number of lines received.
    async fn random_lines(&mut self, dummy_player: DummyPlayer, received: u32) {
        let player_receiving = {
            let mut range = rand::rng().random_range(0..self.remote_games.len() + 1);
            if range == self.remote_games.len() && dummy_player != self.self_player {
                let _ = self
                    .tx_commands_second
                    .send(SecondLevelCommands::TrashReceived(received))
                    .await;
                self.self_player.clone()
            } else {
                if range == self.remote_games.len() {
                    range -= 1;
                }
                let mut sender = self.remote_games.iter().nth(range).unwrap();
                while sender.0 == &dummy_player {
                    sender = self
                        .remote_games
                        .iter()
                        .nth(rand::rng().random_range(0..self.remote_games.len()))
                        .unwrap();
                }
                let player_receiving = sender.0;
                let _ = sender
                    .1
                    .send(OnlineToRemoteGameCommunication::TrashReceived(
                        dummy_player.clone(),
                        received,
                    ))
                    .await;
                player_receiving.clone()
            }
        };

        self.store_lines(player_receiving, received);
    }
    /// Handles the most recent player request.
    ///
    /// # Arguments
    ///
    /// * `dummy_player` - The player who sent the lines.
    /// * `dummy_player1` - The player who is receiving the lines.
    async fn most_recent_received_from_player(
        &mut self,
        dummy_player: DummyPlayer,
        dummy_player1: Option<DummyPlayer>,
    ) {
        let lines = self
            .waiting_for_payback_lines
            .remove(&dummy_player)
            .unwrap();
        let receiving;
        let sender = match dummy_player1 {
            Some(dummy_player) => {
                receiving = dummy_player.clone();
                self.remote_games.get(&dummy_player).unwrap()
            }
            None => {
                let sender = self.remote_games.iter().choose(&mut rand::rng()).unwrap();
                receiving = sender.0.clone();
                sender.1
            }
        };
        let _ = sender
            .send(OnlineToRemoteGameCommunication::TrashReceived(
                dummy_player.clone(),
                lines,
            ))
            .await;
        self.store_lines(receiving, lines);
    }
    /// Emits the other player state.
    ///
    /// # Arguments
    ///
    /// * `dummy_player` - The player who's state is being emitted.
    /// * `state` - The state to emit.
    fn other_player_state_emit(&self, dummy_player: DummyPlayer, state: String) {
        let _ = self.app.emit(
            STATE_EMIT_OTHER_PLAYERS,
            OtherPlayerState {
                player: dummy_player,
                state,
            },
        );
    }
    /// Emits the other player lost event.
    ///
    /// # Arguments
    ///
    /// * `dummy_player` - The player who lost.
    fn other_player_lost(&self, dummy_player: DummyPlayer) {
        let _ = self.app.emit(OTHER_PLAYER_LOST, dummy_player);
    }
    /// Checks if the game is over.
    ///
    /// # Arguments
    ///
    /// * `dummy_player` - The player who lost.
    async fn lost_checks(&mut self, dummy_player: DummyPlayer) {
        self.players_lost.insert(dummy_player);
        if self.players_lost.len() == self.players.len() {
            let winner = self.get_winner();
            if winner != self.self_player {
                self.other_player_won(winner.clone());
            }
            self.send_winner(winner).await;
        }
    }
    /// Emits the other player won event.
    ///
    /// # Arguments
    ///
    /// * `dummy_player` - The player who won.
    fn other_player_won(&self, dummy_player: DummyPlayer) {
        let _ = self.app.emit(
            OTHER_PLAYER_WON,
            WonSignal {
                player: dummy_player,
                is_hosting: true,
            },
        );
    }
    /// Gets the winner of the game.
    fn get_winner(&self) -> DummyPlayer {
        match self
            .players
            .iter()
            .map(|player| {
                let player: DummyPlayer = player.into();
                player
            })
            .find(|player| !self.players_lost.contains(player))
        {
            Some(value) => value,
            None => self.self_player.clone(),
        }
    }
    /// Sends the winner to all remote games.
    ///
    /// # Arguments
    ///
    /// * `winner` - The winner of the game.
    async fn send_winner(&self, winner: DummyPlayer) {
        match self.remote_games.get(&winner) {
            Some(channel) => {
                let _ = channel.send(OnlineToRemoteGameCommunication::Won).await;
            }
            None => {
                let _ = self.tx_commands_second.send(SecondLevelCommands::Won).await;
            }
        };
        stream::iter(self.remote_games.values().cloned())
            .for_each_concurrent(self.players.len(), |tx| {
                let player = winner.clone();
                async move {
                    let _ = tx
                        .send(OnlineToRemoteGameCommunication::GameEnded(player))
                        .await;
                }
            })
            .await;
        let _ = self.app.emit(
            OTHER_PLAYER_WON,
            WonSignal {
                player: winner,
                is_hosting: true,
            },
        );
        *self.playing.lock().await = false;
    }

    /// Stores the lines for a player.
    ///
    /// # Arguments
    ///
    /// * `receiver` - The player who is receiving the lines.
    /// * `lines` - The number of lines received.
    fn store_lines(&mut self, receiver: DummyPlayer, lines: u32) {
        self.even_lines.insert(receiver, lines);
    }
}
