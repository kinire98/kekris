#![allow(clippy::needless_range_loop)]

use crate::game::{
    board::{cell::Cell, local_board::LocalBoard},
    pieces::Piece,
    queue::local_queue::LocalQueue,
};

#[test]
fn insert_trash() {
    let mut board = LocalBoard::new(LocalQueue::default());
    board.trash_lines_queue = vec![(3, 0)];
    board.cur_piece = Piece::I.try_into().unwrap();
    board.hard_drop();
    let mut cmp_board = [Cell::Empty; 200];
    for i in 191..200 {
        cmp_board[i] = Cell::Full(Piece::Trash);
    }
    for i in 181..190 {
        cmp_board[i] = Cell::Full(Piece::Trash);
    }
    for i in 171..180 {
        cmp_board[i] = Cell::Full(Piece::Trash);
    }
    for i in 163..167 {
        cmp_board[i] = Cell::Full(Piece::I);
    }
    assert_eq!(board.cells, cmp_board);
    board.cur_piece = Piece::I.try_into().unwrap();
    board.hard_drop();
    for i in 153..157 {
        cmp_board[i] = Cell::Full(Piece::I);
    }
    assert_eq!(board.cells, cmp_board);
}

#[test]
fn insert_trash_different_pattern() {
    let mut board = LocalBoard::new(LocalQueue::default());
    board.cur_piece = Piece::I.try_into().unwrap();
    board.trash_lines_queue = vec![(1, 0), (1, 9)];
    board.hard_drop();
    let mut cmp_board = [Cell::Empty; 200];
    for i in 191..200 {
        cmp_board[i] = Cell::Full(Piece::Trash);
    }
    for i in 180..189 {
        cmp_board[i] = Cell::Full(Piece::Trash);
    }
    for i in 173..177 {
        cmp_board[i] = Cell::Full(Piece::I);
    }
    assert_eq!(board.cells, cmp_board);
}
#[test]
fn removing_trash_lines() {
    let mut board = LocalBoard::new(LocalQueue::default());
    board.cur_piece = Piece::I.try_into().unwrap();
    board.trash_lines_queue = vec![(1, 0)];
    board.hard_drop();
    board.cur_piece = Piece::I.try_into().unwrap();
    board.rotation_clockwise();
    for _ in 0..10 {
        board.move_left();
    }
    board.hard_drop();
    let mut cmp_board = [Cell::Empty; 200];
    cmp_board[190] = Cell::Full(Piece::I);
    cmp_board[180] = Cell::Full(Piece::I);
    cmp_board[170] = Cell::Full(Piece::I);
    for i in 193..197 {
        cmp_board[i] = Cell::Full(Piece::I);
    }
    assert_eq!(board.cells, cmp_board);
}
