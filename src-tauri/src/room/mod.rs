use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{net::IpAddr, time::Duration};

use local_ip_address::local_ip;
use player::Player;
use serde::{Deserialize, Serialize};
use server::listen_to_broadcast_requests::listen_to_request;
use server::send_receive_room_updates::listen_to_room_requests;
use tauri::{AppHandle, Emitter};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::sync::{Mutex, broadcast};

use crate::models::dummy_room::DummyPlayer;

const PLAYERS_EMIT: &str = "playersEmit";

const UPDATES_IN_SECONDS: u64 = 1;
// Can only be open for 4.85 hours
#[derive(Debug)]
pub struct Room {
    local_player: Player,
    players: Vec<Player>,
    visibility: Visibility,
    name: String,
    limit_of_players: u8,
    games_played: u16,
    ip: IpAddr,
    app: AppHandle,
    close_room: Receiver<bool>,
    receive_commands: Receiver<FirstLevelCommands>,
    send_updates: Sender<Updates>,
    player_info: Arc<Mutex<u8>>,
    start_time: u64,
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
        let (tx_updates, rx_updates) = mpsc::channel(32);
        let (tx_commands, rx_commands) = mpsc::channel(32);
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
            send_updates: tx_updates,
            player_info: players_info.clone(),
            start_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards 🗿🤙")
                .as_secs(),
        };
        listen_to_request((&info).into(), app, stop_listening_channel.resubscribe());
        listen_to_room_requests(
            tx_commands,
            rx_updates,
            stop_listening_channel.resubscribe(),
            (&info).into(),
            players_info,
            players_limit,
        );
        info
    }
    pub async fn room_start(&mut self) {
        self.players_emit();
        loop {
            if self.close_room.try_recv().is_ok() {
                return;
            }
            if let Ok(command) = self.receive_commands.try_recv() {
                match command {
                    FirstLevelCommands::FatalFail => todo!(),
                    FirstLevelCommands::PlayerConnected(player_info) => {
                        self.players.push(player_info.into());
                        self.players_emit();
                        self.players_update().await;
                    }
                    FirstLevelCommands::PlayerDisconnected(dummy_player) => todo!(),
                }
            }
            if self.send_updates_time_passed() {
                self.updates().await;
            }
            let mut value = self.player_info.lock().await;
            *value = (self.players.len() + 1) as u8;

            tokio::time::sleep(Duration::from_micros(16_666)).await;
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
    async fn players_update(&self) {
        let _ = self
            .send_updates
            .send(Updates::PlayersUpdate(self.players.clone()))
            .await;
    }
    fn send_updates_time_passed(&self) -> bool {
        let now_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards 🗿🤙")
            .as_secs();
        now_time - self.start_time >= UPDATES_IN_SECONDS
    }
    async fn updates(&self) {
        self.players_update().await;
        self.players_emit();
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Default, Debug)]
pub enum Visibility {
    #[default]
    LocalNetwork,
    Internet,
}

pub enum FirstLevelCommands {
    FatalFail,
    PlayerConnected((DummyPlayer, TcpStream)),
    PlayerDisconnected(DummyPlayer),
}

pub enum Updates {
    PlayersUpdate(Vec<Player>),
    NameChanged(String),
    PlayerLimitChanged(u8),
}

pub mod client;
pub mod player;
pub mod server;
