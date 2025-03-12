use crate::game::{board::local_board::LocalBoard, pieces::Piece, queue::local_queue::LocalQueue};

#[test]
fn rotation_full_1() {
    let mut board = LocalBoard::new(LocalQueue::default());
    board.cur_piece = Piece::T.try_into().unwrap();
    for _ in 0..20 {
        board.soft_drop();
    }
    let init_x = board.cur_piece.x();
    let init_y = board.cur_piece.y();
    board.rotation_full();
    assert_eq!(init_x, board.cur_piece.x());
    assert_eq!(init_y, board.cur_piece.y());
}
#[test]
fn rotation_full_2() {
    let mut board = LocalBoard::new(LocalQueue::default());
    board.cur_piece = Piece::T.try_into().unwrap();
    let init_x = board.cur_piece.x();
    let init_y = board.cur_piece.y();
    board.rotation_full();
    assert_eq!(init_x, board.cur_piece.x());
    assert_eq!(init_y + 1, board.cur_piece.y());
}
