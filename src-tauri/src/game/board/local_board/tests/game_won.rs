use crate::game::{
    board::{Board, local_board::LocalBoard},
    queue::local_queue::LocalQueue,
};

#[test]
fn game_won_normal_game() {
    let board = LocalBoard::new(LocalQueue::default());
    assert!(!board.game_won(|_game_over, _lines| { false }));
}

#[test]
fn game_won_test_40_lines_not_won_yet() {
    let mut board = LocalBoard::new(LocalQueue::default());
    board.lines_cleared = 39;
    assert!(!board.game_won(|game_over, lines| { !game_over && lines >= 40 }))
}

#[test]
fn game_won_test_40_lines_won() {
    let mut board = LocalBoard::new(LocalQueue::default());
    board.lines_cleared = 41;
    assert!(board.game_won(|game_over, lines| { !game_over && lines >= 40 }))
}

#[test]
fn game_won_2_minutes_not_passed() {
    let board = LocalBoard::new(LocalQueue::default());
    let seconds = 119;
    assert!(!board.game_won(|game_over, _lines| { !game_over && seconds >= 120 }));
}

#[test]
fn game_won_2_minutes_passed() {
    let board = LocalBoard::new(LocalQueue::default());
    let seconds = 120;
    assert!(board.game_won(|game_over, _lines| { !game_over && seconds >= 120 }));
}
