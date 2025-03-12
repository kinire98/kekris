use crate::{
    game::{
        board::{cell::Cell, local_board::LocalBoard},
        pieces::Piece,
        queue::local_queue::LocalQueue,
    },
    init_trace::initialize,
};

#[test]
fn rotation_counterclockwise_1() {
    initialize();
    let mut board_check = [Cell::Empty; 200];
    board_check[199] = Cell::Full(Piece::I);
    let mut board = LocalBoard::new(LocalQueue::default());
    for i in 170..179 {
        board.cells[i] = Cell::Full(Piece::I);
    }
    for i in 180..189 {
        board.cells[i] = Cell::Full(Piece::I);
    }
    for i in 190..199 {
        board.cells[i] = Cell::Full(Piece::I);
    }
    board.cur_piece = Piece::I.try_into().unwrap();
    for _ in 0..5 {
        board.move_right();
    }
    for _ in 0..20 {
        board.soft_drop();
    }
    board.rotation_clockwise();
    board.next_tick();
    board.next_tick();
    assert_eq!(board_check, board.cells);
}

#[test]
fn rotation_counterclockwise_2() {
    let mut board = LocalBoard::new(LocalQueue::default());
    board.cur_piece = Piece::I.try_into().unwrap();
    let init_x = board.cur_piece.x();
    let init_y = board.cur_piece.y();
    board.rotation_counterclockwise();
    assert_eq!(init_x + 1, board.cur_piece.x());
    assert_eq!(init_y - 1, board.cur_piece.y());
}
