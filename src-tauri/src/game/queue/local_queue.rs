use std::collections::HashSet;
use super::super::pieces::Piece;
use super::Queue;

#[allow(dead_code)]
const PIECES_SHOWN: usize = 5; // Used in the module, but marked as not used for an unknown reason

#[allow(dead_code)]
const PIECES_GENERATED_BY_CYCLE: usize = 7; // Used in the module, but marked as not used for an unknown reason

pub struct LocalQueue {
    pieces: Vec<Piece>,
    max_piece: usize,
}

impl LocalQueue {
    fn generate_new_pieces(&mut self) {
        let mut generated_pieces = HashSet::new();
        while generated_pieces.len() < PIECES_GENERATED_BY_CYCLE {
            let piece_num = rand::random_range(0..=6);
            let piece = Self::get_piece_by_number(piece_num);
            if generated_pieces.contains(&piece) {
                continue;
            }
            generated_pieces.insert(piece);
        }
        self.pieces.append(&mut Vec::from_iter(generated_pieces));
        self.max_piece += PIECES_GENERATED_BY_CYCLE;
    }
    fn get_piece_by_number(i: u8) -> Piece {
        match i {
            0 => Piece::I,
            1 => Piece::J,
            2 => Piece::L,
            3 => Piece::O,
            4 => Piece::S,
            5 => Piece::T,
            _ => Piece::Z
        }
    }
}
impl Queue for LocalQueue {
    fn new() -> LocalQueue {
        let mut queue = LocalQueue { pieces: Vec::new(), max_piece: 0};
        queue.generate_new_pieces();
        queue
    }
    fn get_piece(&mut self, position: usize) -> Option<Piece> {
        if self.max_piece < position {
            self.max_piece = position;
        }
        if self.max_piece - position < PIECES_SHOWN {
            self.generate_new_pieces();
        }
        self.pieces.get(position).copied()
        
    }

}


#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{game::queue::Queue, init_trace::initialize};

    use super::{LocalQueue, PIECES_GENERATED_BY_CYCLE};

    #[test]
    fn get_unique_pieces() {
        initialize();
        let mut queue = LocalQueue::new();
        let mut pieces = HashSet::new();
        for i in 0..PIECES_GENERATED_BY_CYCLE {
            let piece = queue.get_piece(i);
            assert!(!pieces.contains(&piece));
            pieces.insert(piece);
        }
    }

    #[test]
    fn generate_undetermined_number_of_pieces() {
        initialize();
        let mut queue = LocalQueue::new();
        let number_of_pieces = rand::random_range(1..500);
        for i in 0..number_of_pieces {
            assert!(queue.get_piece(i).is_some());
        }
    }

    #[test]
    fn generate_undetermined_number_of_pieces_and_check_unique() {
        initialize();
        let mut queue = LocalQueue::new();
        let number_of_pieces = rand::random_range(1..500);
        let mut unique_pieces = HashSet::new();
        for i in 0..number_of_pieces {
            if i % PIECES_GENERATED_BY_CYCLE == 0 {
                unique_pieces.clear();
            }
            let piece = queue.get_piece(i);
            assert!(!unique_pieces.contains(&piece));
            unique_pieces.insert(piece);
        }
    }
}