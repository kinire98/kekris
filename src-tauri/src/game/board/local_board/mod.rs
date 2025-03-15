use std::{cmp::Ordering, ops::Range};

use moving_piece::{MovingPiece, Orientation, moving_piece_t::MovingPieceT};

use crate::game::{pieces::Piece, queue::Queue, strategy::Strategy};

use super::{Board, cell::Cell, danger_level::DangerLevel};

mod moving_piece;

const BOARD_WIDTH: i16 = 10;
const BOARD_HEIGHT: i16 = 20;

#[derive(Debug)]
pub struct LocalBoard {
    queue: Box<dyn Queue>,
    held_piece: Option<Piece>,
    cur_piece: Box<dyn MovingPiece>,
    strategy: Strategy,
    piece_num: usize,
    trash_lines_queue: Vec<(u8, u8)>,
    cells: [Cell; (BOARD_HEIGHT * BOARD_WIDTH) as usize],
    buffer: [Cell; (BOARD_HEIGHT * BOARD_WIDTH) as usize],
    lock_out: bool,
    piece_blocked: bool,
    line_cleared: bool,
    lines_cleared: u32,
    clear_pattern: ClearLinePattern,
    rotation: bool,
    rotation_option: RotationOption,
    rotation_variation: i16,
}
impl Board for LocalBoard {
    fn game_over(&self) -> bool {
        self.top_out() || self.lock_out || self.block_out()
    }

    fn game_won(&self, win_condition: impl Fn(bool, u32) -> bool) -> bool {
        win_condition(self.game_over(), self.lines_cleared)
    }

    fn board_state(&self) -> String {
        let mut buf: Vec<u8> = [Cell::Empty.string_representation() as u8;
            (BOARD_HEIGHT * BOARD_WIDTH * 2) as usize]
            .into();
        let ghost_piece = self.ghost_piece(self.cur_piece.clone());
        for (x, y) in ghost_piece.get_coords() {
            if y >= 0 {
                buf[(y * BOARD_WIDTH + x + (BOARD_HEIGHT * BOARD_WIDTH)) as usize] =
                    Piece::Ghost.string_representation() as u8;
            } else {
                buf[((BOARD_HEIGHT + y) * BOARD_WIDTH + x) as usize] =
                    Piece::Ghost.string_representation() as u8;
            }
        }
        for (i, el) in self.buffer.iter().enumerate() {
            if *el == Cell::Empty {
                continue;
            }
            buf[i] = el.string_representation() as u8;
        }

        for (i, el) in self.cells.iter().enumerate() {
            if *el == Cell::Empty {
                continue;
            }
            buf[i + (BOARD_HEIGHT * BOARD_WIDTH) as usize] = el.string_representation() as u8;
        }

        for (x, y) in self.cur_piece.get_coords() {
            if y >= 0 {
                buf[(y * BOARD_WIDTH + x + (BOARD_HEIGHT * BOARD_WIDTH)) as usize] =
                    self.cur_piece.piece().string_representation() as u8;
            } else {
                buf[((BOARD_HEIGHT + y) * BOARD_WIDTH + x) as usize] =
                    self.cur_piece.piece().string_representation() as u8;
            }
        }

        String::from_utf8(buf).expect("Should be valid UTF as I just wrote it")
    }

    fn strategy(&self) -> Strategy {
        self.strategy
    }
    fn danger_level(&self) -> DangerLevel {
        let coords = self.get_highest_piece();
        if coords.is_none() {
            return DangerLevel::Empty;
        }

        let coords = coords.unwrap();
        println!("{:?}", coords);
        if coords.1 > 14 && coords.1 <= 19 {
            return DangerLevel::VeryLow;
        }

        if coords.1 > 12 && coords.1 <= 14 {
            return DangerLevel::Low;
        }

        if coords.1 > 7 && coords.1 <= 12 {
            return DangerLevel::Medium;
        }

        if coords.1 > 5 && coords.1 <= 7 {
            return DangerLevel::High;
        }

        if coords.1 > 2 && coords.1 <= 5 {
            return DangerLevel::VeryHigh;
        }

        DangerLevel::AlmostDead
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
            lock_out: false,
            piece_blocked: false,
            line_cleared: false,
            lines_cleared: 0,
            clear_pattern: ClearLinePattern::None,
            rotation: false,
            rotation_option: RotationOption::Full,
            rotation_variation: 0,
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
        self.check_rotation(rotation_piece, 1..6, RotationOption::ClockWise);
        self.rotation = true;
    }

