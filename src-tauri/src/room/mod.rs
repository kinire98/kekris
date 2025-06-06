use std::sync::Arc;
use std::time::Duration;

use player::Player;
use serde::{Deserialize, Serialize};
use server::listen_to_broadcast_requests::listen_to_request;
use server::listen_to_room_requests::listen_to_room_requests;
use server::room_player_listener::RoomPlayerListener;
use tauri::{AppHandle, Emitter};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::{Mutex, broadcast};
use tokio::time::Instant;

use crate::game::game_types::online_game::OnlineGame;
use crate::game::pieces::Piece;
use crate::game::queue::Queue;
use crate::game::queue::local_queue::LocalQueue;
use crate::globals::{PING_IN_MILLIS, UPDATES_IN_MILLIS};
use crate::models;
use crate::models::dummy_room::DummyPlayer;
use crate::models::game_options::GameOptions;

const PLAYERS_EMIT: &str = "playersEmit";

const PIECES_TO_GENERATE: usize = 10000;

// Can only be open for 4.85 hours
/// `Room` represents a game room where players can connect and play together.
#[derive(Debug)]
pub struct Room {
    /// The local player (host) of the room.
    local_player: Player,
    /// The list of players connected to the room.
    players: Vec<Player>,
    /// The visibility of the room (LocalNetwork or Internet).
    visibility: Visibility,
    /// The name of the room.
    name: String,
    /// The maximum number of players allowed in the room.
    limit_of_players: u8,
    /// The number of games played in the room.
    games_played: u16,
    /// Tauri application handle for emitting events.
    app: AppHandle,
    /// Receiver for closing the room.
    close_room: Receiver<bool>,
    /// Receiver for commands sent to the room.
    receive_commands: Receiver<FirstLevelCommands>,
    /// Sender for sending commands to the room.
    send_commands: Sender<FirstLevelCommands>, // Needed to clone for players listening
    /// Sender for sending updates to all players in the room.
    send_updates: broadcast::Sender<Updates>,
    /// An atomic counter for the number of players in the room.
    player_info: Arc<Mutex<u8>>,
    /// An atomic boolean indicating whether a game is currently being played in the room.
    cur_game_playing: Arc<Mutex<bool>>,
    /// The game options for the room.
    options: GameOptions,
}

impl Room {
    /// Creates a new `Room` instance.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the room.
    /// * `app` - Tauri application handle for emitting events.
    /// * `close_room` - Receiver for closing the room.
    /// * `stop_listening_channel` - Receiver for stopping the broadcast listener.
    /// * `player_name` - The name of the local player.
    /// * `sender_commands` - Sender for sending commands to the room.
    /// * `receiver_commands` - Receiver for receiving commands in the room.
    pub async fn new(
        name: String,
        app: AppHandle,
        close_room: Receiver<bool>,
        stop_listening_channel: broadcast::Receiver<bool>,
        player_name: String,
        sender_commands: Sender<FirstLevelCommands>,
        receiver_commands: Receiver<FirstLevelCommands>,
    ) -> Self {
        let (tx_updates, _) = broadcast::channel(32);
        // let (tx_commands, rx_commands) = mpsc::channel(32);
        let players_limit = 15;
        let players_info = Arc::new(Mutex::new(1));
        let info = Self {
            local_player: player_name.into(),
            players: vec![],
            visibility: Visibility::LocalNetwork,
            name,
            limit_of_players: players_limit,
            games_played: 0,
            app: app.clone(),
            close_room,
            receive_commands: receiver_commands,
            send_commands: sender_commands.clone(),
            send_updates: tx_updates,
            player_info: players_info.clone(),
            cur_game_playing: Arc::new(Mutex::new(false)),
            options: GameOptions::default(),
        };
        listen_to_request(
            (&info).into(),
            app,
            stop_listening_channel.resubscribe(),
            players_info.clone(),
        );
        listen_to_room_requests(sender_commands, (&info).into(), players_info, players_limit);
        info
    }
    /// Starts the room's main loop.
    pub async fn room_start(&mut self) {
        self.players_emit();
        let now = Instant::now();
        loop {
            let lock = self.cur_game_playing.lock().await;
            if !*lock {
                drop(lock);
                tokio::select! {
                    _ = self.close_room.recv() => {
                        self.close_room();
                        break;
                    },
                    result = self.receive_commands.recv() => {
                        let Some(command) = result else {
                            continue;
                        };
                        match command {
                            FirstLevelCommands::FatalFail => todo!(),
                            FirstLevelCommands::PlayerConnected(player_info) => {
                                self.player_connected(player_info).await;
                            }
                            FirstLevelCommands::PlayerDisconnected(dummy_player) => {
                                self.player_disconnected(dummy_player).await;
                            },
                            FirstLevelCommands::PingReceived((dummy_player, ping)) => {
                                self.ping_received(dummy_player, ping).await;
                            },
                            FirstLevelCommands::GameStarts => {
                                self.start_game().await;
                            }
                        }
                    },
                    _ = tokio::time::sleep(Duration::from_millis(PING_IN_MILLIS)) => {}
                }
                let playing = self.cur_game_playing.lock().await;
                let result = self.send_updates.send(Updates::SendPing(*playing));
                if result.is_err() {
                    let _ = self.send_updates.send(Updates::SendPing(*playing));
                }
                self.players_emit();
                tokio::time::sleep(Duration::from_millis(PING_IN_MILLIS)).await;
                if now.elapsed() > Duration::from_millis(UPDATES_IN_MILLIS) {
                    self.players_update();
                }
            } else {
                drop(lock);
                tokio::time::sleep(Duration::from_millis(300)).await;
            }
        }
    }

