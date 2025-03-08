use crate::{
    game::{
        board::{
            cell::Cell,
            local_board::{ClearLinePattern, LocalBoard},
        },
        pieces::Piece,
        queue::local_queue::LocalQueue,
    },
    init_trace::initialize,
};

#[test]
fn pattern_correct_behaviour() {
    initialize();
    let mut board = LocalBoard::new(LocalQueue::default());
    board.clear_pattern = ClearLinePattern::Tetris;
    assert_eq!(ClearLinePattern::Tetris, board.clear_line_pattern());
    assert_eq!(ClearLinePattern::None, board.clear_line_pattern());
}

#[test]
fn single_pattern() {
    initialize();
    let mut board = LocalBoard::new(LocalQueue::default());
    for i in 191..200 {
        board.cells[i] = Cell::Full(Piece::I);
    }
    board.cur_piece = Piece::I.try_into().unwrap();
    board.rotation_clockwise();
    for _ in 0..5 {
        board.move_left();
    }
    board.hard_drop();
    assert_eq!(ClearLinePattern::Single, board.clear_line_pattern());
}

#[test]
fn single_pattern_t() {
    initialize();
    let mut board = LocalBoard::new(LocalQueue::default());
    for i in 191..200 {
        board.cells[i] = Cell::Full(Piece::I);
    }
    board.cur_piece = Piece::T.try_into().unwrap();
    board.rotation_clockwise();
    for _ in 0..5 {
        board.move_left();
    }
    board.hard_drop();
    assert_eq!(ClearLinePattern::Single, board.clear_line_pattern());
}

#[test]
fn double_pattern() {
    initialize();
    let mut board = LocalBoard::new(LocalQueue::default());
    for i in 181..190 {
        board.cells[i] = Cell::Full(Piece::I);
    }
    for i in 191..200 {
        board.cells[i] = Cell::Full(Piece::I);
    }
    board.cur_piece = Piece::I.try_into().unwrap();
    board.rotation_clockwise();
    for _ in 0..5 {
        board.move_left();
    }
    board.hard_drop();
    assert_eq!(ClearLinePattern::Double, board.clear_line_pattern());
}

#[test]
fn double_pattern_t() {
    initialize();
    let mut board = LocalBoard::new(LocalQueue::default());
    for i in 182..190 {
        board.cells[i] = Cell::Full(Piece::I);
    }
    for i in 191..200 {
        board.cells[i] = Cell::Full(Piece::I);
    }
    board.cur_piece = Piece::T.try_into().unwrap();
    board.rotation_clockwise();
    for _ in 0..5 {
        board.move_left();
    }
    board.hard_drop();
    assert_eq!(ClearLinePattern::Double, board.clear_line_pattern());
}

#[test]
fn double_pattern_t_alternative() {
    initialize();
    let mut board = LocalBoard::new(LocalQueue::default());
    board.cur_piece = Piece::T.try_into().unwrap();
    for i in 180..183 {
        board.cells[i] = Cell::Full(Piece::T);
    }
    for i in 186..190 {
        board.cells[i] = Cell::Full(Piece::T);
    }
    for i in 190..194 {
        board.cells[i] = Cell::Full(Piece::T);
    }
    for i in 195..200 {
        board.cells[i] = Cell::Full(Piece::T);
    }
    board.rotation_full();
    board.hard_drop();
    assert_eq!(ClearLinePattern::Double, board.clear_line_pattern());
}

#[test]
fn triple_pattern() {
    initialize();
    let mut board = LocalBoard::new(LocalQueue::default());
    for i in 171..180 {
        board.cells[i] = Cell::Full(Piece::I);
    }
    for i in 181..190 {
        board.cells[i] = Cell::Full(Piece::I);
    }
    for i in 191..200 {
        board.cells[i] = Cell::Full(Piece::I);
    }
    board.cur_piece = Piece::I.try_into().unwrap();
    board.rotation_clockwise();
    for _ in 0..5 {
        board.move_left();
    }
    board.hard_drop();
    assert_eq!(ClearLinePattern::Triple, board.clear_line_pattern());
}

