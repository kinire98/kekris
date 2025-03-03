use std::{
    io::{BufWriter, Write},
    ops::Range,
};

use moving_piece::{MovingPiece, Orientation};

use crate::game::{pieces::Piece, queue::Queue, strategy::Strategy};

use super::{cell::Cell, danger_level::DangerLevel, Board};

mod moving_piece;

const BOARD_WIDTH: i16 = 10;
const BOARD_HEIGHT: i16 = 20;


pub struct LocalBoard {
    queue: Box<dyn Queue>,
    held_piece: Option<Piece>,
    cur_piece: Box<dyn MovingPiece>,
    strategy: Strategy,
    piece_num: usize,
    trash_lines_queue: Vec<(u8, u8)>,
    cells: [Cell; (BOARD_HEIGHT * BOARD_WIDTH) as usize],
    buffer: [Cell; (BOARD_HEIGHT * BOARD_WIDTH) as usize],
    prev_cells: [Cell; (BOARD_HEIGHT * BOARD_WIDTH) as usize],
    piece_blocked: bool,
    line_cleared: bool,
    lines_cleared: u32,
    game_over: bool
}
impl Board for LocalBoard {

    fn game_over(&self) -> bool {
        self.game_over || self.top_out() || self.lock_out() || self.block_out()
    }

    fn game_won(&self, win_condition: impl Fn(bool, u32) -> bool) -> bool {
        win_condition(self.game_over(), self.lines_cleared)
    }

    fn board_state(&self) -> String {
        let mut buf: Vec<u8> = [Cell::Empty.string_representation() as u8; (BOARD_HEIGHT * BOARD_WIDTH * 2) as usize].into();
        for (i, el) in self.cells.iter().enumerate() {
            buf[i] = el.string_representation() as u8;
        }
        
        for (i, el) in self.cells.iter().enumerate() {
            buf[i + (BOARD_HEIGHT * BOARD_WIDTH) as usize] = el.string_representation() as u8;
        }

        for (x, y) in self.cur_piece.get_coords() {
            if y >= 0 {
                buf[(y * BOARD_WIDTH + x + (BOARD_HEIGHT * BOARD_WIDTH)) as usize] = self.cur_piece.piece().string_representation() as u8;
            } else {
                buf[((BOARD_HEIGHT + y) * BOARD_WIDTH + x) as usize] = self.cur_piece.piece().string_representation() as u8;
            }
        }
        let ghost_piece= self.ghost_piece(self.cur_piece.clone());
        for (x, y) in ghost_piece.get_coords() {
            if y >= 0 {
                buf[(y * BOARD_WIDTH + x + (BOARD_HEIGHT * BOARD_WIDTH)) as usize] = Piece::Ghost.string_representation() as u8;
            } else {
                buf[((BOARD_HEIGHT + y) * BOARD_WIDTH + x) as usize] = Piece::Ghost.string_representation() as u8;
            }
        }
        String::from_utf8(buf).expect("Should be valid UTF as I just wrote it")
    }

    fn strategy(&self) -> Strategy {
        self.strategy
    }
}

