use crate::game::{
    board::{danger_level::DangerLevel, local_board::LocalBoard},
    queue::local_queue::LocalQueue,
};

#[test]
fn danger_level_empty() {
    let mut board = LocalBoard::new(LocalQueue::new());
    assert_eq!(board.danger_level(), DangerLevel::Empty);
}

#[test]
fn danger_level_low() {
    let mut board = LocalBoard::new(LocalQueue::new());
    board.hard_drop();
    assert_eq!(board.danger_level(), DangerLevel::VeryLow);
}

#[test]
fn danger_level_very_low() {
    todo!()
}

#[test]
fn danger_level_medium() {
    todo!()
}

#[test]
fn danger_level_high() {
    todo!()
}

#[test]
fn danger_level_very_high() {
    todo!()
}

#[test]
fn danger_level_almost_dead() {
    todo!()
}
