mod moving_piece_i;
mod moving_piece_j;
mod moving_piece_l;
mod moving_piece_o;
mod moving_piece_s;
mod moving_piece_t;
mod moving_piece_z;


use moving_piece_i::MovingPieceI;
use moving_piece_j::MovingPieceJ;
use moving_piece_l::MovingPieceL;
use moving_piece_o::MovingPieceO;
use moving_piece_s::MovingPieceS;
use moving_piece_t::MovingPieceT;
use moving_piece_z::MovingPieceZ;

use crate::game::pieces::Piece;

pub trait MovingPiece : Send + Sync + std::fmt::Debug + rotations::Rotations {
    fn move_down(&mut self);
    fn move_up(&mut self);
    fn move_left(&mut self);
    fn move_right(&mut self);
    fn get_bottom_facing_sides(&self) -> Vec<(i16, i16)>;
    fn get_right_facing_sides(&self) -> Vec<(i16, i16)>;
    fn get_left_facing_sides(&self) -> Vec<(i16, i16)>;
    fn get_coords(&self) -> Vec<(i16, i16)>;
    fn piece(&self) -> Piece;
    fn rotate_clockwise(&mut self, option: RotationOption) {
        match option {
            RotationOption::First => self.first_option_clockwise(),
            RotationOption::Second => self.second_option_clockwise(),
            RotationOption::Third => self.third_option_clockwise(),
            RotationOption::Fourth => self.fourth_option_clockwise(),
            RotationOption::Fifth => self.fifth_option_clockwise(),
        }
    }

    fn rotate_counterclockwise(&mut self, option: RotationOption) {
        match option {
            RotationOption::First => self.first_option_counterclockwise(),
            RotationOption::Second => self.second_option_counterclockwise(),
            RotationOption::Third => self.third_option_counterclockwise(),
            RotationOption::Fourth => self.fourth_option_counterclockwise(),
            RotationOption::Fifth => self.fifth_option_counterclockwise(),
        }
    }
    fn rotate_full(&mut self, option: RotationOption);

    fn clone_box(&self) -> Box<dyn MovingPiece>;

}

mod rotations {
    /// Rules for implementing:
    /// Rule 1: the clockwise rotation must cancel out with its counterclockwise rotation counterpart 
    /// Rule 2: all the rotations in one implementation must cancel each other (for example, if the north and east variants add 3 each to x, the south and west ones must substract 3 each).
    /// This rule has only one exception, the T piece doesn't implement some levels with certain rotations
    pub trait Rotations {

        fn first_option_clockwise(&mut self);
        fn second_option_clockwise(&mut self);
        fn third_option_clockwise(&mut self);
        fn fourth_option_clockwise(&mut self);
        fn fifth_option_clockwise(&mut self);

        fn first_option_counterclockwise(&mut self);
        fn second_option_counterclockwise(&mut self);
        fn third_option_counterclockwise(&mut self);
        fn fourth_option_counterclockwise(&mut self);
        fn fifth_option_counterclockwise(&mut self);
    }
}


#[derive(Debug, Clone, Copy)]
pub enum Orientation {
    North,
    South, 
    East,
    West
}

#[derive(Debug, Clone, Copy)]
pub enum RotationOption {
    First,
    Second,
    Third,
    Fourth,
    Fifth
}

impl TryFrom<Piece> for Box<dyn MovingPiece> {
    type Error = ();
    
    fn try_from(value: Piece) -> Result<Self, Self::Error> {
        match value {
            Piece::I => Ok(Box::new(MovingPieceI::new())),
            Piece::J => Ok(Box::new(MovingPieceJ::new())),
            Piece::L => Ok(Box::new(MovingPieceL::new())),
            Piece::O => Ok(Box::new(MovingPieceO::new())),
            Piece::S => Ok(Box::new(MovingPieceS::new())),
            Piece::T => Ok(Box::new(MovingPieceT::new())),
            Piece::Z => Ok(Box::new(MovingPieceZ::new())),
            _ => Err(())
        }
    }
}

impl From<u8> for RotationOption {
    fn from(value: u8) -> Self {
        match value {
            0..=1 => RotationOption::First,
            2 => RotationOption::Second,
            3 => RotationOption::Third,
            4 => RotationOption::Fourth,
            _ => RotationOption::Fifth
        }
    }
}

impl Clone for Box<dyn MovingPiece> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

fn change_orientation_clockwise(orientation: Orientation) -> Orientation {
    match orientation {
        Orientation::North => Orientation::East,
        Orientation::East => Orientation::South,
        Orientation::South => Orientation::West,
        Orientation::West => Orientation::North
    }
}


fn change_orientation_counterclockwise(orientation: Orientation) -> Orientation {
    match orientation {
        Orientation::North => Orientation::West,
        Orientation::East => Orientation::North,
        Orientation::South => Orientation::East,
        Orientation::West => Orientation::South
    }
}

fn change_rotation_full(orientation: Orientation) -> Orientation {
    match orientation {
        Orientation::North => Orientation::South,
        Orientation::East => Orientation::West,
        Orientation::South => Orientation::North,
        Orientation::West => Orientation::East
    }
}