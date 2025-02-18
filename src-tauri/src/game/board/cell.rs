use serde::{Deserialize, Serialize};

use crate::game::pieces::Piece;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Cell {
    Empty,
    Full(Piece)
}