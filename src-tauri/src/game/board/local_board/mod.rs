use std::{cmp::Ordering, collections::HashMap, ops::Range};

use moving_piece::{MovingPiece, Orientation, moving_piece_t::MovingPieceT};
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::game::{pieces::Piece, queue::Queue, strategy::Strategy};

use super::{Board, cell::Cell, danger_level::DangerLevel};

mod moving_piece;

/// The width of the game board.
const BOARD_WIDTH: i16 = 10;
/// The height of the game board.
const BOARD_HEIGHT: i16 = 20;

/// `LocalBoard` represents the game board for a single-player game.
#[derive(Debug)]
pub struct LocalBoard {
    /// The queue of upcoming pieces.
    queue: Box<dyn Queue>,
    /// The currently held piece.
    held_piece: Option<Piece>,
    /// The currently moving piece.
    cur_piece: Box<dyn MovingPiece>,
    /// The strategy being used by the board.
    strategy: Strategy,
    /// The number of pieces that have been placed on the board.
    piece_num: usize,
    /// The queue of trash lines to be added to the board.
    trash_lines_queue: Vec<(u8, u8)>,
    /// The cells of the main board.
    cells: [Cell; (BOARD_HEIGHT * BOARD_WIDTH) as usize],
    /// The cells of the buffer board (used for pieces above the visible board).
    buffer: [Cell; (BOARD_HEIGHT * BOARD_WIDTH) as usize],
    /// A boolean indicating whether the player has locked out (placed a piece too high).
    lock_out: bool,
    /// A boolean indicating whether the player has topped out (filled the board to the top).
    top_out: bool,
    /// A boolean indicating whether the current piece is blocked.
    piece_blocked: bool,
    /// A boolean indicating whether a line has been cleared.
    line_cleared: bool,
    /// The number of lines that have been cleared.
    lines_cleared: u32,
    /// The pattern of lines that have been cleared.
    clear_pattern: ClearLinePattern,
    /// A boolean indicating whether the piece has been rotated.
    rotation: bool,
    /// The rotation option used.
    rotation_option: RotationOption,
    /// The variation of the rotation used.
    rotation_variation: i16,
}
impl Board for LocalBoard {
    /// Checks if the game is over based on topping out, locking out, or blocking out.
    fn game_over(&self) -> bool {
        self.top_out || self.lock_out || self.block_out()
    }

    /// Checks if the game has been won based on the given win condition.
    ///
    /// The win condition is a closure that takes a boolean indicating whether the game is over
    /// and the number of lines cleared as input, and returns a boolean indicating whether the game has been won.
    fn game_won(&self, win_condition: impl Fn(bool, u32) -> bool) -> bool {
        win_condition(self.game_over(), self.lines_cleared)
    }

    /// Returns the current state of the board as a string.
    ///
    /// The string representation includes the state of the main board, the buffer board,
    /// and the current piece.
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

