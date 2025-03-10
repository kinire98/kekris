use super::strategy::Strategy;

pub mod cell;
pub mod danger_level;
pub mod local_board;

pub trait Board {
    /// Assume it's only be called when a piece is set
    fn game_over(&self) -> bool;
    /// The win condition receives if the game has been meet one of the game over conditions
    /// or to check the number of lines set
    fn game_won(&self, win_condition: impl Fn(bool, u32) -> bool) -> bool;
    fn board_state(&self) -> String;
    fn strategy(&self) -> Strategy;
}
