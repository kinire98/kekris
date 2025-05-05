use std::net::IpAddr;

use serde::{Deserialize, Serialize};

use crate::room::{Room, Visibility, player::Player};

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct DummyRoom {
    players: Vec<DummyPlayer>,
    visibility: Visibility,
    name: String,
    limit_of_players: u8,
    games_played: u8,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct DummyPlayer {
    name: String,
    ip: IpAddr,
    games_won: u16,
    playing: bool,
    last_time: u32,
    ping: u32,
}

impl From<&Room> for DummyRoom {
    fn from(value: &Room) -> Self {
        let dummy_players: Vec<DummyPlayer> =
            value.players().iter().map(|player| player.into()).collect();
        DummyRoom {
            players: dummy_players,
            visibility: value.visibility(),
            name: value.name().to_string(),
            limit_of_players: value.limit_of_players(),
            games_played: value.limit_of_players(),
        }
    }
}

impl DummyPlayer {
    pub fn fill(mut player: Self) -> Self {
        player.ip = local_ip_address::local_ip().expect("Reasonable to expect it won't panic");
        player.games_won = 0;
        player.playing = false;
        player.last_time = 0;
        player.ping = 0;
        player
    }

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

impl From<&Player> for DummyPlayer {
    fn from(value: &Player) -> Self {
        DummyPlayer {
            name: value.name().to_string(),
            ip: value.ip(),
            games_won: value.games_won(),
            playing: value.playing(),
            last_time: value.last_time(),
            ping: value.ping(),
        }
    }
}