    /// Returns the strategy being used by the board.
    fn strategy(&self) -> Strategy {
        self.strategy
    }
    /// Returns the danger level of the board based on the height of the highest piece.
    fn danger_level(&self) -> DangerLevel {
        let coords = self.get_highest_piece();
        if coords.is_none() {
            return DangerLevel::Empty;
        }

        let coords = coords.unwrap();
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
    /// Creates a new `LocalBoard` instance.
    ///
    /// Initializes the board with an empty queue, no held piece, a new current piece,
    /// a default strategy, and empty data structures for trash lines, cells, and other state.
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
            top_out: false,
            piece_blocked: false,
            line_cleared: false,
            lines_cleared: 0,
            clear_pattern: ClearLinePattern::None,
            rotation: false,
            rotation_option: RotationOption::Full,
            rotation_variation: 0,
        }
    }
    /// Moves the current piece to the right if possible.
    ///
    /// Checks if there are any obstructions to the right of the piece and moves the piece if there are none.
    pub fn move_right(&mut self) -> bool {
        let sides = self.cur_piece.get_right_facing_sides();
        for (x, y) in sides {
            if x == BOARD_WIDTH - 1 {
                return false;
            }
            if y >= 0 {
                match self.get_cell_from_main_board(x + 1, y) {
                    Cell::Empty => continue,
                    Cell::Full(_) => return false,
                }
            } else {
                match self.get_cell_from_buffer_board(x + 1, y) {
                    Cell::Empty => continue,
                    Cell::Full(_) => return false,
                }
            }
        }
        self.cur_piece.move_right();
        true
    }

    /// Moves the current piece to the left if possible.
    ///
    /// Checks if there are any obstructions to the left of the piece and moves the piece if there are none.
    pub fn move_left(&mut self) -> bool {
        let sides = self.cur_piece.get_left_facing_sides();
        for (x, y) in sides {
            if x == 0 {
                return false;
            }
            if y >= 0 {
                match self.get_cell_from_main_board(x - 1, y) {
                    Cell::Empty => continue,
                    Cell::Full(_) => return false,
                }
            } else {
                match self.get_cell_from_buffer_board(x - 1, y) {
                    Cell::Empty => continue,
                    Cell::Full(_) => return false,
                }
            }
        }
        self.cur_piece.move_left();
        true
    }

    /// Rotates the current piece clockwise if possible.
    pub fn rotation_clockwise(&mut self) {
        let rotation_piece = self.cur_piece.clone();
        self.check_rotation(rotation_piece, 1..6, RotationOption::ClockWise);
        self.rotation = true;
    }

    /// Rotates the current piece counterclockwise if possible.
    pub fn rotation_counterclockwise(&mut self) {
        let rotation_piece = self.cur_piece.clone();
        self.check_rotation(rotation_piece, 1..6, RotationOption::CounterClockWise);
        self.rotation = true;
    }

    /// Rotates the current piece 180 degrees if possible.
    pub fn rotation_full(&mut self) {
        let rotation_piece = self.cur_piece.clone();
        self.check_rotation(rotation_piece, 1..3, RotationOption::Full);
    }
    /// Checks if a rotation is possible and performs it.
    ///
    /// Iterates through a range of possible rotation variations and checks if the rotation is valid.
    /// If a valid rotation is found, the current piece is updated and the function returns.
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

    /// Checks if the current piece is at the bottom of the board.
    ///
    /// Returns true if the piece cannot move down any further, either because it has reached the bottom of the board
    /// or because it is blocked by another piece.
    pub fn piece_at_bottom(&mut self) -> bool {
        self.push_down(false)
    }
    /// Moves the current piece down by one row.
    pub fn soft_drop(&mut self) {
        let _ = self.push_down(true);
    }

    /// Advances the game by one tick.
    ///
    /// Moves the current piece down by one row and checks if it has reached the bottom of the board.
    /// If the piece has reached the bottom, it is fixed in place and a new piece is generated.
    pub fn next_tick(&mut self) -> bool {
        let is_in_bottom = self.push_down(true);
        if !is_in_bottom {
            return false;
        }
        self.next_piece_operations();
        true
    }

    /// Hard drops the current piece to the bottom of the board.
    ///
    /// Moves the current piece down until it reaches the bottom of the board, then fixes it in place and generates a new piece.
    pub fn hard_drop(&mut self) {
        while !self.push_down(true) {}
        self.next_piece_operations();
    }

    /// Generates a ghost piece for the current piece.
    ///
    /// The ghost piece is a visual aid that shows where the current piece will land if it is dropped.
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

    /// Pushes the current piece down by one row.
    ///
    /// Returns true if the piece cannot move down any further, either because it has reached the bottom of the board
    /// or because it is blocked by another piece.
    fn push_down(&mut self, move_piece: bool) -> bool {
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
        if move_piece {
            self.cur_piece.move_down();
        }
        self.rotation = false;
        false
    }

    /// Performs operations that occur after a piece has been placed.
    ///
    /// This includes fixing the piece in place, generating a new piece, checking for line clears,
    /// and setting any trash lines that have been received.
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
        self.set_trash_in_board();
    }
    /// Checks if any lines have been cleared and returns the y coordinates of the cleared lines.
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
    /// Clears a line at the given y coordinate.
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
    /// Determines the clear pattern based on the lines cleared and the piece that was settled.
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
    /// Calculates the clear pattern for a T-spin.
    ///
    /// A T-spin is a special type of line clear that occurs when a T-shaped piece is used to clear lines
    /// in a specific configuration. This function checks if the current line clear qualifies as a T-spin
    /// and sets the clear pattern accordingly.
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

    /// Sets the trash lines in the board.
    ///
    /// Trash lines are lines that are added to the bottom of the board to make it more difficult for the player.
    fn set_trash_in_board(&mut self) {
        if self.trash_lines_queue.is_empty() {
            return;
        }
        let lines = self
            .trash_lines_queue
            .iter()
            .fold(0, |acc, val| acc + val.0);
        if self.top_out_check(lines) {
            self.top_out = true;
            return;
        }
        for (i, _) in self.buffer.clone().iter().enumerate() {
            let modified = i + (BOARD_WIDTH as usize * lines as usize);
            if modified >= ((BOARD_HEIGHT * BOARD_WIDTH) - 1) as usize {
                self.buffer[i] = self.cells[modified - ((BOARD_HEIGHT * BOARD_WIDTH - 1) as usize)];
            } else {
                self.buffer[i] = self.buffer[modified];
            }
        }
        for (i, _) in self.cells.clone().iter().enumerate() {
            let modified = i + (BOARD_WIDTH as usize * lines as usize);
            if modified >= ((BOARD_HEIGHT * BOARD_WIDTH) - 1) as usize {
                break;
            } else {
                self.cells[i] = self.cells[modified];
            }
        }
        let mut lines_added = 0;
        for (number, column) in self.trash_lines_queue.clone() {
            for i in 0..number {
                self.add_lines(column as i16, BOARD_HEIGHT - 1 - lines_added - i as i16);
            }
            lines_added += number as i16;
        }
        self.trash_lines_queue.clear();
    }
    /// Adds lines to the board.
    fn add_lines(&mut self, x: i16, y: i16) {
        for i in 0..BOARD_WIDTH {
            match (i == x, y < 0) {
                (true, true) => self.set_cell_in_buffer_board(i, y, Cell::Empty),
                (true, false) => self.set_cell_in_main_board(i, y, Cell::Empty),
                (false, true) => self.set_cell_in_buffer_board(i, y, Cell::Full(Piece::Trash)),
                (false, false) => self.set_cell_in_main_board(i, y, Cell::Full(Piece::Trash)),
            }
        }
    }
    /// Checks if the board has topped out.
    fn top_out_check(&mut self, lines: u8) -> bool {
        let highest_piece = self.get_highest_piece();
        if let Some((_, y)) = highest_piece {
            let remaining_pieces = if y < 0 {
                BOARD_HEIGHT * 2 - (BOARD_HEIGHT + y)
            } else if y > 0 {
                y + BOARD_HEIGHT
            } else {
                BOARD_HEIGHT
            };
            lines as i16 >= remaining_pieces
        } else {
            false
        }
    }

    /// Saves the current piece for later use.
    ///
    /// If there is no held piece, the current piece is saved and a new piece is generated.
    /// If there is a held piece, the current piece is swapped with the held piece.
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

    /// Changes the strategy being used by the board.
    pub fn change_strategy(&mut self, strategy: Strategy) {
        self.strategy = strategy;
    }

    /// Returns the currently held piece.
    pub fn held_piece(&self) -> Option<Piece> {
        self.held_piece
    }

    /// Returns the number of trash lines in the queue.
    pub fn num_of_trash_lines(&self) -> u8 {
        self.trash_lines_queue
            .iter()
            .fold(0, |acc, (lines, _)| acc + lines)
    }

    /// Inserts trash lines into the queue.
    ///
    /// Trash lines are added to the queue and will be added to the board at a later time.
    pub fn insert_trash(&mut self, number_of_trash_received: u8) {
        if self.trash_lines_queue.is_empty() {
            self.trash_lines_queue.push((
                number_of_trash_received,
                rand::rng().random_range(0..BOARD_WIDTH) as u8,
            ));
            return;
        }
        let sum = self
            .trash_lines_queue
            .iter()
            .fold(0, |acc, val| acc + val.0);
        if sum < 5 {
            self.trash_lines_queue.push((
                number_of_trash_received,
                rand::rng().random_range(0..BOARD_WIDTH) as u8,
            ));
            return;
        }
        let mut repetitions: HashMap<u8, u8> = HashMap::new();
        for (lines, column) in &self.trash_lines_queue {
            *repetitions.entry(*column).or_insert(0) += lines;
        }
        if let Some((key, _)) = repetitions.iter().max_by_key(|entry| entry.1) {
            self.trash_lines_queue
                .push((number_of_trash_received, *key));
        } else {
            self.trash_lines_queue.push((
                number_of_trash_received,
                rand::rng().random_range(0..BOARD_WIDTH) as u8,
            ));
        }
    }

    /// Counters the trash lines with the lines cleared.
    ///
    /// If the player clears lines, they can be used to reduce the number of trash lines in the queue.
    pub fn counter_trash(&mut self, lines_cleared: u8) -> u8 {
        if self.trash_lines_queue.is_empty() {
            return lines_cleared;
        }
        let mut lines_cleared = lines_cleared;
        for (index, (lines, column)) in self.trash_lines_queue.clone().iter().enumerate() {
            match lines_cleared.cmp(lines) {
                Ordering::Less => {
                    self.trash_lines_queue[index] = (lines - lines_cleared, *column);
                    return 0;
                }
                Ordering::Equal => {
                    let _ = self.trash_lines_queue.remove(index);
                    return 0;
                }
                Ordering::Greater => {
                    lines_cleared -= lines;
                    let _ = self.trash_lines_queue.remove(index);
                }
            }
        }
        lines_cleared
    }

    /// Gets the pieces in the queue within the given range.
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

    /// Returns a boolean indicating whether the current piece is blocked.
    pub fn piece_blocked(&self) -> bool {
        self.piece_blocked
    }

    /// Returns a boolean indicating whether a line has been cleared.
    pub fn line_cleared(&self) -> bool {
        self.line_cleared
    }

    /// Checks if the board has blocked out.
    ///
    /// Block out occurs when the current piece is partially above the board and cannot move down any further.
    fn block_out(&self) -> bool {
        for (x, y) in self.cur_piece.get_coords() {
            if y >= 0 {
                return false;
            }
            if let Cell::Full(_) = self.get_cell_from_buffer_board(x, y) {
                return true;
            }
        }
        false
    }

    /// Gets the cell at the given coordinates from the main board.
    fn get_cell_from_main_board(&self, x: i16, y: i16) -> Cell {
        self.cells[(y * BOARD_WIDTH + x) as usize]
    }

    /// Sets the cell at the given coordinates in the main board.
    fn set_cell_in_main_board(&mut self, x: i16, y: i16, cell: Cell) {
        self.cells[(y * BOARD_WIDTH + x) as usize] = cell;
    }

    /// Gets the cell at the given coordinates from the buffer board.
    fn get_cell_from_buffer_board(&self, x: i16, y: i16) -> Cell {
        self.buffer[((BOARD_HEIGHT + y/* y is less than 0 */) * BOARD_WIDTH + x) as usize]
    }

    /// Sets the cell at the given coordinates in the buffer board.
    fn set_cell_in_buffer_board(&mut self, x: i16, y: i16, cell: Cell) {
        self.buffer[((BOARD_HEIGHT + y) * BOARD_WIDTH + x) as usize] = cell;
    }

    /// Gets the highest piece on the board.
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
    /// Returns the orientation of the current piece.
    pub fn orientation(&self) -> Orientation {
        self.cur_piece.orientation()
    }
    /// Returns the clear line pattern.
    pub fn clear_line_pattern(&mut self) -> ClearLinePattern {
        let pattern_tmp = self.clear_pattern;
        self.clear_pattern = ClearLinePattern::None;
        pattern_tmp
    }
    /// Returns the number of lines completed.
    pub fn lines_completed(&self) -> u32 {
        self.lines_cleared
    }
    /// Returns the piece number.
    pub fn piece_num(&self) -> usize {
        self.piece_num
    }
    /// Returns the current piece.
    pub fn cur_piece(&self) -> Piece {
        self.cur_piece.piece()
    }
    /// Returns the x coordinate of the current piece.
    pub fn piece_x(&self) -> i16 {
        self.cur_piece.x()
    }
    /// Returns the y coordinate of the current piece.
    pub fn piece_y(&self) -> i16 {
        self.cur_piece.y()
    }
    /// Inserts pieces into the queue.
    pub fn insert_in_queue(&mut self, pieces: Vec<Piece>) {
        self.queue.insert_pieces(pieces);
    }
    /// Gets the queue.
    pub fn get_queue(&mut self) -> Vec<Piece> {
        self.queue.get_pieces()
    }
}

/// `RotationOption` represents the different rotation options for a piece.
#[derive(Debug, Clone, Copy)]
enum RotationOption {
    /// Rotate the piece clockwise.
    ClockWise,
    /// Rotate the piece counterclockwise.
    CounterClockWise,
    /// Rotate the piece 180 degrees.
    Full,
}

/// `ClearLinePattern` represents the different patterns of lines that can be cleared.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClearLinePattern {
    /// No lines were cleared.
    None,
    /// A single line was cleared.
    Single,
    /// A double line was cleared.
    Double,
    /// A triple line was cleared.
    Triple,
    /// A tetris (four lines) was cleared.
    Tetris,
    /// A T-spin was performed.
    TSpin,
    /// A T-spin single line clear was performed.
    TSpinSingle,
    /// A T-spin double line clear was performed.
    TSpinDouble,
    /// A T-spin triple line clear was performed.
    TSpinTriple,
    /// A mini T-spin was performed.
    MiniTSpin,
    /// A mini T-spin single line clear was performed.
    MiniTSpinSingle,
}

#[cfg(test)]
mod tests;
