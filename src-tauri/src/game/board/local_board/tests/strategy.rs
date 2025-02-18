use crate::game::{board::{local_board::LocalBoard, Board}, queue::{local_queue::LocalQueue, Queue}, strategy::Strategy};

#[test]
fn correct_change() {
    let mut board = LocalBoard::new(LocalQueue::new());
    board.change_strategy(Strategy::Elimination);
    assert_eq!(Strategy::Elimination, board.strategy());
}