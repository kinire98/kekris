use player::Player;
use serde::{Deserialize, Serialize};

mod player;

#[derive(Serialize, Deserialize)]
pub struct Room {
    players: Vec<Player>,
    visibility: Visibility,
    name: String,
    limit_of_players: u8,
    games_played: u16,
}

impl Room {
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
}

#[derive(Serialize, Deserialize)]
enum Visibility {
    LocalNetwork,
    Internet,
}
