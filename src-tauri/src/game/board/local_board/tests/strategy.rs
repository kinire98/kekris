use crate::game::{
    board::local_board::LocalBoard, queue::local_queue::LocalQueue, strategy::Strategy,
};

#[test]
fn correct_change() {
    let mut board = LocalBoard::new(LocalQueue::default());
    board.change_strategy(Strategy::Elimination);
    assert_eq!(Strategy::Elimination, board.strategy);
}
