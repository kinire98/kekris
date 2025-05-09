use std::sync::Arc;
use std::{net::IpAddr, time::Duration};

use local_ip_address::local_ip;
use player::Player;
use serde::{Deserialize, Serialize};
use server::listen_to_broadcast_requests::listen_to_request;
use server::send_receive_room_updates::{listen_to_player_updates, listen_to_room_requests};
use tauri::{AppHandle, Emitter};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::sync::{Mutex, broadcast};
use tokio::time::Instant;

use crate::globals::{PING_IN_MILLIS, UPDATES_IN_MILLIS};
use crate::models::dummy_room::DummyPlayer;

const PLAYERS_EMIT: &str = "playersEmit";

// Can only be open for 4.85 hours
#[derive(Debug)]
pub struct Room {
    local_player: Player,
    players: Vec<Player>,
    visibility: Visibility,
    name: String,
    limit_of_players: u8,
    games_played: u16,
    #[allow(dead_code)]
    ip: IpAddr,
    app: AppHandle,
    close_room: Receiver<bool>,
    receive_commands: Receiver<FirstLevelCommands>,
    send_commands: Sender<FirstLevelCommands>, // Needed to clone for players listening
    send_updates: broadcast::Sender<Updates>,
    game_starting_sender: broadcast::Sender<bool>,
    player_info: Arc<Mutex<u8>>,
}

impl Room {
    pub async fn new(
        name: String,
        app: AppHandle,
        close_room: Receiver<bool>,
        stop_listening_channel: broadcast::Receiver<bool>,
        player_name: String,
    ) -> Self {
        let ip = local_ip().unwrap();
        let (tx_updates, _) = broadcast::channel(32);
        let (tx_commands, rx_commands) = mpsc::channel(32);
        let (tx_started, _) = broadcast::channel(32);
        let players_limit = 15;
        let players_info = Arc::new(Mutex::new(1));
        let info = Self {
            local_player: player_name.into(),
            players: vec![],
            visibility: Visibility::LocalNetwork,
            name,
            limit_of_players: players_limit,
            games_played: 0,
            ip,
            app: app.clone(),
            close_room,
            receive_commands: rx_commands,
            send_commands: tx_commands.clone(),
            send_updates: tx_updates,
            game_starting_sender: tx_started.clone(),
            player_info: players_info.clone(),
        };
        listen_to_request(
            (&info).into(),
            app,
            stop_listening_channel.resubscribe(),
            players_info.clone(),
        );
        listen_to_room_requests(tx_commands, (&info).into(), players_info, players_limit);
        info
    }
    pub async fn room_start(&mut self) {
        self.players_emit();
        let now = Instant::now();
        loop {
            tokio::select! {
                _ = self.close_room.recv() => {
                    self.close_room();
                    break;
                },
                result = self.receive_commands.recv() => {
                    let Some(command) = result else {
                        continue;
                    };
                    // if let FirstLevelCommands::PlayerDisconnected(player) = &command {
                    //     dbg!("here");
                    // }
                    match command {
                        FirstLevelCommands::FatalFail => todo!(),
                        FirstLevelCommands::PlayerConnected(player_info) => {
                            self.players.push(player_info.into());
                            self.players_emit();
                            self.players_update();
                            let info: &Player = self.players.last().expect("Exists");
                            listen_to_player_updates(
                                self.send_commands.clone(),
                                info.stream().expect("Exists"),
                                info.into(),
                                self.game_starting_sender.subscribe(),
                                self.send_updates.subscribe(),
                            );
                            let mut value = self.player_info.lock().await;
                            *value = (self.players.len() + 1) as u8;
                        }
                        FirstLevelCommands::PlayerDisconnected(dummy_player) => {
                            self
                                .players
                                .retain(|player| {
                                    let player: DummyPlayer = player.into();
                                    player != dummy_player
                                });
                            self.players_emit();
                            self.players_update();
                            let mut value = self.player_info.lock().await;
                            *value = (self.players.len() + 1) as u8;
                        },
                        FirstLevelCommands::PingReceived((dummy_player, ping)) => {
                            let mut players = self.players.clone();
                            for player in &mut players {
                                let dummy: DummyPlayer = (&*player).into();
                                if dummy == dummy_player {
                                    player.ping_received(ping);
                                }
                            }
                            self.players = players;
                        },
                    }
                },
                _ = tokio::time::sleep(Duration::from_millis(PING_IN_MILLIS)) => {
                    let result = self.send_updates.send(Updates::SendPing);
                    if result.is_err() {
                        let _ = self.send_updates.send(Updates::SendPing);
                    }
                    self.players_emit();
                }
            }
            tokio::time::sleep(Duration::from_millis(PING_IN_MILLIS)).await;
            if now.elapsed() > Duration::from_millis(UPDATES_IN_MILLIS) {
                self.players_update();
            }
        }
    }

    pub fn players(&self) -> &[Player] {
        &self.players
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn limit_of_players(&self) -> u8 {
        self.limit_of_players
    }

    pub fn games_played(&self) -> u16 {
        self.games_played
    }

    pub fn visibility(&self) -> Visibility {
        self.visibility
    }

    fn players_emit(&self) {
        let mut players: Vec<DummyPlayer> =
            self.players.iter().map(|player| player.into()).collect();
        let local_player = &self.local_player;
        players.push(local_player.into());
        self.app.emit(PLAYERS_EMIT, players).unwrap();
    }
    fn players_update(&self) {
        let mut players = self.players.clone();
        players.push(self.local_player.clone());
        let _ = self.send_updates.send(Updates::PlayersUpdate(players));
    }

    fn updates(&self) {
        self.players_update();
        self.players_emit();
    }
    fn close_room(&mut self) {
        let _ = self.send_updates.send(Updates::RoomEnded).unwrap();
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Default, Debug)]
pub enum Visibility {
    #[default]
    LocalNetwork,
    Internet,
}

#[derive(Debug)]
pub enum FirstLevelCommands {
    FatalFail,
    PlayerConnected((DummyPlayer, TcpStream)),
    PlayerDisconnected(DummyPlayer),
    PingReceived((DummyPlayer, u64)),
}

#[derive(Debug, Clone)]
pub enum Updates {
    PlayersUpdate(Vec<Player>),
    NameChanged(String),
    PlayerLimitChanged(u8),
    RoomEnded,
    SendPing,
}

pub mod client;
pub mod player;
pub mod server;
