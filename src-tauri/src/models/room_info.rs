use std::net::IpAddr;

use serde::{Deserialize, Serialize};

use crate::room::Room;

/// `RoomInfo` represents the information about a game room that is shared over the network.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RoomInfo {
    /// The current number of players in the room.
    number_of_players: u8,
    /// The maximum number of players allowed in the room.
    limit_of_players: u8,
    /// The name of the room.
    name: String,
    /// The number of games played in the room.
    games_played: u16,
    /// The IP address of the room's host.
    ip: IpAddr,
}

impl RoomInfo {
    /// Returns the number of players in the room.
    pub fn number_of_players(&self) -> u8 {
        self.number_of_players
    }

    /// Returns the limit of players in the room.
    pub fn limit_of_players(&self) -> u8 {
        self.limit_of_players
    }

    /// Returns the name of the room.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the number of games played in the room.
    pub fn games_played(&self) -> u16 {
        self.games_played
    }

    /// Returns the IP address of the room.
    pub fn ip(&self) -> &IpAddr {
        &self.ip
    }

    /// Changes the number of players in the room.
    ///
    /// # Arguments
    ///
    /// * `players` - The new number of players in the room.
    pub fn change_number_of_players(&mut self, players: u8) {
        self.number_of_players = players;
    }
}

impl From<&Room> for RoomInfo {
    fn from(value: &Room) -> Self {
        RoomInfo {
            number_of_players: value.players().len() as u8,
            limit_of_players: value.limit_of_players() + 1,
            name: value.name().to_string(),
            games_played: value.games_played(),
            ip: local_ip_address::local_ip().unwrap(),
        }
    }
}

#[cfg(dev)]
pub fn generate_random_info(number: u32) -> Vec<RoomInfo> {
    let mut info = vec![];
    for i in 0..25 {
        info.push(RoomInfo {
            number_of_players: 10,
            limit_of_players: 15,
            name: format!("Room {number}:{i}"),
            games_played: i,
            ip: local_ip_address::local_ip().unwrap(),
        });
    }
    info
}
