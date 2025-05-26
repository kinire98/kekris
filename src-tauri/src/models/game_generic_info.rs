/// `GameGenericInfo` represents generic information about a game stored in the database.
#[derive(Debug, sqlx::FromRow)]
pub struct GameGenericInfo {
    /// The unique identifier of the game.
    id: u32,
    /// The type of game (e.g., "Classic", "Lines", "Blitz").
    game_type: String,
    /// The identifier of the specific game information (e.g., `ClassicGameInfo.id`).
    id_game: u32,
}
impl GameGenericInfo {
    /// Returns the ID of the game.
    pub fn id(&self) -> u32 {
        self.id
    }
    /// Returns the type of the game.
    pub fn game_type(&self) -> &str {
        self.game_type.as_ref()
    }
    /// Returns the ID of the specific game information.
    pub fn game_id(&self) -> u32 {
        self.id_game
    }
}
