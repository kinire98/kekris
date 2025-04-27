use std::net::IpAddr;

use local_ip_address::local_ip;
use player::Player;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};
use tokio::net::UdpSocket;

use crate::{
    globals::{LISTENING_DIRECTION_BROADCAST, LISTENING_RESPONSE_PORT_BROADCAST, SIZE_FOR_KB},
    models::{room_commands::RoomNetCommands, room_info::RoomInfo},
};

const ERROR_OPENING_CONNECTION_EMIT: &str = "error_opening_connection";

#[derive(Debug)]
pub struct Room {
    players: Vec<Player>,
    visibility: Visibility,
    name: String,
    limit_of_players: u8,
    games_played: u16,
    ip: IpAddr,
    app: AppHandle,
}

impl Room {
    pub fn new(name: String, app: AppHandle) -> Self {
        let ip = local_ip().unwrap();
        let info = Self {
            players: vec![],
            visibility: Visibility::LocalNetwork,
            name,
            limit_of_players: 16,
            games_played: 0,
            ip,
            app: app.clone(),
        };
        Self::listen_to_request(info.as_ref().into(), app);
        info
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

    fn listen_to_request(info: RoomInfo, app: AppHandle) {
        tokio::spawn(async move {
            let Ok(mut socket) = UdpSocket::bind(LISTENING_DIRECTION_BROADCAST).await else {
                Self::finish_listening(&app);
                return;
            };
            socket
                .set_broadcast(true)
                .expect("Check for not broadcast allowance");
            let mut data = [0; SIZE_FOR_KB];
            loop {
                let Ok((valid_bytes, mut addr)) = socket.recv_from(&mut data).await else {
                    Self::finish_listening(&app);
                    break;
                };
                let deserialized: Result<RoomNetCommands, serde_json::Error> =
                    serde_json::from_slice(&data[..valid_bytes]);
                let Ok(net_command) = deserialized else {
                    continue;
                };
                addr.set_port(LISTENING_RESPONSE_PORT_BROADCAST);
                if let RoomNetCommands::RoomDiscover = net_command {
                    let _ = socket
                        .send_to(
                            &serde_json::to_vec(&info)
                                .expect("It's reasonable to expect that this shouldn't panic"),
                            addr,
                        )
                        .await;
                }
            }
        });
    }

    fn finish_listening(app: &AppHandle) {
        let _ = app.emit(ERROR_OPENING_CONNECTION_EMIT, false);
    }
}
impl AsRef<Room> for Room {
    fn as_ref(&self) -> &Room {
        self
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Default, Debug)]
pub enum Visibility {
    #[default]
    LocalNetwork,
    Internet,
}

pub mod player;

pub mod listen_to_rooms;
