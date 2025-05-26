use std::fmt::Debug;

use danger_level::DangerLevel;

use super::strategy::Strategy;

pub mod cell;
pub mod danger_level;
pub mod local_board;

/// `Board` trait defines the common behavior for different board implementations.
pub trait Board: Debug + Send + Sync {
    /// Checks if the game is over. Assume it's only be called when a piece is set
    fn game_over(&self) -> bool;
    /// Checks if the win condition has been met. The win condition receives if the game has been meet one of the game over conditions
    /// or to check the number of lines set
    fn game_won(&self, win_condition: impl Fn(bool, u32) -> bool) -> bool;
    /// Returns the current state of the board as a string.
    fn board_state(&self) -> String;
    /// Returns the strategy being used by the board.
    fn strategy(&self) -> Strategy;
    /// Returns the danger level of the board.
    fn danger_level(&self) -> DangerLevel;
}
