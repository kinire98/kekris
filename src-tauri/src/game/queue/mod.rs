use super::pieces::Piece;

pub mod local_queue;
pub mod remote_queue;

pub trait Queue {
    fn new() -> impl Queue where Self: Sized;
    fn get_piece(&mut self, position: usize) -> Option<Piece>;
}