use crate::game::pieces::Piece;

pub enum Cell {
    Empty,
    Full(Piece)
}