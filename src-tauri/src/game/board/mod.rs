pub mod danger_level;
pub mod cell;
pub mod local_board;

pub trait Board {
    fn game_over() -> bool;
    fn game_won() -> bool;
    fn board_state() -> String;
}