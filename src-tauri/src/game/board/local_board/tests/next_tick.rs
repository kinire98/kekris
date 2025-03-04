use crate::game::{
    board::{
        local_board::{moving_piece::MovingPiece, LocalBoard},
        Board,
    },
    pieces::Piece,
    queue::local_queue::LocalQueue,
};

#[test]
fn next_tick_posible() {
    let mut board = LocalBoard::new(LocalQueue::new());
    board.rotation_counterclockwise();
    println!("{}", board.board_state());
}
