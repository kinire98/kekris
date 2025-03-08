use crate::{
    game::{
        board::{
            cell::Cell,
            local_board::{ClearLinePattern, LocalBoard},
            Board,
        },
        pieces::Piece,
        queue::local_queue::LocalQueue,
    },
    init_trace::initialize,
};

#[test]
fn clear_single_line() {
    initialize();
    let mut board = LocalBoard::new(LocalQueue::new());
    let mut cells: [Cell; 200] = [Cell::Empty; 200];
    for i in 191..200 {
        cells[i] = Cell::Full(Piece::I);
    }
    let mut cells_check = [Cell::Empty; 200];
    cells_check[190] = Cell::Full(Piece::I);
    cells_check[180] = Cell::Full(Piece::I);
    cells_check[170] = Cell::Full(Piece::I);
    board.cells = cells;
    board.cur_piece = Piece::I.try_into().unwrap();
    board.rotation_clockwise();
    for _ in 0..5 {
        board.move_left();
    }
    board.hard_drop();
    assert_eq!(cells_check, board.cells);
}

#[test]
fn clear_two_lines() {
    initialize();
    let cells_check = [Cell::Empty; 200];
    let mut board = LocalBoard::new(LocalQueue::new());
    board.cur_piece = Piece::T.try_into().unwrap();
    for i in 180..183 {
        board.cells[i] = Cell::Full(Piece::T);
    }
    for i in 186..190 {
        board.cells[i] = Cell::Full(Piece::T);
    }
    for i in 190..194 {
        board.cells[i] = Cell::Full(Piece::T);
    }
    for i in 195..200 {
        board.cells[i] = Cell::Full(Piece::T);
    }
    board.rotation_full();
    board.hard_drop();
    assert_eq!(cells_check, board.cells);
}

#[test]
fn clear_three_lines() {
    let cells_check = [Cell::Empty; 200];
    let mut board = LocalBoard::new(LocalQueue::new());
    board.cur_piece = Piece::J.try_into().unwrap();
    for i in 172..180 {
        board.cells[i] = Cell::Full(Piece::J);
    }
    for i in 181..190 {
        board.cells[i] = Cell::Full(Piece::J);
    }
    for i in 191..200 {
        board.cells[i] = Cell::Full(Piece::J);
    }
    board.rotation_clockwise();
    for _ in 0..5 {
        board.move_left();
    }
    board.hard_drop();
    assert_eq!(cells_check, board.cells);
}

#[test]
fn bad_play_but_not_terrible() {
    let mut cells_check = [Cell::Empty; 200];
    let mut board = LocalBoard::new(LocalQueue::new());
    board.cur_piece = Piece::J.try_into().unwrap();
    for i in 172..180 {
        board.cells[i] = Cell::Full(Piece::J);
    }
    for i in 181..190 {
        board.cells[i] = Cell::Full(Piece::J);
        cells_check[i] = Cell::Full(Piece::J);
    }
    for i in 191..200 {
        board.cells[i] = Cell::Full(Piece::J);
        cells_check[i] = Cell::Full(Piece::J);
    }
    cells_check[161] = Cell::Full(Piece::J);
    cells_check[171] = Cell::Full(Piece::J);
    board.rotation_counterclockwise();
    for _ in 0..5 {
        board.move_left();
    }
    board.hard_drop();
    assert_eq!(cells_check, board.cells);
}

#[test]
fn pattern_correct_behaviour() {
    initialize();
    let mut board = LocalBoard::new(LocalQueue::new());
    board.clear_pattern = ClearLinePattern::Tetris;
    assert_eq!(ClearLinePattern::Tetris, board.clear_line_pattern());
    assert_eq!(ClearLinePattern::None, board.clear_line_pattern());
}
