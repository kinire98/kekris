use serde::{Deserialize, Serialize};

use crate::room::Room;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RoomInfo {
    number_of_players: u8,
    limit_of_players: u8,
    name: String,
    games_played: u16,
}

impl RoomInfo {
    pub fn number_of_players(&self) -> u8 {
        self.number_of_players
    }

    pub fn limit_of_players(&self) -> u8 {
        self.limit_of_players
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn games_played(&self) -> u16 {
        self.games_played
    }
}

impl From<Room> for RoomInfo {
    fn from(value: Room) -> Self {
        RoomInfo {
            number_of_players: value.players().len() as u8,
            limit_of_players: value.limit_of_players(),
            name: value.name().to_string(),
            games_played: value.games_played(),
        }
    }
}

#[cfg(dev)]
pub fn generate_random_info(number: u32) -> Vec<RoomInfo> {
    let mut info = vec![];
    for i in 0..5 {
        info.push(RoomInfo {
            number_of_players: 10,
            limit_of_players: 15,
            name: format!("Room {number}:{i}"),
            games_played: i,
        });
    }
    info
}
