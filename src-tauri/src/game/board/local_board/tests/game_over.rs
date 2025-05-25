use crate::game::{
    board::{Board, cell::Cell, local_board::LocalBoard},
    pieces::Piece,
    queue::local_queue::LocalQueue,
};

#[test]
fn game_over_block_out() {
    let mut board = LocalBoard::new(LocalQueue::default());
    for i in 180..200 {
        board.buffer[i] = Cell::Full(Piece::O);
    }
    assert!(board.game_over());
}

#[test]
fn game_over_lock_out() {
    let mut board = LocalBoard::new(LocalQueue::default());
    for i in 1..200 {
        if i % 10 == 0 {
            continue;
        }
        board.cells[i] = Cell::Full(Piece::O);
    }
    board.next_tick();
    board.next_tick();
    assert!(board.game_over());
}

#[test]
fn game_over_top_out() {
    let mut board = LocalBoard::new(LocalQueue::default());
    board.cells[181] = Cell::Full(Piece::I);
    board.trash_lines_queue = vec![(38, 0)];
    board.hard_drop();
    println!("{:?}\n{:?}", board.buffer, board.cells);
    println!("{}", board.top_out);
    assert!(board.game_over());
}
