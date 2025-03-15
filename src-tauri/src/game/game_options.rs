use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GameOptions {
    number_of_players: u8,
    lines_40: bool,
    blitz: bool,
    normal: bool,
}
impl Default for GameOptions {
    fn default() -> Self {
        Self {
            number_of_players: 1,
            lines_40: false,
            blitz: false,
            normal: true,
        }
    }
}
impl GameOptions {
    pub fn normal(&mut self) {
        self.lines_40 = false;
        self.blitz = false;
        self.normal = true;
    }
    pub fn blitz(&mut self) {
        self.lines_40 = false;
        self.blitz = true;
        self.normal = false;
    }
    pub fn lines_40(&mut self) {
        self.lines_40 = true;
        self.blitz = false;
        self.normal = false;
    }
    pub fn single_player(&mut self) {
        self.number_of_players = 1;
    }
    pub fn multi_player(&mut self, players: u8) {
        if players < 2 {
            panic!("In multiplayer should be more than one player");
        }
        self.number_of_players = players;
    }
    pub fn is_normal(&self) -> bool {
        self.normal
    }
    pub fn is_blitz(&self) -> bool {
        self.blitz
    }
    pub fn is_lines_40(&self) -> bool {
        self.lines_40
    }
    pub fn number_of_players(&self) -> u8 {
        self.number_of_players
    }
}
