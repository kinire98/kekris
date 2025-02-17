use crate::game::{pieces::Piece, queue::local_queue::LocalQueue, strategy::Strategy};

use super::{danger_level::DangerLevel, Board};


pub struct LocalBoard {
    queue: LocalQueue,
    held_piece: Piece,
    cur_piece: Piece,
    strategy: Strategy,
}


impl Board for LocalBoard {
    fn game_over() -> bool {
        todo!()
    }

    fn game_won() -> bool {
        todo!()
    }

    fn board_state() -> String {
        todo!()
    }
}

impl LocalBoard {
    pub fn move_right() {
        todo!()
    }

    pub fn move_left() {
        todo!()
    }

    pub fn rotation_clockwise() {
        todo!()
    }

    pub fn rotation_counterclockwise() {
        todo!()
    }

    pub fn rotation_full() {
        todo!()
    }

    pub fn soft_drop() {
        todo!()
    }

    pub fn hard_drop() {
        todo!()
    }

    pub fn save_piece() {
        todo!()
    }

    pub fn change_strategy(strategy: Strategy) {
        todo!()
    }
    
    pub fn game_over() -> bool {
        todo!()
    }

    pub fn game_won() {
        todo!()
    }

    pub fn board_state() -> String {
        todo!()
    }

    pub fn held_piece() -> Piece {
        todo!()
    }

    pub fn num_of_trash_lines() -> u8 {
        todo!()
    }

    pub fn strategy() -> Strategy {
        todo!()
    }

    pub fn danger_level() -> DangerLevel {
        todo!()
    }

    pub fn insert_trash(number_of_trash_received: u8) {
        todo!()
    }
}



#[cfg(test)]
mod tests;