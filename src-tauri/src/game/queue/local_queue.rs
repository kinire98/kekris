use super::super::pieces::Piece;
use super::Queue;
use std::collections::HashSet;

#[allow(dead_code)]
const PIECES_SHOWN: usize = 5; // Used in the module, but marked as not used for an unknown reason

#[allow(dead_code)]
const PIECES_GENERATED_BY_CYCLE: usize = 7; // Used in the module, but marked as not used for an unknown reason

#[derive(Default, Debug)]
pub struct LocalQueue {
    pieces: Vec<Piece>,
    max_piece: usize,
}

impl LocalQueue {
    fn generate_new_pieces(&mut self) {
        let mut generated_pieces = HashSet::new();
        while generated_pieces.len() < PIECES_GENERATED_BY_CYCLE {
            let piece_num = rand::random_range(0..=6);
            let piece = piece_num.into();
            if generated_pieces.contains(&piece) {
                continue;
            }
            generated_pieces.insert(piece);
        }
        self.pieces.append(&mut Vec::from_iter(generated_pieces));
        self.max_piece += PIECES_GENERATED_BY_CYCLE;
    }
    pub fn get_pieces(&self) -> Vec<Piece> {
        self.pieces.clone()
    }
}
impl Queue for LocalQueue {
    fn get_piece(&mut self, position: usize) -> Option<Piece> {
        if self.max_piece < position {
            self.max_piece = position;
        }
        if self.max_piece - position < PIECES_SHOWN {
            self.generate_new_pieces();
        }
        self.pieces.get(position).copied()
    }

    fn insert_pieces(&mut self, _pieces: Vec<Piece>) {
        panic!("SHOULD NEVER ARRIVE HERE")
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
        let mut queue = LocalQueue::default();
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
        let mut queue = LocalQueue::default();
        let number_of_pieces = rand::random_range(1..500);
        for i in 0..number_of_pieces {
            assert!(queue.get_piece(i).is_some());
        }
    }

    #[test]
    fn generate_undetermined_number_of_pieces_and_check_unique() {
        initialize();
        let mut queue = LocalQueue::default();
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
