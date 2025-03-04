use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Hash, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Piece {
    T,
    O,
    I,
    L,
    J,
    S,
    Z,
    Ghost,
    Trash,
}
// E -> Empty
// C -> Clear
// G -> Ghost
// O -> yellow
// I -> cyan
// T -> Purple
// L -> orange
// J -> blue
// S -> green
// Z -> red
impl Piece {
    pub fn string_representation(&self) -> char {
        match self {
            Piece::Ghost => 'G',
            Piece::I => 'I',
            Piece::J => 'J',
            Piece::L => 'L',
            Piece::O => 'O',
            Piece::S => 'S',
            Piece::T => 'T',
            Piece::Trash => 'R',
            Piece::Z => 'Z',
        }
    }
}
