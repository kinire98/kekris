use std::{net::IpAddr, time::Duration};

use local_ip_address::local_ip;
use player::Player;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::UpdateHookResult;
use tauri::{AppHandle, Emitter};
use tokio::net::UdpSocket;
use tokio::sync::mpsc::Receiver;

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
    close_room: Receiver<bool>,
}

impl Room {
    pub fn new(
        name: String,
        app: AppHandle,
        close_room: Receiver<bool>,
        stop_listening_channel: Receiver<bool>,
    ) -> Self {
        let ip = local_ip().unwrap();
        let info = Self {
            players: vec![],
            visibility: Visibility::LocalNetwork,
            name,
            limit_of_players: 16,
            games_played: 0,
            ip,
            app: app.clone(),
            close_room,
        };
        Self::listen_to_request(info.as_ref().into(), app, stop_listening_channel);
        info
    }
    pub async fn room_start(&mut self) {
        loop {
            if self.close_room.try_recv().is_ok() {
                return;
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
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

    fn listen_to_request(info: RoomInfo, app: AppHandle, mut stop_channel: Receiver<bool>) {
        tokio::spawn(async move {
            let Ok(socket) = UdpSocket::bind(LISTENING_DIRECTION_BROADCAST).await else {
                Self::finish_listening(&app);
                return;
            };
            socket
                .set_broadcast(true)
                .expect("Check for not broadcast allowance");
            let mut data = vec![0; SIZE_FOR_KB];
            loop {
                dbg!(socket.local_addr());
                let Ok((valid_bytes, mut addr)) = socket.recv_from(&mut data).await else {
                    Self::finish_listening(&app);
                    return;
                };
                dbg!(valid_bytes);
                let deserialized: Result<RoomNetCommands, serde_json::Error> =
                    serde_json::from_slice(&data[..valid_bytes]);
                let Ok(net_command) = deserialized else {
                    return;
                };
                addr.set_port(LISTENING_RESPONSE_PORT_BROADCAST);
                dbg!(&net_command);
                dbg!(addr);
                if let RoomNetCommands::RoomDiscover = net_command {
                    let _ = socket
                        .send_to(
                            &serde_json::to_vec(&RoomNetCommands::RoomDiscoverResponse(
                                info.clone(),
                            ))
                            .expect("It's reasonable to expect that this shouldn't panic"),
                            addr,
                        )
                        .await;
                }
                if stop_channel.try_recv().is_ok() {
                    break;
                }
            }
            dbg!("ended_loop");
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

pub mod join_room;
