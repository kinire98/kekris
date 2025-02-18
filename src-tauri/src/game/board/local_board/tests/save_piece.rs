use crate::{
    game::{
        board::{local_board::LocalBoard, Board},
        queue::local_queue::LocalQueue,
    },
    init_trace::initialize,
};

#[test]
fn piece_saved_correctly() {
    initialize();
    let mut board = LocalBoard::new(LocalQueue::new());
    let pieces = board.get_pieces(0..5);
    board.save_piece();
    let piece = board.held_piece();
    assert!(piece.is_some());
    assert_eq!(pieces[0], piece.unwrap());
}

#[test]
fn piece_saved_correctly_with_previously_saved_piece() {
    initialize();
    let mut board = LocalBoard::new(LocalQueue::new());
    let pieces = board.get_pieces(0..2);
    println!("{:?}", &pieces);
    board.save_piece();
    board.piece_blocked = false;
    board.save_piece();
    let piece = board.held_piece();
    println!("{:?}", piece);
    let current_piece = board.cur_piece;
    println!("{:?}", current_piece);
    assert_eq!(pieces[0], current_piece);
    assert!(piece.is_some());
    assert_eq!(pieces[1], piece.unwrap());
}

#[test]
fn piece_saved_lock() {
    initialize();
    let mut board = LocalBoard::new(LocalQueue::new());
    let pieces = board.get_pieces(0..3);
    board.save_piece();
    assert!(board.piece_blocked);
    board.save_piece();
    board.save_piece();
    board.save_piece();
    board.save_piece();
    board.save_piece();
    let held_piece = board.held_piece();
    assert!(held_piece.is_some());
    assert_eq!(pieces[0], held_piece.unwrap());
    board.next_piece();
    assert!(!board.piece_blocked);
    board.save_piece();
    assert!(board.piece_blocked);
    let held_piece = board.held_piece();
    assert!(held_piece.is_some());
    assert_eq!(pieces[1], held_piece.unwrap());
}
