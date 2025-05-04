use std::{net::IpAddr, sync::Arc};

use serde::{Deserialize, Serialize};
use tokio::{net::TcpStream, sync::Mutex};

use crate::{game::remote_game::RemoteGame, models::dummy_room::DummyPlayer};

#[derive(Debug, Clone)]
pub struct Player {
    name: String,
    ip: IpAddr,
    games_won: u16,
    playing: bool,
    last_time: u32,
    game: Option<RemoteGame>,
    ping: u32,
    stream: Option<Arc<Mutex<TcpStream>>>,
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
    pub fn stream(&mut self) -> Option<Arc<Mutex<TcpStream>>> {
        self.stream.clone()
    }
}

impl From<(DummyPlayer, TcpStream)> for Player {
    fn from(value: (DummyPlayer, TcpStream)) -> Self {
        Player {
            name: value.0.name().to_string(),
            ip: value.0.ip(),
            games_won: value.0.games_won(),
            playing: value.0.playing(),
            last_time: value.0.last_time(),
            game: None,
            ping: value.0.ping(),
            stream: Some(Arc::new(Mutex::new(value.1))),
        }
    }
}

impl From<String> for Player {
    fn from(value: String) -> Self {
        Player {
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
