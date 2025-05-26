mod moving_piece_i;
mod moving_piece_j;
mod moving_piece_l;
mod moving_piece_o;
mod moving_piece_s;
pub mod moving_piece_t; // This specific module must be public for the specific functionality for the t-spins
mod moving_piece_z;

use std::any::Any;

use moving_piece_i::MovingPieceI;
use moving_piece_j::MovingPieceJ;
use moving_piece_l::MovingPieceL;
use moving_piece_o::MovingPieceO;
use moving_piece_s::MovingPieceS;
use moving_piece_t::MovingPieceT;
use moving_piece_z::MovingPieceZ;

use crate::game::pieces::Piece;

/// `MovingPiece` trait defines the behavior of a moving piece on the board.
pub trait MovingPiece: Send + Sync + std::fmt::Debug + rotations::Rotations + Any {
    /// Moves the piece down by one row.
    fn move_down(&mut self);
    /// Moves the piece up by one row.
    fn move_up(&mut self);
    /// Moves the piece to the left by one column.
    fn move_left(&mut self);
    /// Moves the piece to the right by one column.
    fn move_right(&mut self);
    /// Gets the coordinates of the sides of the piece that are facing downwards.
    fn get_bottom_facing_sides(&self) -> Vec<(i16, i16)>;
    /// Gets the coordinates of the sides of the piece that are facing rightwards.
    fn get_right_facing_sides(&self) -> Vec<(i16, i16)>;
    /// Gets the coordinates of the sides of the piece that are facing leftwards.
    fn get_left_facing_sides(&self) -> Vec<(i16, i16)>;
    /// Gets the coordinates of all the cells occupied by the piece.
    fn get_coords(&self) -> Vec<(i16, i16)>;
    /// Returns the `Piece` type of the moving piece.
    fn piece(&self) -> Piece;
    /// Rotates the piece clockwise based on the given rotation option.
    fn rotate_clockwise(&mut self, option: RotationOption) {
        match option {
            RotationOption::First => self.first_option_clockwise(),
            RotationOption::Second => self.second_option_clockwise(),
            RotationOption::Third => self.third_option_clockwise(),
            RotationOption::Fourth => self.fourth_option_clockwise(),
            RotationOption::Fifth => self.fifth_option_clockwise(),
        }
    }

    /// Rotates the piece counterclockwise based on the given rotation option.
    fn rotate_counterclockwise(&mut self, option: RotationOption) {
        match option {
            RotationOption::First => self.first_option_counterclockwise(),
            RotationOption::Second => self.second_option_counterclockwise(),
            RotationOption::Third => self.third_option_counterclockwise(),
            RotationOption::Fourth => self.fourth_option_counterclockwise(),
            RotationOption::Fifth => self.fifth_option_counterclockwise(),
        }
    }
    /// Rotates the piece 180 degrees based on the given rotation option.
    fn rotate_full(&mut self, option: RotationOption);

    /// Creates a clone of the `MovingPiece` as a boxed trait object.
    fn clone_box(&self) -> Box<dyn MovingPiece>;

    /// Returns the x coordinate of the piece.
    fn x(&self) -> i16;
    /// Returns the y coordinate of the piece.
    fn y(&self) -> i16;
    /// Returns the orientation of the piece.
    fn orientation(&self) -> Orientation;

    /// Returns a reference to the underlying type as `Any`.
    fn as_any(&self) -> Box<dyn Any>;
}

mod rotations {
    /// `Rotations` trait defines the different rotation options for a moving piece.
    ///
    /// Rules for implementing:
    /// Rule 1: the clockwise rotation must cancel out with its counterclockwise rotation counterpart
    /// Rule 2: all the rotations in one implementation must cancel each other (for example, if the north and east variants add 3 each to x, the south and west ones must substract 3 each).
    /// This rule has only one exception, the T piece doesn't implement some levels with certain rotations
    pub trait Rotations {
        /// Performs the first option for clockwise rotation.
        fn first_option_clockwise(&mut self);
        /// Performs the second option for clockwise rotation.
        fn second_option_clockwise(&mut self);
        /// Performs the third option for clockwise rotation.
        fn third_option_clockwise(&mut self);
        /// Performs the fourth option for clockwise rotation.
        fn fourth_option_clockwise(&mut self);
        /// Performs the fifth option for clockwise rotation.
        fn fifth_option_clockwise(&mut self);

        /// Performs the first option for counterclockwise rotation.
        fn first_option_counterclockwise(&mut self);
        /// Performs the second option for counterclockwise rotation.
        fn second_option_counterclockwise(&mut self);
        /// Performs the third option for counterclockwise rotation.
        fn third_option_counterclockwise(&mut self);
        /// Performs the fourth option for counterclockwise rotation.
        fn fourth_option_counterclockwise(&mut self);
        /// Performs the fifth option for counterclockwise rotation.
        fn fifth_option_counterclockwise(&mut self);
    }
}

/// `Orientation` represents the orientation of a moving piece.
#[derive(Debug, Clone, Copy)]
pub enum Orientation {
    /// The piece is facing north.
    North,
    /// The piece is facing south.
    South,
    /// The piece is facing east.
    East,
    /// The piece is facing west.
    West,
}

/// `RotationOption` represents the different options for rotating a piece.
#[derive(Debug, Clone, Copy)]
pub enum RotationOption {
    /// The first rotation option.
    First,
    /// The second rotation option.
    Second,
    /// The third rotation option.
    Third,
    /// The fourth rotation option.
    Fourth,
    /// The fifth rotation option.
    Fifth,
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
            _ => Err(()),
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
            _ => RotationOption::Fifth,
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
        Orientation::West => Orientation::North,
    }
}

fn change_orientation_counterclockwise(orientation: Orientation) -> Orientation {
    match orientation {
        Orientation::North => Orientation::West,
        Orientation::East => Orientation::North,
        Orientation::South => Orientation::East,
        Orientation::West => Orientation::South,
    }
}

fn change_rotation_full(orientation: Orientation) -> Orientation {
    match orientation {
        Orientation::North => Orientation::South,
        Orientation::East => Orientation::West,
        Orientation::South => Orientation::North,
        Orientation::West => Orientation::East,
    }
}
