use crate::game::{
    board::{cell::Cell, local_board::LocalBoard},
    pieces::Piece,
    queue::local_queue::LocalQueue,
};

#[test]
fn hard_drop_1() {
    let mut check_board = [Cell::Empty; 200];
    for el in check_board.iter_mut().take(197).skip(193) {
        *el = Cell::Full(Piece::I);
    }
    let mut board = LocalBoard::new(LocalQueue::default());
    board.cur_piece = Piece::I.try_into().unwrap();
    board.hard_drop();
    assert_eq!(check_board, board.cells);
}

#[test]
fn hard_drop_2() {
    let mut check_board = [Cell::Empty; 200];
    check_board[160] = Cell::Full(Piece::I);
    check_board[170] = Cell::Full(Piece::I);
    check_board[180] = Cell::Full(Piece::I);
    check_board[190] = Cell::Full(Piece::I);
    let mut board = LocalBoard::new(LocalQueue::default());
    board.cur_piece = Piece::I.try_into().unwrap();
    board.rotation_clockwise();
    board.move_left();
    board.move_left();
    board.move_left();
    board.move_left();
    board.move_left();
    board.move_left();
    board.hard_drop();
    assert_eq!(check_board, board.cells);
}