#[test]
fn tetris_pattern() {
    initialize();
    let mut board = LocalBoard::new(LocalQueue::default());
    for i in 161..170 {
        board.cells[i] = Cell::Full(Piece::I);
    }
    for i in 171..180 {
        board.cells[i] = Cell::Full(Piece::I);
    }
    for i in 181..190 {
        board.cells[i] = Cell::Full(Piece::I);
    }
    for i in 191..200 {
        board.cells[i] = Cell::Full(Piece::I);
    }
    board.cur_piece = Piece::I.try_into().unwrap();
    board.rotation_clockwise();
    for _ in 0..5 {
        board.move_left();
    }
    board.hard_drop();
    assert_eq!(ClearLinePattern::Tetris, board.clear_line_pattern());
}

#[test]
/// Extracted directly from Tetris Implementation Guide
fn t_spin_1() {
    initialize();
    let mut board = LocalBoard::new(LocalQueue::default());
    board.cells[172] = Cell::Full(Piece::T);
    board.cells[173] = Cell::Full(Piece::T);
    board.cells[182] = Cell::Full(Piece::T);
    for i in 190..194 {
        board.cells[i] = Cell::Full(Piece::T);
    }
    for i in 195..199 {
        board.cells[i] = Cell::Full(Piece::T);
    }
    board.cur_piece = Piece::T.try_into().unwrap();
    board.rotation_clockwise();
    for _ in 0..20 {
        board.soft_drop();
    }
    board.rotation_clockwise();
    board.next_tick();
    assert_eq!(ClearLinePattern::TSpin, board.clear_line_pattern());
}

#[test]
/// Extracted directly from Tetris Implementation Guide
fn t_spin_2() {
    let mut board = LocalBoard::new(LocalQueue::default());
    for i in 160..166 {
        board.cells[i] = Cell::Full(Piece::T);
    }
    for i in 170..174 {
        board.cells[i] = Cell::Full(Piece::T);
    }
    board.cells[175] = Cell::Full(Piece::T);
    for i in 180..183 {
        board.cells[i] = Cell::Full(Piece::T);
    }
    for i in 190..194 {
        board.cells[i] = Cell::Full(Piece::T);
    }
    board.cur_piece = Piece::T.try_into().unwrap();
    board.rotation_full();
    for _ in 0..10 {
        board.move_right();
    }
    for _ in 0..20 {
        board.soft_drop();
    }
    for _ in 0..10 {
        board.move_left();
    }
    board.rotation_clockwise();
    board.next_tick();
    assert_eq!(ClearLinePattern::TSpin, board.clear_line_pattern());
}

#[test]
fn t_spin_4() {
    let mut board = LocalBoard::new(LocalQueue::default());
    board.cells[152] = Cell::Full(Piece::T);
    board.cells[153] = Cell::Full(Piece::T);
    board.cells[161] = Cell::Full(Piece::T);
    board.cells[162] = Cell::Full(Piece::T);
    board.cells[171] = Cell::Full(Piece::T);
    board.cells[172] = Cell::Full(Piece::T);
    board.cells[174] = Cell::Full(Piece::T);
    board.cells[175] = Cell::Full(Piece::T);
    board.cells[181] = Cell::Full(Piece::T);
    board.cells[185] = Cell::Full(Piece::T);
    board.cells[186] = Cell::Full(Piece::T);
    board.cur_piece = Piece::T.try_into().unwrap();
    for i in 191..193 {
        board.cells[i] = Cell::Full(Piece::T);
    }
    for i in 194..198 {
        board.cells[i] = Cell::Full(Piece::T);
    }
    board.move_right();
    for _ in 0..20 {
        board.soft_drop();
    }
    board.move_left();
    board.rotation_clockwise();
    board.next_tick();
    assert_eq!(ClearLinePattern::TSpin, board.clear_line_pattern());
}

#[test]
/// Extracted directly from Tetris Implementation Guide
fn t_spin_5() {
    initialize();
    let mut board = LocalBoard::new(LocalQueue::default());
    board.cells[172] = Cell::Full(Piece::T);
    board.cells[173] = Cell::Full(Piece::T);
    board.cells[182] = Cell::Full(Piece::T);
    for i in 190..194 {
        board.cells[i] = Cell::Full(Piece::T);
    }
    for i in 195..200 {
        board.cells[i] = Cell::Full(Piece::T);
    }
    board.cur_piece = Piece::T.try_into().unwrap();
    board.rotation_clockwise();
    for _ in 0..20 {
        board.soft_drop();
    }
    board.rotation_clockwise();
    board.next_tick();
    assert_eq!(ClearLinePattern::TSpinSingle, board.clear_line_pattern());
}
