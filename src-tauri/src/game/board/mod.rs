use super::queue::Queue;

pub mod cell;
pub mod danger_level;
pub mod local_board;

pub trait Board {
    fn new(queue: impl Queue + 'static) -> Self
    where
        Self: Sized;
    fn game_over(&self) -> bool;
    fn game_won(&self) -> bool;
    fn board_state(&self) -> String;
}
