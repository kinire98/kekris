use serde::{Deserialize, Serialize};

use crate::game::pieces::Piece;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Cell {
    Empty,
    Full(Piece),
}
impl Cell {
    pub fn string_representation(&self) -> char {
        match self {
            Cell::Empty => 'E',
            Cell::Full(piece) => piece.string_representation()
        }
    }
}
