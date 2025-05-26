use serde::{Deserialize, Serialize};

/// `GameOptions` represents the options for a game session.
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct GameOptions {
    /// The number of players in the game.
    number_of_players: u8,
    /// A boolean indicating whether the game is in 40-lines mode.
    lines_40: bool,
    /// A boolean indicating whether the game is in blitz mode.
    blitz: bool,
    /// A boolean indicating whether the game is in normal mode.
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
    /// Sets the game to normal mode.
    pub fn normal(&mut self) {
        self.lines_40 = false;
        self.blitz = false;
        self.normal = true;
    }
    /// Sets the game to blitz mode.
    pub fn blitz(&mut self) {
        self.lines_40 = false;
        self.blitz = true;
        self.normal = false;
    }
    /// Sets the game to 40-lines mode.
    pub fn lines_40(&mut self) {
        self.lines_40 = true;
        self.blitz = false;
        self.normal = false;
    }
    /// Sets the game to single player mode.
    pub fn single_player(&mut self) {
        self.number_of_players = 1;
    }
    /// Sets the game to multi player mode.
    ///
    /// # Arguments
    ///
    /// * `players` - The number of players in the game.
    pub fn multi_player(&mut self, players: u8) {
        if players < 1 {
            panic!("In multiplayer should be more than one player");
        }
        self.number_of_players = players;
    }
    /// Returns a boolean indicating whether the game is in normal mode.
    pub fn is_normal(&self) -> bool {
        self.normal
    }
    /// Returns a boolean indicating whether the game is in blitz mode.
    pub fn is_blitz(&self) -> bool {
        self.blitz
    }
    /// Returns a boolean indicating whether the game is in 40-lines mode.
    pub fn is_lines_40(&self) -> bool {
        self.lines_40
    }
    /// Returns the number of players in the game.
    pub fn number_of_players(&self) -> u8 {
        self.number_of_players
    }
}
