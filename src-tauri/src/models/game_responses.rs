use crate::game::{board::danger_level::DangerLevel, pieces::Piece, strategy::Strategy};

/// `GameResponses` represents the responses that the local game can send to the online game.
#[derive(Debug)]
pub enum GameResponses {
    /// Represents the current state of the game board.
    BoardState(String),
    /// Represents the current danger level of the board.
    DangerLevel(DangerLevel),
    /// Represents the current strategy being used by the player.
    Strategy(Strategy),
    /// Indicates that trash lines have been sent to other players.
    TrashSent(u32),
    /// Indicates that the player has lost the game.
    Lost,
    /// Represents the queue of upcoming pieces.
    Queue(Vec<Piece>),
}