impl LocalBoard {
    pub fn new(mut queue: impl Queue + 'static) -> Self {
        let cur_piece = queue
            .get_piece(0)
            .expect("Queue must have at least one piece!");
        LocalBoard {
            queue: Box::new(queue),
            held_piece: None,
            cur_piece: cur_piece.try_into().unwrap(),
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
        let sides = self.cur_piece.get_right_facing_sides();
        for (x, y) in sides {
            if x == BOARD_WIDTH - 1 {
                return;
            }
            if y >= 0 {
                match self.get_cell_from_main_board(x + 1, y) {
                    Cell::Empty => continue,
                    Cell::Full(_) => return,
                }
            } else {
                match self.get_cell_from_buffer_board(x + 1, y) {
                    Cell::Empty => continue,
                    Cell::Full(_) => return,
                }
            }
        }
        self.cur_piece.move_right();
    }

    pub fn move_left(&mut self) {
        let sides = self.cur_piece.get_left_facing_sides();
        for (x, y) in sides {
            if x == 0 {
                return;
            }
            if y >= 0 {
                match self.get_cell_from_main_board(x - 1, y) {
                    Cell::Empty => continue,
                    Cell::Full(_) => return,
                }
            } else {
                match self.get_cell_from_buffer_board(x - 1, y) {
                    Cell::Empty => continue,
                    Cell::Full(_) => return,
                }
            }
        }
        self.cur_piece.move_left();
    }

    pub fn rotation_clockwise(&mut self) {
        let rotation_piece = self.cur_piece.clone();
        self.check_rotation(rotation_piece, 1..3, RotationOption::ClockWise);
    }

    pub fn rotation_counterclockwise(&mut self) {
        let rotation_piece = self.cur_piece.clone();
        self.check_rotation(rotation_piece, 1..3, RotationOption::CounterClockWise);
    }

    pub fn rotation_full(&mut self) {
        let rotation_piece = self.cur_piece.clone();
        self.check_rotation(rotation_piece, 1..3, RotationOption::Full);
    }
    fn check_rotation(&mut self, piece: Box<dyn MovingPiece>, positibility_iteration_range: Range<u8>, option: RotationOption) {
        let mut continue_in_iteration;
        let mut piece = piece;
        for i in positibility_iteration_range {
            println!("{i}");
            continue_in_iteration = false;
            match option {
                RotationOption::ClockWise => piece.rotate_clockwise(i.into()),
                RotationOption::CounterClockWise => piece.rotate_counterclockwise(i.into()),
                RotationOption::Full => piece.rotate_full(i.into()),
            }
            for (x, y) in piece.get_coords() {
                if !(0..=BOARD_WIDTH - 1).contains(&x) { 
                    // Checks that the coordinate is in the board bounds 
                    continue_in_iteration = true;
                    break;
                }
                if !(-BOARD_HEIGHT..BOARD_HEIGHT).contains(&y) {
                    continue_in_iteration = true;
                    break;
                }
                if y >= 0 {
                    match self.get_cell_from_main_board(x, y) {
                        Cell::Empty => continue,
                        Cell::Full(_) => {
                            continue_in_iteration = true;
                            break;
                        }
                    }
                } else {
                    match self.get_cell_from_buffer_board(x, y) {
                        Cell::Empty => continue,
                        Cell::Full(_) => {
                            continue_in_iteration = true;
                            break;
                        }
                    }
                }
            }
            if !continue_in_iteration {
                self.cur_piece = piece.clone();
                break;
            }
        }
    }

    pub fn soft_drop(&mut self) {
        let _ = self.push_down();
    }

    pub fn next_tick(&mut self) {
        let is_in_bottom = self.push_down();
        if !is_in_bottom {
            return;
        }
        self.next_piece_operations();
    }

    fn push_down(&mut self) -> bool {
        let sides = self.cur_piece.get_bottom_facing_sides();
        for (x, y) in sides {
            if y == BOARD_HEIGHT - 1 {
                return true;
            }

            if y + 1 >= 0 {
                match self.get_cell_from_main_board(x, y + 1) {
                    Cell::Empty => continue,
                    Cell::Full(_) => return true,
                }
            } else {
                match self.get_cell_from_buffer_board(x, y + 1) {
                    Cell::Empty => continue,
                    Cell::Full(_) => return true,
                }
            }
        }
        self.cur_piece.move_down();
        false
    }

    fn ghost_piece(&self, mut piece: Box<dyn MovingPiece>) -> Box<dyn MovingPiece> {
        let mut bottom_reached = false;
        while !bottom_reached {
            piece.move_down();
            for (x, y) in piece.get_bottom_facing_sides() {
                if y == BOARD_HEIGHT - 1 {
                    bottom_reached = true;
                    break;
                }
                if y + 1 >= 0 {
                    match self.get_cell_from_main_board(x, y + 1) {
                        Cell::Empty => continue,
                        Cell::Full(_) => bottom_reached = true,
                    }
                } else {
                    match self.get_cell_from_buffer_board(x, y + 1) {
                        Cell::Empty => continue,
                        Cell::Full(_) => bottom_reached = true,
                    }
                }
            }
        }
        piece
    }

    pub fn hard_drop(&mut self) {
        while self.push_down() {}
        self.next_piece_operations();
    }

    fn next_piece_operations(&mut self) {
        let coords = self.cur_piece.get_coords();
        let piece = self.cur_piece.piece();
        for (x, y) in coords {
            if y >= 0 {
                self.set_cell_in_main_board(x, y, piece);
            } else {
                self.set_cell_in_buffer_board(x, y, piece);
            }
        }
        self.game_over = self.game_over();
        self.prev_cells = self.cells;
        self.piece_num += 1;
        self.cur_piece = self.queue.get_piece(self.piece_num).unwrap().try_into().unwrap();
        self.piece_blocked = false;
        todo!("clear line opps needed yet");
    }

    pub fn save_piece(&mut self) {
        if self.piece_blocked {
            return;
        }
        let cur_piece = self.cur_piece.clone();
        if self.held_piece.is_none() {
            self.piece_num += 1;
            self.cur_piece = self
                .queue
                .get_piece(self.piece_num)
                .expect("Should be pieces").try_into().unwrap();
        } else {
            self.cur_piece = self.held_piece.expect("Already checked").try_into().unwrap();
        }
        self.piece_blocked = true;
        self.held_piece = Some(cur_piece.piece());
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

        if coords.1 > 12 && coords.1 <= 14 {
            return DangerLevel::High;
        }

        if coords.1 > 14 && coords.1 <= 18 {
            return DangerLevel::VeryHigh;
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
        if self.cur_piece.piece() == Piece::O {
            self.get_cell_from_buffer_board(5, -2) == Cell::Empty
        } else {
            self.get_cell_from_buffer_board(4, -2) == Cell::Empty 
        }
    }


    fn get_cell_from_main_board(&self, x: i16, y: i16) -> Cell {
        self.cells[(y * BOARD_WIDTH + x) as usize]
    }

    fn set_cell_in_main_board(&mut self, x: i16, y: i16, piece: Piece) {
        self.cells[(y * BOARD_WIDTH + x) as usize] = Cell::Full(piece);
    }

    fn get_cell_from_buffer_board(&self, x: i16, y: i16) -> Cell {
        self.buffer[((BOARD_HEIGHT + y /* y is less than 0 */)* BOARD_WIDTH + x) as usize ]
    }

    fn set_cell_in_buffer_board(&mut self, x: i16, y: i16, piece: Piece) {
        self.cells[((BOARD_HEIGHT + y) * BOARD_WIDTH + x) as usize] = Cell::Full(piece);
    }

    fn get_highest_piece(&self) -> Option<(i16, i16)> {
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
    pub fn orientation(&self) -> Orientation {
        self.cur_piece.orientation()
    }

}

enum RotationOption {
    ClockWise,
    CounterClockWise,
    Full
}

#[cfg(test)]
mod tests;
