use crate::game::pieces::Piece;

use tokio::sync::mpsc::{Receiver, Sender};

use super::Queue;

const PIECES_LIMIT: usize = 2000;

#[derive(Debug)]
pub struct RemoteQueue {
    pieces: Vec<Piece>,
    pieces_request: Sender<bool>,
    pieces_received: Receiver<Vec<Piece>>,
}

impl Queue for RemoteQueue {
    fn get_piece(&mut self, position: usize) -> Option<crate::game::pieces::Piece> {
        Some(self.pieces[position])
    }
}
impl RemoteQueue {
    pub fn new(
        initial_pieces: Vec<Piece>,
        pieces_request: Sender<bool>,
        pieces_received: Receiver<Vec<Piece>>,
    ) -> RemoteQueue {
        Self {
            pieces: initial_pieces,
            pieces_request,
            pieces_received,
        }
    }
}
