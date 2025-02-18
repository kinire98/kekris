use std::{
    io::{BufWriter, Write},
    ops::Range,
};

use crate::game::{pieces::Piece, queue::Queue, strategy::Strategy};

use super::{cell::Cell, danger_level::DangerLevel, Board};

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 20;


pub struct LocalBoard {
    queue: Box<dyn Queue>,
    held_piece: Option<Piece>,
    cur_piece: MovingPiece,
    strategy: Strategy,
    piece_num: usize,
    trash_lines_queue: Vec<(u8, u8)>,
    cells: [Cell; BOARD_HEIGHT * BOARD_WIDTH],
    buffer: [Cell; BOARD_HEIGHT * BOARD_WIDTH],
    prev_cells: [Cell; BOARD_HEIGHT * BOARD_WIDTH],
    piece_blocked: bool,
    line_cleared: bool,
    lines_cleared: u32,
    game_over: bool
}
#[derive(Debug, Clone, Copy)]
struct MovingPiece {
    piece: Piece,
    orientation: Orientation,
    x: i8,
    y: i8
}
#[derive(Debug, Clone, Copy)]
enum Orientation {
    North,
    South,
    East,
    West
}

impl Board for LocalBoard {

    fn game_over(&self) -> bool {
        self.game_over || self.top_out() || self.lock_out() || self.block_out()
    }

    fn game_won(&self, win_condition: impl Fn(bool, u32) -> bool) -> bool {
        win_condition(self.game_over(), self.lines_cleared)
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
                    Piece::Ghost => buf.write(b"G"),
                    Piece::Trash => buf.write(b"R"),
                },
            }
            .expect("Should be written correctly into buffer");
        });
        let bytes = buf.into_inner().expect("Should be valid");
        String::from_utf8(bytes).expect("Should be valid UTF as I just wrote it")
    }
}

impl LocalBoard {
    pub fn new(mut queue: impl Queue + 'static) -> Self {
        let cur_piece = queue
            .get_piece(0)
            .expect("Queue must have at least one piece!");
        let cur_piece = MovingPiece { piece: cur_piece, 
            orientation: Orientation::North,
            x: 4,
            y: -2 };
        LocalBoard {
            queue: Box::new(queue),
            held_piece: None,
            cur_piece,
            strategy: Strategy::Even,
            piece_num: 0,
            trash_lines_queue: Vec::new(),
            cells: [Cell::Empty; 200],
            buffer: [Cell::Empty; 200],
            prev_cells: [Cell::Empty; 200],
            piece_blocked: false,
            line_cleared: false,
            lines_cleared: 0,
            game_over: false
        }
    }
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
            self.cur_piece = Self::get_moving_piece(self
                .queue
                .get_piece(self.piece_num)
                .expect("Should be pieces"));
        } else {
            self.cur_piece = Self::get_moving_piece(self.held_piece.expect("Already checked"));
        }
        self.piece_blocked = true;
        self.held_piece = Some(cur_piece.piece);
    }

    pub fn change_strategy(&mut self, strategy: Strategy) {
        self.strategy = strategy;
    }

    pub fn held_piece(&self) -> Option<Piece> {
        self.held_piece
    }

    pub fn num_of_trash_lines(&self) -> u8 {
        let mut lines = 0;
        self.trash_lines_queue.iter().for_each(|tup| {
            lines += tup.0;
        });
        lines
    }

    pub fn strategy(&self) -> Strategy {
        self.strategy
    }

    pub fn danger_level(&self) -> DangerLevel {
        let coords = self.get_highest_piece();
        if coords.is_none() {
            return DangerLevel::Empty;
        }

        let coords = coords.unwrap();
        if coords.1 > 0 && coords.1 <= 5 {
            return DangerLevel::VeryLow;
        }

        if coords.1 > 5 && coords.1 <= 7 {
            return DangerLevel::Low;
        }

        if coords.1 > 7 && coords.1 <= 12 {
            return DangerLevel::Medium;
        }
        DangerLevel::AlmostDead
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

    fn top_out(&self) -> bool {
        // ! Probably needs a fix
        let highest_coords = self.get_highest_piece();
        if let Some(coord) = highest_coords {
            (self.num_of_trash_lines() as isize - coord.1 as isize) < 0
        } else {
            false
        }
    }

    fn lock_out(&self) -> bool {
        self.prev_cells == self.cells
    }

    fn block_out(&self) -> bool {
        if self.cur_piece.piece == Piece::O {
            self.get_cell_from_buffer_board(5, -2) == Cell::Empty
        } else {
            self.get_cell_from_buffer_board(4, -2) == Cell::Empty 
        }
    }

    fn get_moving_piece(piece: Piece) -> MovingPiece {
        MovingPiece { piece, orientation: Orientation::North, x: if piece == Piece::O { 5 } else { 4 }, y: -2 }
    }

    fn get_cell_from_main_board(&self, x: usize, y: usize) -> Cell {
        self.cells[y * BOARD_WIDTH + x]
    }

    fn get_cell_from_buffer_board(&self, x: isize, y: isize) -> Cell {
        self.buffer[(BOARD_HEIGHT as isize + y) as usize * BOARD_WIDTH + x as usize]
    }

    fn get_highest_piece(&self) -> Option<(usize, usize)> {
        let mut i = 0;
        for el in self.buffer {
            if el != Cell::Empty {
                return Some((i % BOARD_WIDTH, i / BOARD_WIDTH));
            }
        }
        for el in self.cells {
            if el != Cell::Empty {
                return Some((i % BOARD_WIDTH, i / BOARD_WIDTH));
            }
        }
        None
    }

    fn piece_fixed(&mut self) {
        self.game_over = self.game_over();
        self.prev_cells = self.cells;
        self.cur_piece = Self::get_moving_piece(self.queue.get_piece(self.piece_num).unwrap());
    }
}

#[cfg(test)]
mod tests;
