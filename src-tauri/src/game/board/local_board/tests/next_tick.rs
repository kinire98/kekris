use crate::game::{
    board::{Board, local_board::LocalBoard},
    queue::local_queue::LocalQueue,
};

#[test]
fn next_tick_posible() {
    let mut board = LocalBoard::new(LocalQueue::default());
    board.rotation_counterclockwise();
    println!("{}", board.board_state());
}
