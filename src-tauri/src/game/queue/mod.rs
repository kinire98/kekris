use std::fmt::Debug;

use super::pieces::Piece;

pub mod local_queue;
pub mod remote_queue;

pub trait Queue: Send + Sync + Debug {
    fn get_piece(&mut self, position: usize) -> Option<Piece>;
    fn insert_pieces(&mut self, pieces: Vec<Piece>);
}
