use crate::game::{board::danger_level::DangerLevel, strategy::Strategy};

pub enum GameResponses {
    BoardState(String),
    DangerLevel(DangerLevel),
    Strategy(Strategy),
    TrashSent(u32),
    Lost,
}
