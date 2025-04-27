use std::net::IpAddr;

use serde::{Deserialize, Serialize};

use crate::game::remote_game::RemoteGame;

#[derive(Debug)]
pub struct Player {
    name: String,
    ip: IpAddr,
    games_won: u16,
    playing: bool,
    last_time: u32,
    game: Option<RemoteGame>,
    ping: u32,
}
impl Player {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn ip(&self) -> IpAddr {
        self.ip
    }

    pub fn games_won(&self) -> u16 {
        self.games_won
    }

    pub fn playing(&self) -> bool {
        self.playing
    }

    pub fn last_time(&self) -> u32 {
        self.last_time
    }

    pub fn ping(&self) -> u32 {
        self.ping
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            name: Default::default(),
            ip: local_ip_address::local_ip().unwrap(),
            games_won: Default::default(),
            playing: Default::default(),
            last_time: Default::default(),
            game: Default::default(),
            ping: Default::default(),
        }
    }
}
