use crate::{
    game::{board::local_board::LocalBoard, pieces::Piece, queue::local_queue::LocalQueue},
    init_trace::initialize,
};

#[test]
fn piece_saved_correctly() {
    initialize();
    let mut board = LocalBoard::new(LocalQueue::default());
    let pieces = board.get_pieces(0..5);
    board.save_piece();
    let piece = board.held_piece();
    assert!(piece.is_some());
    assert_eq!(pieces[0], piece.unwrap());
}

#[test]
fn piece_saved_correctly_with_previously_saved_piece() {
    initialize();
    let mut board = LocalBoard::new(LocalQueue::default());
    let pieces = board.get_pieces(0..2);
    println!("{:?}", &pieces);
    board.save_piece();
    board.piece_blocked = false;
    board.save_piece();
    let piece = board.held_piece();
    println!("{:?}", piece);
    let current_piece = board.cur_piece;
    println!("{:?}", current_piece);
    assert_eq!(pieces[0], current_piece.piece());
    assert!(piece.is_some());
    assert_eq!(pieces[1], piece.unwrap());
}

#[test]
fn piece_saved_lock() {
    initialize();
    let mut board = LocalBoard::new(LocalQueue::default());
    board.cur_piece = Piece::T.try_into().unwrap();
    board.save_piece();
    board.cur_piece = Piece::I.try_into().unwrap();
    board.save_piece();
    board.save_piece();
    board.save_piece();
    board.save_piece();
    board.save_piece();
    board.save_piece();
    board.hard_drop();
    board.cur_piece = Piece::J.try_into().unwrap();
    board.save_piece();
    assert_eq!(board.cur_piece.piece(), Piece::T);
    assert!(board.held_piece.is_some());
    assert_eq!(board.held_piece.unwrap(), Piece::J);
}
