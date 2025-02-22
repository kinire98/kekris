use crate::game::pieces::Piece;

use super::{rotations::Rotations, MovingPiece, Orientation};

const START_X: i16 = 4;
const START_Y: i16 = -2;

#[derive(Debug, Clone, Copy)]
pub struct MovingPieceS {
    x: i16,
    y: i16,
    orientation: Orientation
}

impl MovingPieceS {
    pub fn new() -> Self {
        MovingPieceS { x: START_X, y: START_Y, orientation: Orientation::North }
    }
}

impl MovingPiece for MovingPieceS {
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
            Orientation::North | Orientation::South => [(self.x, self.y + 1), (self.x + 1, self.y + 1), (self.x + 2, self.y)].into(),
            Orientation::East | Orientation::West => [(self.x, self.y + 1), (self.x + 1, self.y + 2)].into()
        }
    }

    fn get_right_facing_sides(&self) -> Vec<(i16, i16)> {
        match self.orientation {
            Orientation::North | Orientation::South => [(self.x + 2, self.y), (self.x + 1, self.y + 1)].into(),
            Orientation::East | Orientation::West => [(self.x, self.y), (self.x + 1, self.y + 1), (self.x + 1, self.x + 2)].into()
        }
    }

    fn get_left_facing_sides(&self) -> Vec<(i16, i16)> {
        match self.orientation {
            Orientation::North | Orientation::South => [(self.x , self.y + 1), (self.x + 1, self.y)].into(),
            Orientation::East | Orientation::West => [(self.x, self.y), (self.x, self.y + 1), (self.x + 1, self.y + 2)].into()
        }
    }

    fn get_coords(&self) -> Vec<(i16, i16)> {
        match self.orientation {
            Orientation::North | Orientation::South => [(self.x + 1, self.y), (self.x + 2, self.y), (self.x, self.y + 1), (self.x + 1, self.y + 1)].into(),
            Orientation::East | Orientation::West => [(self.x, self.y), (self.x, self.y + 1), (self.x + 1, self.y + 1), (self.x + 1, self.y + 2)].into()
        }
    }

    fn piece(&self) -> Piece {
        Piece::S
    }

    fn rotate_full(&mut self, _option: super::RotationOption) {}

    fn clone_box(&self) -> Box<dyn MovingPiece> {
        Box::new(*self)
    }

}

impl Rotations for MovingPieceS {
    fn first_option_clockwise(&mut self) {
        match self.orientation {
            Orientation::North => {
                self.x += 1;
            },
            Orientation::East => {
                self.x -= 1;
                self.y += 1;
            },
            Orientation::South => {
                self.x -= 1
            },
            Orientation::West => {}
        }
        self.orientation = super::change_orientation_clockwise(self.orientation);
    }

    fn second_option_clockwise(&mut self) {
        match self.orientation {
            Orientation::North => {},
            Orientation::East => {
                self.y += 1;
            },
            Orientation::South => {
                self.x += 1;
                self.y -= 1;
            },
            Orientation::West => {
                self.x += 1;
            }
        }
        self.orientation = super::change_orientation_clockwise(self.orientation);
    }

    fn third_option_clockwise(&mut self) {
        match self.orientation {
            Orientation::North => {
                self.y -= 1;
            },
            Orientation::East => {
                self.y += 2;
            },
            Orientation::South => {
                self.x += 1;
                self.y -= 2;
            },
            Orientation::West => {
                self.x -= 1;
                self.y += 1;
            }
        }
        self.orientation = super::change_orientation_clockwise(self.orientation);
    }

    fn fourth_option_clockwise(&mut self) {
        match self.orientation {
            Orientation::North => {
                self.x += 1;
                self.y += 2;
            },
            Orientation::East => {
                self.x -= 1;
                self.y -= 1;
            },
            Orientation::South => {
                self.y += 1;
            },
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
            },
            Orientation::East => {
                self.y -= 1;
            },
            Orientation::South => {
                self.x += 1;
                self.y += 1;
            },
            Orientation::West => {
                self.x -= 1;
                self.y -= 2;
            }
        }
        self.orientation = super::change_orientation_clockwise(self.orientation);
    }

    fn first_option_counterclockwise(&mut self) {
        match self.orientation {
            Orientation::North => {},
            Orientation::East => {
                self.x -= 1;
            },
            Orientation::South => {
                self.x += 1;
                self.y -= 1;
            },
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
            },
            Orientation::East => {},
            Orientation::South => {
                self.y -= 1;
            },
            Orientation::West => {
                self.x -= 1;
                self.y += 1;
            }
        }
        self.orientation = super::change_orientation_counterclockwise(self.orientation)
    }

    fn third_option_counterclockwise(&mut self) {
        match self.orientation {
            Orientation::North => {
                self.x += 1;
                self.y -= 1;
            },
            Orientation::East => {
                self.y += 1;
            },
            Orientation::South => {
                self.y -= 2;
            },
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
                self.y += 2;
            },
            Orientation::East => {
                self.x -= 1;
                self.y -= 2;
            },
            Orientation::South => {
                self.x += 1;
                self.y += 1;
            },
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
            },
            Orientation::East => {
                self.y -= 2;
            },
            Orientation::South => {
                self.y += 1;
            },
            Orientation::West => {
                self.x -= 1;
                self.y -= 1;
            }
        }
        self.orientation = super::change_orientation_counterclockwise(self.orientation);
    }
}