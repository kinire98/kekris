use crate::game::pieces::Piece;

use super::Queue;

const PIECES_LIMIT: usize = 2000;

#[derive(Debug)]
pub struct RemoteQueue {
    pieces: Vec<Piece>,
    pieces_request: std::sync::mpsc::Sender<bool>,
}

impl Queue for RemoteQueue {
    fn get_piece(&mut self, position: usize) -> Option<crate::game::pieces::Piece> {
        if position < self.pieces.len() - PIECES_LIMIT {
            let sender = self.pieces_request.clone();
            tokio::task::spawn_blocking(move || {
                let _ = sender.send(true);
            });
        }
        Some(self.pieces[position])
    }

    fn insert_pieces(&mut self, pieces: Vec<Piece>) {
        self.pieces = pieces
    }
}
impl RemoteQueue {
    pub fn new(pieces: Vec<Piece>, pieces_request: std::sync::mpsc::Sender<bool>) -> RemoteQueue {
        Self {
            pieces,
            pieces_request,
        }
    }
}
