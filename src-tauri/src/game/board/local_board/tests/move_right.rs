use crate::game::{
    board::{cell::Cell, local_board::LocalBoard},
    pieces::Piece,
    queue::local_queue::LocalQueue,
};

#[test]
fn move_right_1() {
    let mut board = LocalBoard::new(LocalQueue::default());
    board.cur_piece = Piece::I.try_into().unwrap();
    for _ in 0..10 {
        board.move_right();
    }
    let coords = board.cur_piece.get_coords();
    assert_eq!(coords[0].0, 6);
    assert_eq!(coords[1].0, 7);
    assert_eq!(coords[2].0, 8);
    assert_eq!(coords[3].0, 9);
}

#[test]
fn move_right_2() {
    let mut board = LocalBoard::new(LocalQueue::default());
    board.cur_piece = Piece::I.try_into().unwrap();
    board.cells[199] = Cell::Full(Piece::L);
    for _ in 0..21 {
        board.soft_drop();
    }
    for _ in 0..5 {
        board.move_right();
    }
    let coords = board.cur_piece.get_coords();
    assert_eq!(coords[0].0, 5);
    assert_eq!(coords[1].0, 6);
    assert_eq!(coords[2].0, 7);
    assert_eq!(coords[3].0, 8);
}
#[test]
fn move_right_3() {
    let mut board = LocalBoard::new(LocalQueue::default());
    board.cur_piece = Piece::I.try_into().unwrap();
    board.cells[198] = Cell::Full(Piece::L);
    for _ in 0..21 {
        board.soft_drop();
    }
    for _ in 0..5 {
        board.move_right();
    }
    let coords = board.cur_piece.get_coords();
    assert_eq!(coords[0].0, 4);
    assert_eq!(coords[1].0, 5);
    assert_eq!(coords[2].0, 6);
    assert_eq!(coords[3].0, 7);
}
