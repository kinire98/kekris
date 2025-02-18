use std::{
    io::{BufWriter, Write},
    ops::Range,
};

use crate::game::{pieces::Piece, queue::Queue, strategy::Strategy};

use super::{cell::Cell, danger_level::DangerLevel, Board};

pub struct LocalBoard {
    queue: Box<dyn Queue>,
    held_piece: Option<Piece>,
    cur_piece: Piece,
    strategy: Strategy,
    piece_num: usize,
    trash_lines_queue: Vec<(u8, u8)>,
    cells: [Cell; 200],
    piece_blocked: bool,
    line_cleared: bool,
}

impl Board for LocalBoard {
    fn game_over(&self) -> bool {
        todo!()
    }

    fn game_won(&self) -> bool {
        todo!()
    }

    fn board_state(&self) -> String {
        let mut buf = BufWriter::new(Vec::new());
        self.cells.iter().for_each(|cell| {
            match cell {
                Cell::Empty => buf.write(b"E"),
                Cell::Full(piece) => match piece {
                    Piece::T => buf.write(b"T"),
                    Piece::O => buf.write(b"O"),
                    Piece::I => buf.write(b"I"),
                    Piece::L => buf.write(b"L"),
                    Piece::J => buf.write(b"J"),
                    Piece::S => buf.write(b"S"),
                    Piece::Z => buf.write(b"Z"),
                    Piece::Clear => buf.write(b"C"),
                    Piece::Ghost => buf.write(b"G"),
                    Piece::Trash => buf.write(b"R"),
                },
            }
            .expect("Should be written correctly into buffer");
        });
        let bytes = buf.into_inner().expect("Should be valid");
        String::from_utf8(bytes).expect("Should be valid UTF as I just wrote it")
    }

    fn new(mut queue: impl Queue + 'static) -> Self
    where
        Self: Sized,
    {
        let cur_piece = queue
            .get_piece(0)
            .expect("Queue must have at least one piece!");
        LocalBoard {
            queue: Box::new(queue),
            held_piece: None,
            cur_piece,
            strategy: Strategy::Even,
            piece_num: 0,
            trash_lines_queue: Vec::new(),
            cells: [Cell::Empty; 200],
            piece_blocked: false,
            line_cleared: false,
        }
    }
}

impl LocalBoard {
    pub fn move_right(&mut self) {
        todo!()
    }

    pub fn move_left(&mut self) {
        todo!()
    }

    pub fn rotation_clockwise(&mut self) {
        todo!()
    }

    pub fn rotation_counterclockwise(&mut self) {
        todo!()
    }

    pub fn rotation_full(&mut self) {
        todo!()
    }

    pub fn soft_drop(&mut self) {
        todo!()
    }

    pub fn hard_drop(&mut self) {
        todo!()
    }

    pub fn save_piece(&mut self) {
        if self.piece_blocked {
            return;
        }
        let cur_piece = self.cur_piece;
        if self.held_piece.is_none() {
            self.piece_num += 1;
            self.cur_piece = self
                .queue
                .get_piece(self.piece_num)
                .expect("Should be pieces");
        } else {
            self.cur_piece = self.held_piece.expect("Already checked");
        }
        self.piece_blocked = true;
        self.held_piece = Some(cur_piece);
    }

    pub fn change_strategy(&mut self, strategy: Strategy) {
        self.strategy = strategy;
    }

    pub fn held_piece(&self) -> Option<Piece> {
        self.held_piece
    }

    pub fn num_of_trash_lines(&self) -> u8 {
        todo!()
    }

    pub fn strategy(&self) -> Strategy {
        self.strategy
    }

    pub fn danger_level(&self) -> DangerLevel {
        todo!()
    }

    pub fn insert_trash(&mut self, number_of_trash_received: u8) {
        todo!()
    }

    pub fn get_pieces(&mut self, r: Range<u128>) -> Vec<Piece> {
        let mut pieces = Vec::new();
        for i in r {
            pieces.push(
                self.queue
                    .get_piece(i.try_into().unwrap())
                    .expect("Should be pieces"),
            );
        }
        pieces
    }

    pub fn piece_blocked(&self) -> bool {
        self.piece_blocked
    }

    pub fn line_cleared(&self) -> bool {
        self.line_cleared
    }

    fn next_piece(&mut self) {
        self.piece_num += 1;
        self.piece_blocked = false;
    }
}

#[cfg(test)]
mod tests;
