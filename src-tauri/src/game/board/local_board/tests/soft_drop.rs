use crate::game::{
    board::{cell::Cell, local_board::LocalBoard},
    pieces::Piece,
    queue::local_queue::LocalQueue,
};

#[test]
fn soft_drop_1() {
    let mut board = LocalBoard::new(LocalQueue::default());
    board.cur_piece = Piece::I.try_into().unwrap();
    for _ in 0..20 {
        board.soft_drop();
    }
    assert_eq!([Cell::Empty; 200], board.cells);
    let mut board_check = [Cell::Empty; 200];
    for el in board_check.iter_mut().take(197).skip(193) {
        *el = Cell::Full(Piece::I);
    }
    board.next_tick();
    assert_eq!([Cell::Empty; 200], board.cells);
}
