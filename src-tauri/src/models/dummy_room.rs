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
#[derive(Serialize, Deserialize, Clone, Debug)]
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