    pub fn rotation_counterclockwise(&mut self) {
        let rotation_piece = self.cur_piece.clone();
        self.check_rotation(rotation_piece, 1..6, RotationOption::CounterClockWise);
        self.rotation = true;
    }

    pub fn rotation_full(&mut self) {
        let rotation_piece = self.cur_piece.clone();
        self.check_rotation(rotation_piece, 1..3, RotationOption::Full);
    }
    fn check_rotation(
        &mut self,
        piece: Box<dyn MovingPiece>,
        positibility_iteration_range: Range<u8>,
        option: RotationOption,
    ) {
        let mut continue_in_iteration;
        for i in positibility_iteration_range {
            let mut piece = piece.clone();
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
                if self.rotation {
                    self.rotation_option = option;
                    self.rotation_variation = i as i16;
                }
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

    pub fn hard_drop(&mut self) {
        while !self.push_down() {}
        self.next_piece_operations();
    }

    fn ghost_piece(&self, mut piece: Box<dyn MovingPiece>) -> Box<dyn MovingPiece> {
        let mut bottom_reached = false;
        while !bottom_reached {
            let sides = piece.get_bottom_facing_sides();
            for (x, y) in sides {
                if y >= BOARD_HEIGHT - 1 {
                    bottom_reached = true;
                    break;
                }

                if y + 1 >= 0 {
                    match self.get_cell_from_main_board(x, y + 1) {
                        Cell::Empty => continue,
                        Cell::Full(_) => {
                            bottom_reached = true;
                            break;
                        }
                    }
                } else {
                    match self.get_cell_from_buffer_board(x, y + 1) {
                        Cell::Empty => continue,
                        Cell::Full(_) => {
                            bottom_reached = true;
                            break;
                        }
                    }
                }
            }
            if !bottom_reached {
                piece.move_down();
            }
        }
        piece
    }

    fn push_down(&mut self) -> bool {
        let sides = self.cur_piece.get_bottom_facing_sides();
        for (x, y) in sides {
            if y >= BOARD_HEIGHT - 1 {
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
        self.rotation = false;
        false
    }

    fn next_piece_operations(&mut self) {
        let coords = self.cur_piece.get_coords();
        let piece = self.cur_piece.clone();
        let mut topped = true;
        for (x, y) in coords {
            if y >= 0 {
                self.set_cell_in_main_board(x, y, Cell::Full(piece.piece()));
                topped = false;
            } else {
                self.set_cell_in_buffer_board(x, y, Cell::Full(piece.piece()));
            }
        }
        self.lock_out = topped;
        self.piece_num += 1;
        self.cur_piece = self
            .queue
            .get_piece(self.piece_num)
            .unwrap()
            .try_into()
            .unwrap();
        self.piece_blocked = false;
        match self.is_line_cleared() {
            None => {
                self.clear_pattern(0, piece);
            }
            Some(y_cleared) => {
                let mut lines = 0;
                y_cleared.iter().for_each(|y| {
                    if *y != -128 {
                        lines += 1;
                    }
                });
                self.lines_cleared += lines as u32;
                self.clear_pattern(lines, piece);
                y_cleared.iter().for_each(|y| {
                    if *y != -128 {
                        self.clear_line(*y);
                    }
                });
            }
        }
    }
    fn is_line_cleared(&self) -> Option<[i16; 4]> {
        let mut lines = [-128; 4];
        let mut pieces_cleared = 0;
        let mut cur_position = 0;
        for (i, el) in self.buffer.iter().enumerate() {
            if let &Cell::Full(_) = el {
                pieces_cleared += 1;
            }
            if i % (BOARD_WIDTH) as usize == 9 {
                // Checks if an entire row has been read
                if pieces_cleared == BOARD_WIDTH {
                    // Checks if the number of cleared pieces is equal to the width of the board
                    lines[cur_position] = BOARD_HEIGHT - ((i / (BOARD_WIDTH) as usize - 1) as i16); // Stores the y position in an array
                    cur_position += 1;
                }
                pieces_cleared = 0;
            }
        }
        for (i, el) in self.cells.iter().enumerate() {
            if let &Cell::Full(_) = el {
                pieces_cleared += 1;
            }
            if i % (BOARD_WIDTH) as usize == 9 {
                // Checks if an entire row has been read
                if pieces_cleared == BOARD_WIDTH {
                    // Checks if the number of cleared pieces is equal to the width of the board
                    lines[cur_position] = (i / (BOARD_WIDTH) as usize) as i16; // Stores the y position in an array
                    cur_position += 1;
                }
                pieces_cleared = 0;
            }
        }
        if lines[0] == -128 { None } else { Some(lines) }
    }
    fn clear_line(&mut self, y: i16) {
        (-BOARD_HEIGHT..=y).rev().for_each(|y| {
            (0..BOARD_WIDTH).for_each(|x| {
                if y == -BOARD_HEIGHT {
                    self.set_cell_in_buffer_board(x, y, Cell::Empty);
                    return;
                }
                match y.cmp(&0) {
                    Ordering::Greater => {
                        self.set_cell_in_main_board(x, y, self.get_cell_from_main_board(x, y - 1));
                    }
                    Ordering::Less => {
                        self.set_cell_in_buffer_board(
                            x,
                            y,
                            self.get_cell_from_buffer_board(x, y - 1),
                        );
                    }
                    Ordering::Equal => {
                        self.set_cell_in_main_board(
                            x,
                            y,
                            self.get_cell_from_buffer_board(x, y - 1),
                        );
                    }
                }
            });
        });
    }
    fn clear_pattern(&mut self, lines_cleared: i16, piece_settled: Box<dyn MovingPiece>) {
        match (piece_settled.piece(), self.rotation) {
            (Piece::T, true) => self.t_spin_calculation(lines_cleared, piece_settled),
            _ => match lines_cleared {
                0 => self.clear_pattern = ClearLinePattern::None,
                1 => self.clear_pattern = ClearLinePattern::Single,
                2 => self.clear_pattern = ClearLinePattern::Double,
                3 => self.clear_pattern = ClearLinePattern::Triple,
                4 => self.clear_pattern = ClearLinePattern::Tetris,
                _ => panic!("Shouldn't arrive here"),
            },
        }
    }
    fn t_spin_calculation(&mut self, lines_cleared: i16, piece_settled: Box<dyn MovingPiece>) {
        let t_piece: MovingPieceT = *piece_settled
            .as_any()
            .downcast::<MovingPieceT>()
            .expect("Checked that is the correct type");
        let a_slot = t_piece.get_t_spin_point_a();
        let a_cell = if a_slot.1 >= 0
            && a_slot.1 < 20
            && a_slot.1 >= -20
            && a_slot.0 >= 0
            && a_slot.0 < 20
        {
            self.get_cell_from_main_board(a_slot.0, a_slot.1)
        } else if a_slot.1 < 20 && a_slot.1 >= -20 && a_slot.0 >= 0 && a_slot.0 < 20 {
            self.get_cell_from_buffer_board(a_slot.0, a_slot.1)
        } else {
            Cell::Full(Piece::I)
        };
        let b_slot = t_piece.get_t_spin_point_b();
        let b_cell = if b_slot.1 >= 0
            && b_slot.1 < 20
            && b_slot.1 >= -20
            && b_slot.0 >= 0
            && b_slot.0 < 20
        {
            self.get_cell_from_main_board(b_slot.0, b_slot.1)
        } else if b_slot.1 < 20 && b_slot.1 >= -20 && b_slot.0 >= 0 && b_slot.0 < 20 {
            self.get_cell_from_buffer_board(b_slot.0, b_slot.1)
        } else {
            Cell::Full(Piece::I)
        };
        let c_slot = t_piece.get_t_spin_point_c();
        let c_cell = if c_slot.1 >= 0
            && c_slot.1 < 20
            && c_slot.1 >= -20
            && c_slot.0 >= 0
            && c_slot.0 < 20
        {
            self.get_cell_from_main_board(c_slot.0, c_slot.1)
        } else if c_slot.1 < 20 && c_slot.1 >= -20 && c_slot.0 >= 0 && c_slot.0 < 20 {
            self.get_cell_from_buffer_board(c_slot.0, c_slot.1)
        } else {
            Cell::Full(Piece::I) // Piece is irrelevant
        };
        let d_slot = t_piece.get_t_spin_point_d();
        let d_cell = if d_slot.1 >= 0
            && d_slot.1 < 20
            && d_slot.1 >= -20
            && d_slot.0 >= 0
            && d_slot.0 < 20
        {
            self.get_cell_from_main_board(d_slot.0, d_slot.1)
        } else if d_slot.1 < 20 && d_slot.1 >= -20 && d_slot.0 >= 0 && d_slot.0 < 20 {
            self.get_cell_from_buffer_board(d_slot.0, d_slot.1)
        } else {
            Cell::Full(Piece::I)
        };

        if let (Cell::Full(_), Cell::Full(_)) = (a_cell, b_cell) {
            if let Cell::Full(_) = c_cell {
                match lines_cleared {
                    0 => self.clear_pattern = ClearLinePattern::TSpin,
                    1 => self.clear_pattern = ClearLinePattern::TSpinSingle,
                    2 => self.clear_pattern = ClearLinePattern::TSpinDouble,
                    3 => self.clear_pattern = ClearLinePattern::TSpinTriple,
                    _ => panic!("Shouldn't arrive here"),
                }
                return;
            }
            if let Cell::Full(_) = d_cell {
                match lines_cleared {
                    0 => self.clear_pattern = ClearLinePattern::TSpin,
                    1 => self.clear_pattern = ClearLinePattern::TSpinSingle,
                    2 => self.clear_pattern = ClearLinePattern::TSpinDouble,
                    3 => self.clear_pattern = ClearLinePattern::TSpinTriple,
                    _ => panic!("Shouldn't arrive here"),
                }
                return;
            }
        }
        if let (Cell::Full(_), Cell::Full(_)) = (c_cell, d_cell) {
            if let Cell::Full(_) = a_cell {
                match lines_cleared {
                    0 => self.clear_pattern = ClearLinePattern::MiniTSpin,
                    1 => self.clear_pattern = ClearLinePattern::MiniTSpinSingle,
                    _ => panic!("Shouldn't arrive here"),
                }
                return;
            }
            if let Cell::Full(_) = b_cell {
                match lines_cleared {
                    0 => self.clear_pattern = ClearLinePattern::MiniTSpin,
                    1 => self.clear_pattern = ClearLinePattern::MiniTSpinSingle,
                    _ => panic!("Shouldn't arrive here"),
                }
                return;
            }
        }
        match lines_cleared {
            0 => self.clear_pattern = ClearLinePattern::None,
            1 => self.clear_pattern = ClearLinePattern::Single,
            2 => self.clear_pattern = ClearLinePattern::Double,
            3 => self.clear_pattern = ClearLinePattern::Triple,
            _ => panic!("Shouldn't arrive here"),
        }
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
                .expect("Should be pieces")
                .try_into()
                .unwrap();
        } else {
            self.cur_piece = self
                .held_piece
                .expect("Already checked")
                .try_into()
                .unwrap();
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

    pub fn insert_trash(&mut self, _number_of_trash_received: u8) {
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

    fn block_out(&self) -> bool {
        for (x, y) in self.cur_piece.get_coords() {
            if let Cell::Full(_) = self.get_cell_from_buffer_board(x, y) {
                return true;
            }
        }
        false
    }

    fn get_cell_from_main_board(&self, x: i16, y: i16) -> Cell {
        self.cells[(y * BOARD_WIDTH + x) as usize]
    }

    fn set_cell_in_main_board(&mut self, x: i16, y: i16, cell: Cell) {
        self.cells[(y * BOARD_WIDTH + x) as usize] = cell;
    }

    fn get_cell_from_buffer_board(&self, x: i16, y: i16) -> Cell {
        self.buffer[((BOARD_HEIGHT + y/* y is less than 0 */) * BOARD_WIDTH + x) as usize]
    }

    fn set_cell_in_buffer_board(&mut self, x: i16, y: i16, cell: Cell) {
        self.buffer[((BOARD_HEIGHT + y) * BOARD_WIDTH + x) as usize] = cell;
    }

    fn get_highest_piece(&self) -> Option<(i16, i16)> {
        for (i, el) in self.buffer.iter().enumerate() {
            if *el != Cell::Empty {
                return Some((
                    i as i16 % BOARD_WIDTH,
                    (i as i16 / BOARD_WIDTH) - BOARD_HEIGHT,
                ));
            }
        }
        for (i, el) in self.cells.iter().enumerate() {
            if *el != Cell::Empty {
                return Some((i as i16 % BOARD_WIDTH, i as i16 / BOARD_WIDTH));
            }
        }
        None
    }
    pub fn orientation(&self) -> Orientation {
        self.cur_piece.orientation()
    }
    pub fn clear_line_pattern(&mut self) -> ClearLinePattern {
        let pattern_tmp = self.clear_pattern;
        self.clear_pattern = ClearLinePattern::None;
        pattern_tmp
    }
    pub fn lines_completed(&self) -> u32 {
        self.lines_cleared
    }
}

#[derive(Debug, Clone, Copy)]
enum RotationOption {
    ClockWise,
    CounterClockWise,
    Full,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClearLinePattern {
    None,
    Single,
    Double,
    Triple,
    Tetris,
    TSpin,
    TSpinSingle,
    TSpinDouble,
    TSpinTriple,
    MiniTSpin,
    MiniTSpinSingle,
}

#[cfg(test)]
mod tests;
