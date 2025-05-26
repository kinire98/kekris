use std::{net::IpAddr, sync::Arc};

use tokio::{net::TcpStream, sync::Mutex};

use crate::{game::game_types::remote_game::RemoteGame, models::dummy_room::DummyPlayer};

static mut ID: u16 = 1;
/// `Player` represents a player connected to the game room.
///
/// It stores information about the player, such as their ID, name, IP address,
/// game statistics, and connection status.
#[derive(Debug, Clone)]
pub struct Player {
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
    /// An optional `RemoteGame` instance associated with the player.
    game: Option<Arc<Mutex<RemoteGame>>>,
    /// The ping time of the player.
    ping: u64,
    /// An optional TCP stream for communication with the player.
    stream: Option<Arc<Mutex<TcpStream>>>,
}
impl Player {
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

    /// Returns the ping of the player.
    pub fn ping(&self) -> u64 {
        self.ping
    }

    /// Returns the `RemoteGame` instance associated with the player.
    pub fn game(&self) -> Option<Arc<Mutex<RemoteGame>>> {
        self.game.clone()
    }

    /// Returns the TCP stream for communication with the player.
    pub fn stream(&self) -> Option<Arc<Mutex<TcpStream>>> {
        self.stream.clone()
    }

    /// Updates the ping time of the player.
    ///
    /// # Arguments
    ///
    /// * `ping` - The new ping time.
    pub fn ping_received(&mut self, ping: u64) {
        self.ping = ping;
    }
}

impl From<(DummyPlayer, Arc<Mutex<TcpStream>>)> for Player {
    fn from(value: (DummyPlayer, Arc<Mutex<TcpStream>>)) -> Self {
        let id = unsafe { ID };
        unsafe {
            ID += 1;
        }
        Player {
            id,
            name: value.0.name().to_string(),
            ip: value.0.ip(),
            games_won: value.0.games_won(),
            playing: value.0.playing(),
            last_time: value.0.last_time(),
            game: None,
            ping: value.0.ping(),
            stream: Some(value.1),
        }
    }
}

impl From<String> for Player {
    fn from(value: String) -> Self {
        let id = unsafe { ID };
        unsafe {
            ID += 1;
        }
        Player {
            id,
            name: value,
            ip: local_ip_address::local_ip().unwrap(),
            games_won: 0,
            playing: false,
            last_time: 0,
            game: None,
            ping: 0,
            stream: None,
        }
    }
}
impl AsRef<Player> for Player {
    fn as_ref(&self) -> &Player {
        self
    }
}
