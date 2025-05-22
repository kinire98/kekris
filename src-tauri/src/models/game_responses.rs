use crate::game::{board::danger_level::DangerLevel, pieces::Piece, strategy::Strategy};

#[derive(Debug)]
pub enum GameResponses {
    BoardState(String),
    DangerLevel(DangerLevel),
    Strategy(Strategy),
    TrashSent(u32),
    Lost,
    Queue(Vec<Piece>),
}
