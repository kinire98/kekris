use crate::game::{
    board::{cell::Cell, local_board::LocalBoard},
    pieces::Piece,
    queue::local_queue::LocalQueue,
};

#[test]
fn move_left_1() {
    let mut board = LocalBoard::new(LocalQueue::default());
    board.cur_piece = Piece::I.try_into().unwrap();
    for _ in 0..5 {
        board.move_left();
    }
    let coords = board.cur_piece.get_coords();
    for i in 0..4 {
        assert_eq!(coords[i].0 as usize, i);
    }
}

#[test]
fn move_left_2() {
    let mut board = LocalBoard::new(LocalQueue::default());
    board.cur_piece = Piece::I.try_into().unwrap();
    board.cells[190] = Cell::Full(Piece::L);
    for _ in 0..21 {
        board.soft_drop();
    }
    for _ in 0..5 {
        board.move_left();
    }
    let coords = board.cur_piece.get_coords();
    for i in 0..4 {
        println!("x: {} y:{}", coords[i].0, coords[i].1);
        assert_eq!(coords[i].0 as usize, i + 1);
    }
}
#[test]
fn move_left_3() {
    let mut board = LocalBoard::new(LocalQueue::default());
    board.cur_piece = Piece::I.try_into().unwrap();
    board.cells[191] = Cell::Full(Piece::L);
    for _ in 0..21 {
        board.soft_drop();
    }
    for _ in 0..5 {
        board.move_left();
    }
    let coords = board.cur_piece.get_coords();
    for i in 0..4 {
        println!("x: {} y:{}", coords[i].0, coords[i].1);
        assert_eq!(coords[i].0 as usize, i + 2);
    }
}
