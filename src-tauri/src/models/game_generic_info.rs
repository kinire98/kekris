#[derive(Debug, sqlx::FromRow)]
pub struct GameGenericInfo {
    id: u32,
    game_type: String,
    id_game: u32,
}
impl GameGenericInfo {
    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn game_type(&self) -> &str {
        self.game_type.as_ref()
    }
    pub fn game_id(&self) -> u32 {
        self.id_game
    }
}
