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
}
