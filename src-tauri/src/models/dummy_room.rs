use std::net::IpAddr;

use serde::{Deserialize, Serialize};

use crate::room::{Room, Visibility, player::Player};

/// `DummyRoom` is a simplified representation of a `Room` used for network communication.
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct DummyRoom {
    /// The list of players in the room.
    players: Vec<DummyPlayer>,
    /// The visibility of the room.
    visibility: Visibility,
    /// The name of the room.
    name: String,
    /// The maximum number of players allowed in the room.
    limit_of_players: u8,
    /// The number of games played in the room.
    games_played: u8,
}
/// `DummyPlayer` is a simplified representation of a `Player` used for network communication.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DummyPlayer {
    /// The unique identifier of the player.
    id: u16,
    /// The name of the player.
    name: String,
    /// The IP address of the player.
    ip: IpAddr,
    /// The number of games won by the player.
    games_won: u16,
    /// A boolean indicating whether the player is currently playing.
    playing: bool,
    /// The last time the player was active.
    last_time: u32,
    /// The ping time of the player.
    ping: u64,
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
    /// Fills in default values for a `DummyPlayer`.
    pub fn fill(mut player: Self) -> Self {
        player.ip = local_ip_address::local_ip().expect("Reasonable to expect it won't panic");
        player.games_won = 0;
        player.playing = false;
        player.last_time = 0;
        player.ping = 0;
        player
    }

    /// Returns the ID of the player.
    pub fn id(&self) -> u16 {
        self.id
    }

    /// Returns the name of the player.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the IP address of the player.
    pub fn ip(&self) -> IpAddr {
        self.ip
    }

    /// Returns the number of games won by the player.
    pub fn games_won(&self) -> u16 {
        self.games_won
    }

    /// Returns a boolean indicating whether the player is currently playing.
    pub fn playing(&self) -> bool {
        self.playing
    }

    /// Returns the last time the player was active.
    pub fn last_time(&self) -> u32 {
        self.last_time
    }

    /// Returns the ping time of the player.
    pub fn ping(&self) -> u64 {
        self.ping
    }
}

impl From<&Player> for DummyPlayer {
    fn from(value: &Player) -> Self {
        DummyPlayer {
            id: value.id(),
            name: value.name().to_string(),
            ip: value.ip(),
            games_won: value.games_won(),
            playing: value.playing(),
            last_time: value.last_time(),
            ping: value.ping(),
        }
    }
}
