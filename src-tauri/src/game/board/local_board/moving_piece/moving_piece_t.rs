use crate::game::pieces::Piece;

use super::{rotations::Rotations, MovingPiece, Orientation, RotationOption};

const START_X: i16 = 3;
const START_Y: i16 = -2;

#[derive(Debug, Clone, Copy)]
pub struct MovingPieceT {
    x: i16,
    y: i16,
    orientation: Orientation,
}

impl MovingPieceT {
    pub fn new() -> Self {
        MovingPieceT {
            x: START_X,
            y: START_Y,
            orientation: Orientation::North,
        }
    }
}

impl MovingPiece for MovingPieceT {
    fn move_down(&mut self) {
        self.y += 1;
    }

    fn move_up(&mut self) {
        self.y -= 1;
    }

    fn move_left(&mut self) {
        self.x -= 1;
    }

    fn move_right(&mut self) {
        self.x += 1;
    }

    fn get_bottom_facing_sides(&self) -> Vec<(i16, i16)> {
        match self.orientation {
            Orientation::North => [
                (self.x, self.y + 1),
                (self.x + 1, self.y + 1),
                (self.x + 2, self.y + 1),
            ]
            .into(),
            Orientation::East => [(self.x + 1, self.y + 1), (self.x, self.y + 2)].into(),
            Orientation::South => [
                (self.x, self.y),
                (self.x + 1, self.y + 1),
                (self.x + 2, self.y),
            ]
            .into(),
            Orientation::West => [(self.x, self.y + 1), (self.x + 1, self.y + 2)].into(),
        }
    }

    fn get_right_facing_sides(&self) -> Vec<(i16, i16)> {
        match self.orientation {
            Orientation::North => [(self.x + 1, self.y), (self.x + 2, self.y + 1)].into(),
            Orientation::East => [
                (self.x, self.y),
                (self.x + 1, self.y + 1),
                (self.x, self.y + 2),
            ]
            .into(),
            Orientation::South => [(self.x + 1, self.y + 1), (self.x + 2, self.y)].into(),
            Orientation::West => [
                (self.x + 1, self.y),
                (self.x + 1, self.y + 1),
                (self.x + 1, self.y + 2),
            ]
            .into(),
        }
    }

    fn get_left_facing_sides(&self) -> Vec<(i16, i16)> {
        match self.orientation {
            Orientation::North => [(self.x + 1, self.y), (self.x, self.y + 1)].into(),
            Orientation::East => {
                [(self.x, self.y), (self.x, self.y + 1), (self.x, self.y + 2)].into()
            }
            Orientation::South => [(self.x, self.y), (self.x + 1, self.y + 1)].into(),
            Orientation::West => [
                (self.x + 1, self.y),
                (self.x, self.y + 1),
                (self.x + 1, self.y + 2),
            ]
            .into(),
        }
    }

    fn get_coords(&self) -> Vec<(i16, i16)> {
        match self.orientation {
            Orientation::North => [
                (self.x + 1, self.y),
                (self.x, self.y + 1),
                (self.x + 1, self.y + 1),
                (self.x + 2, self.y + 1),
            ]
            .into(),
            Orientation::East => [
                (self.x, self.y),
                (self.x, self.y + 1),
                (self.x + 1, self.y + 1),
                (self.x, self.y + 2),
            ]
            .into(),
            Orientation::South => [
                (self.x, self.y),
                (self.x + 1, self.y),
                (self.x + 2, self.y),
                (self.x + 1, self.y + 1),
            ]
            .into(),
            Orientation::West => [
                (self.x + 1, self.y),
                (self.x, self.y + 1),
                (self.x + 1, self.y + 1),
                (self.x + 1, self.y + 2),
            ]
            .into(),
        }
    }

    fn piece(&self) -> Piece {
        Piece::T
    }

    fn rotate_full(&mut self, option: RotationOption) {
        if let RotationOption::First = option {
            match self.orientation {
                Orientation::North => {
                    self.y += 1;
                }
                Orientation::South => {
                    self.y -= 1;
                }
                Orientation::East => {
                    self.x -= 1;
                }
                Orientation::West => {
                    self.x += 1;
                }
            }
        }
        self.orientation = super::change_rotation_full(self.orientation);
    }

    fn clone_box(&self) -> Box<dyn MovingPiece> {
        Box::new(*self)
    }

    fn x(&self) -> i16 {
        self.x
    }

    fn y(&self) -> i16 {
        self.y
    }

    fn orientation(&self) -> Orientation {
        self.orientation
    }
}