    /// Returns a slice of the players in the room.
    pub fn players(&self) -> &[Player] {
        &self.players
    }

    /// Returns the name of the room.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the limit of players in the room.
    pub fn limit_of_players(&self) -> u8 {
        self.limit_of_players
    }

    /// Returns the number of games played in the room.
    pub fn games_played(&self) -> u16 {
        self.games_played
    }

    /// Returns the visibility of the room.
    pub fn visibility(&self) -> Visibility {
        self.visibility
    }

    /// Emits the players in the room to the UI.
    fn players_emit(&self) {
        let mut players: Vec<DummyPlayer> =
            self.players.iter().map(|player| player.into()).collect();
        let local_player = &self.local_player;
        players.push(local_player.into());
        self.app.emit(PLAYERS_EMIT, players).unwrap();
    }
    /// Sends a player update to all players in the room.
    fn players_update(&self) {
        let mut players = self.players.clone();
        players.push(self.local_player.clone());
        let _ = self.send_updates.send(Updates::PlayersUpdate(players));
    }
    /// Closes the room.
    fn close_room(&self) {
        let _ = self.send_updates.send(Updates::RoomEnded).unwrap();
    }
    /// Handles a player connecting to the room.
    ///
    /// # Arguments
    ///
    /// * `player_info` - The player's information and TCP stream.
    async fn player_connected(
        &mut self,
        player_info: (
            models::dummy_room::DummyPlayer,
            Arc<Mutex<tokio::net::TcpStream>>,
        ),
    ) {
        self.players.push(player_info.into());
        self.players_emit();
        self.players_update();
        let info: &Player = self.players.last().expect("Exists");
        let stream = info.stream().unwrap();
        let player: DummyPlayer = info.into();
        let commands = self.send_commands.clone();
        let updates = self.send_updates.subscribe();
        let playing = self.cur_game_playing.clone();
        tokio::spawn(async move {
            RoomPlayerListener::new(commands, stream, player, updates, playing)
                .listen_to_player_updates()
                .await;
        });
        let mut value = self.player_info.lock().await;
        *value = (self.players.len() + 1) as u8;
    }
    /// Handles a player disconnecting from the room.
    ///
    /// # Arguments
    ///
    /// * `dummy_player` - The player that disconnected.
    async fn player_disconnected(&mut self, dummy_player: DummyPlayer) {
        self.players.retain(|player| {
            let player: DummyPlayer = player.into();
            player != dummy_player
        });
        self.players_emit();
        self.players_update();
        let mut value = self.player_info.lock().await;
        *value = (self.players.len() + 1) as u8;
    }
    /// Handles a ping received from a player.
    ///
    /// # Arguments
    ///
    /// * `dummy_player` - The player that sent the ping.
    /// * `ping` - The ping time.
    async fn ping_received(&mut self, dummy_player: DummyPlayer, ping: u64) {
        let mut players = self.players.clone();
        for player in &mut players {
            let dummy: DummyPlayer = (&*player).into();
            if dummy == dummy_player {
                player.ping_received(ping);
            }
        }
        self.players = players;
    }
    /// Starts a game in the room.
    async fn start_game(&mut self) {
        let mut highest_ping = 0;
        self.players.iter().for_each(|player| {
            if player.ping() > highest_ping {
                highest_ping = player.ping();
            }
        });
        let mut queue = LocalQueue::default();
        for i in 0..PIECES_TO_GENERATE {
            let _ = queue.get_piece(i);
        }
        let pieces = queue.get_pieces();
        self.options.multi_player((self.players.len() + 1) as u8);
        self.send_updates
            .send(Updates::GameStarts((highest_ping, self.options, pieces)))
            .unwrap();
        tokio::time::sleep(Duration::from_millis(highest_ping)).await;
        let mut online_game = OnlineGame::new(
            self.players.clone(),
            self.cur_game_playing.clone(),
            self.app.clone(),
            highest_ping,
            queue,
            (&self.local_player).into(),
        )
        .await;
        tokio::spawn(async move {
            online_game.start().await;
        });
    }
}

/// `Visibility` represents the visibility of a room.
#[derive(Serialize, Deserialize, Clone, Copy, Default, Debug)]
pub enum Visibility {
    /// The room is only visible on the local network.
    #[default]
    LocalNetwork,
    /// The room is visible on the internet.
    Internet,
}

/// `FirstLevelCommands` represents the commands that can be sent to the room.
#[derive(Debug)]
pub enum FirstLevelCommands {
    /// A fatal error occurred.
    FatalFail,
    /// A player connected to the room.
    PlayerConnected((DummyPlayer, Arc<Mutex<TcpStream>>)),
    /// A player disconnected from the room.
    PlayerDisconnected(DummyPlayer),
    /// A ping was received from a player.
    PingReceived((DummyPlayer, u64)),
    /// The game should start.
    GameStarts,
}

/// `Updates` represents the updates that can be sent to players in the room.
#[derive(Debug, Clone)]
pub enum Updates {
    /// The list of players in the room has been updated.
    PlayersUpdate(Vec<Player>),
    /// The name of the room has changed.
    NameChanged(String),
    /// The player limit of the room has changed.
    PlayerLimitChanged(u8),
    /// The room has ended.
    RoomEnded,
    /// Send a ping request to all players in the room.
    SendPing(bool),
    /// The game is starting.
    GameStarts((u64, GameOptions, Vec<Piece>)),
}

pub mod client;
pub mod player;
pub mod server;
