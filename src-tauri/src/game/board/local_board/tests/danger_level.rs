use crate::game::{
    board::{Board, danger_level::DangerLevel, local_board::LocalBoard},
    pieces::Piece,
    queue::local_queue::LocalQueue,
};

#[test]
fn danger_level_empty() {
    let board = LocalBoard::new(LocalQueue::default());
    assert_eq!(board.danger_level(), DangerLevel::Empty);
}

#[test]
fn danger_level_very_low() {
    let mut board = LocalBoard::new(LocalQueue::default());
    board.hard_drop();
    assert_eq!(board.danger_level(), DangerLevel::VeryLow);
}

#[test]
fn danger_level_low() {
    let mut board = LocalBoard::new(LocalQueue::default());
    for _ in 0..3 {
        board.cur_piece = Piece::T.try_into().unwrap();
        board.hard_drop();
    }
    assert_eq!(DangerLevel::Low, board.danger_level());
}

#[test]
fn danger_level_medium() {
    let mut board = LocalBoard::new(LocalQueue::default());
    for _ in 0..4 {
        board.cur_piece = Piece::T.try_into().unwrap();
        board.hard_drop();
    }
    assert_eq!(DangerLevel::Medium, board.danger_level());
}

#[test]
fn danger_level_high() {
    let mut board = LocalBoard::new(LocalQueue::default());
    for _ in 0..7 {
        board.cur_piece = Piece::T.try_into().unwrap();
        board.hard_drop();
    }
    assert_eq!(DangerLevel::High, board.danger_level());
}

#[test]
fn danger_level_very_high() {
    let mut board = LocalBoard::new(LocalQueue::default());
    for _ in 0..8 {
        board.cur_piece = Piece::T.try_into().unwrap();
        board.hard_drop();
    }
    assert_eq!(DangerLevel::VeryHigh, board.danger_level());
}

#[test]
fn danger_level_almost_dead() {
    let mut board = LocalBoard::new(LocalQueue::default());
    for _ in 0..9 {
        board.cur_piece = Piece::T.try_into().unwrap();
        board.hard_drop();
    }
    assert_eq!(DangerLevel::AlmostDead, board.danger_level());
}