impl Rotations for MovingPieceT {
    fn first_option_clockwise(&mut self) {
        match self.orientation {
            Orientation::North => {
                self.x += 1;
            }
            Orientation::East => {
                self.x -= 1;
                self.y += 1;
            }
            Orientation::South => {
                self.y -= 1;
            }
            Orientation::West => {}
        }
        self.orientation = super::change_orientation_clockwise(self.orientation);
    }

    fn second_option_clockwise(&mut self) {
        match self.orientation {
            Orientation::North => {}
            Orientation::East => {
                self.y += 1;
            }
            Orientation::South => {
                self.x += 1;
                self.y -= 1;
            }
            Orientation::West => {
                self.x -= 1;
            }
        }
        self.orientation = super::change_orientation_clockwise(self.orientation);
    }

    fn third_option_clockwise(&mut self) {
        match self.orientation {
            Orientation::North => {
                self.y -= 1;
            }
            Orientation::East => {
                self.y += 2;
            }
            Orientation::South => {
                // * In the implementation of the T piece with the third rotation point from south orientation rotations aren't implemented
                // * This simply recalls the inferior level
                self.fourth_option_clockwise();
                // * Correction of orientation
                self.orientation = super::change_orientation_counterclockwise(self.orientation);
            }
            Orientation::West => {
                self.x -= 1;
                self.y += 1
            }
        }
        self.orientation = super::change_orientation_clockwise(self.orientation);
    }

    fn fourth_option_clockwise(&mut self) {
        match self.orientation {
            Orientation::North => {
                // * In the implementation of the T piece with the fourth rotation point from north orientation rotations aren't implemented
                // * This simply recalls the inferior level
                self.fifth_option_clockwise();
                // * Correction of orientation
                self.orientation = super::change_orientation_counterclockwise(self.orientation);
            }
            Orientation::East => {
                self.x -= 1;
                self.y += 1;
            }
            Orientation::South => {
                self.y += 1;
            }
            Orientation::West => {
                self.y -= 2;
            }
        }
        self.orientation = super::change_orientation_clockwise(self.orientation);
    }

    fn fifth_option_clockwise(&mut self) {
        match self.orientation {
            Orientation::North => {
                self.y += 2;
            }
            Orientation::East => {
                self.y -= 1;
            }
            Orientation::South => {
                self.x += 1;
                self.y += 1;
            }
            Orientation::West => {
                self.x -= 1;
                self.y -= 2;
            }
        }
        self.orientation = super::change_orientation_clockwise(self.orientation);
    }

    fn first_option_counterclockwise(&mut self) {
        match self.orientation {
            Orientation::North => {}
            Orientation::East => {
                self.x -= 1;
            }
            Orientation::South => {
                self.x += 1;
                self.y -= 1;
            }
            Orientation::West => {
                self.y += 1;
            }
        }
        self.orientation = super::change_orientation_counterclockwise(self.orientation);
    }

    fn second_option_counterclockwise(&mut self) {
        match self.orientation {
            Orientation::North => {
                self.x += 1;
            }
            Orientation::East => {}
            Orientation::South => {
                self.y -= 1;
            }
            Orientation::West => {
                self.x -= 1;
                self.y += 1;
            }
        }
        self.orientation = super::change_orientation_counterclockwise(self.orientation);
    }

    fn third_option_counterclockwise(&mut self) {
        match self.orientation {
            Orientation::North => {
                self.x += 1;
                self.y -= 1;
            }
            Orientation::East => {
                self.y += 1;
            }
            Orientation::South => {
                self.fourth_option_counterclockwise();
                self.orientation = super::change_orientation_clockwise(self.orientation);
            }
            Orientation::West => {
                self.x -= 1;
                self.y += 2;
            }
        }
        self.orientation = super::change_orientation_counterclockwise(self.orientation);
    }

    fn fourth_option_counterclockwise(&mut self) {
        match self.orientation {
            Orientation::North => {
                self.fifth_option_counterclockwise();
                self.orientation = super::change_orientation_clockwise(self.orientation);
            }
            Orientation::East => {
                self.x -= 1;
                self.y -= 2;
            }
            Orientation::South => {
                self.x += 1;
                self.y += 1;
            }
            Orientation::West => {
                self.y -= 1;
            }
        }
        self.orientation = super::change_orientation_counterclockwise(self.orientation);
    }

    fn fifth_option_counterclockwise(&mut self) {
        match self.orientation {
            Orientation::North => {
                self.x += 1;
                self.y += 2;
            }
            Orientation::East => {
                self.y -= 2;
            }
            Orientation::South => {
                self.y += 1;
            }
            Orientation::West => {
                self.x -= 1;
                self.y -= 1;
            }
        }
        self.orientation = super::change_orientation_counterclockwise(self.orientation);
    }
}
