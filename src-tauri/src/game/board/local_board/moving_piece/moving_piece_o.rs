use crate::game::pieces::Piece;

use super::{rotations::Rotations, MovingPiece, Orientation};

const START_X: i16 = 5;
const START_Y: i16 = -2;

#[derive(Debug, Clone, Copy)]
pub struct MovingPieceO {
    x: i16,
    y: i16,
    orientation: Orientation
}

impl MovingPieceO {
    pub fn new() -> Self {
        MovingPieceO { x: START_X, y: START_Y, orientation: Orientation::North }
    }
}

impl MovingPiece for MovingPieceO {
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
        [(self.x, self.y + 1), (self.x + 1, self.y + 1)].into()
    }

    fn get_right_facing_sides(&self) -> Vec<(i16, i16)> {
        [(self.x + 1, self.y), (self.x + 1, self.y + 1)].into()
    }

    fn get_left_facing_sides(&self) -> Vec<(i16, i16)> {
        [(self.x, self.y), (self.x, self.y + 1)].into()
    }

    fn get_coords(&self) -> Vec<(i16, i16)> {
        [(self.x, self.y), (self.x + 1, self.y), (self.x, self.y + 1), (self.x + 1, self.y + 1)].into()
    }

    fn piece(&self) -> Piece {
        Piece::O
    }

    fn rotate_full(&mut self, _option: super::RotationOption) {}

    fn clone_box(&self) -> Box<dyn MovingPiece> {
        Box::new(*self)
    }
}

impl Rotations for MovingPieceO {
    fn first_option_clockwise(&mut self) {
        self.orientation = super::change_orientation_clockwise(self.orientation);
    }

    fn second_option_clockwise(&mut self) {
        self.orientation = super::change_orientation_clockwise(self.orientation);
    }

    fn third_option_clockwise(&mut self) {
        self.orientation = super::change_orientation_clockwise(self.orientation);
    }

    fn fourth_option_clockwise(&mut self) {
        self.orientation = super::change_orientation_clockwise(self.orientation);
    }

    fn fifth_option_clockwise(&mut self) {
        self.orientation = super::change_orientation_clockwise(self.orientation);
    }

    fn first_option_counterclockwise(&mut self) {
        self.orientation = super::change_orientation_counterclockwise(self.orientation);
    }

    fn second_option_counterclockwise(&mut self) {
        self.orientation = super::change_orientation_counterclockwise(self.orientation);
    }

    fn third_option_counterclockwise(&mut self) {
        self.orientation = super::change_orientation_counterclockwise(self.orientation);
    }

    fn fourth_option_counterclockwise(&mut self) {
        self.orientation = super::change_orientation_counterclockwise(self.orientation);
    }

    fn fifth_option_counterclockwise(&mut self) {
        self.orientation = super::change_orientation_counterclockwise(self.orientation);
    }
}