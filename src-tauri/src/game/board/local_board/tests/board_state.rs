use crate::{
    game::{
        board::{cell::Cell, local_board::LocalBoard, Board},
        pieces::Piece,
        queue::local_queue::LocalQueue,
    },
    init_trace::initialize,
};

#[test]
fn board_state_1() {
    initialize();
    let expected_board = "E".repeat(200);
    let board = LocalBoard::new(LocalQueue::new());
    assert_eq!(expected_board, board.board_state());
}

#[test]
fn board_state_2() {
    let expected_board = format!(
        "{}{}{}{}{}{}{}{}{}{}",
        "E".repeat(110),
        "G".repeat(10),
        "T".repeat(10),
        "I".repeat(10),
        "O".repeat(10),
        "L".repeat(10),
        "J".repeat(10),
        "S".repeat(10),
        "Z".repeat(10),
        "R".repeat(10)
    );
    let mut board = Vec::new();
    for _ in 0..110 {
        board.push(Cell::Empty);
    }
    for _ in 0..10 {
        board.push(Cell::Full(Piece::Ghost));
    }
    for _ in 0..10 {
        board.push(Cell::Full(Piece::T));
    }
    for _ in 0..10 {
        board.push(Cell::Full(Piece::I));
    }
    for _ in 0..10 {
        board.push(Cell::Full(Piece::O));
    }
    for _ in 0..10 {
        board.push(Cell::Full(Piece::L));
    }
    for _ in 0..10 {
        board.push(Cell::Full(Piece::J));
    }
    for _ in 0..10 {
        board.push(Cell::Full(Piece::S));
    }
    for _ in 0..10 {
        board.push(Cell::Full(Piece::Z));
    }
    for _ in 0..10 {
        board.push(Cell::Full(Piece::Trash));
    }
    let mut board_impl = LocalBoard::new(LocalQueue::new());
    board_impl.cells = board.try_into().unwrap();
    assert_eq!(expected_board, board_impl.board_state());
}

#[test]
fn board_state_3() {
    let expected_board = format!(
        "{}{}{}{}{}",
        "E".repeat(181),
        "T",
        "E".repeat(8),
        "T".repeat(3),
        "E".repeat(7)
    );
    let mut cells = Vec::new();
    for _ in 0..181 {
        cells.push(Cell::Empty);
    }
    cells.push(Cell::Full(Piece::T));
    for _ in 0..8 {
        cells.push(Cell::Empty);
    }
    for _ in 0..3 {
        cells.push(Cell::Full(Piece::T));
    }
    for _ in 0..7 {
        cells.push(Cell::Empty);
    }
    let mut board = LocalBoard::new(LocalQueue::new());
    board.cells = cells.try_into().unwrap();
    assert_eq!(expected_board, board.board_state());
}

