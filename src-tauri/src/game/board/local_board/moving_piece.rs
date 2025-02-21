use crate::game::pieces::Piece;


const START_Y: i16 = -2;
const START_X: i16 = 4;
const START_X_O: i16 = 5;

const O_VALUE: i16 = 1;
const I_VALUE: i16 = 3;
const GENERAL_PIECE_VALUE: i16 = 2;
const I_SECOND_VALUE: i16 = 0;
const GENERAL_PIECE_SECOND_VALUE: i16 = 1;

#[derive(Debug, Clone, Copy)]
pub struct MovingPiece {
    piece: Piece,
    orientation: Orientation,
    x: i16,
    y: i16
}

#[derive(Debug, Clone, Copy)]
pub enum Orientation {
    North,
    South,
    East,
    West
}

impl MovingPiece {
    pub fn new(piece: Piece) -> Self {
        match piece {
            Piece::Ghost | Piece::Trash => panic!("Invalid piece"),
            Piece::O => MovingPiece { piece, orientation: Orientation::North, x: START_X_O, y: START_Y },
            _ => MovingPiece { piece, orientation: Orientation::North, x: START_X, y: START_Y }
        }
    }

    pub fn move_down(&mut self) {
        self.y += 1;
    }

    pub fn move_up(&mut self) {
        self.y -= 1;
    }

    pub fn move_right(&mut self) {
        self.x += 1;
    }

    pub fn move_left(&mut self) {
        self.x -= 1;
    }

    pub fn piece(&self) -> Piece {
        self.piece
    }

    pub fn get_right_facing_sides(&self) -> Vec<(i16, i16)> {
        match self.piece {
            Piece::I => self.right_I(),
            Piece::O => self.right_O(),
            Piece::S => self.right_S(),
            Piece::Z => self.right_Z(),
            Piece::J => self.right_J(),
            Piece::L => self.right_L(),
            Piece::T => self.right_T(),
            _ => panic!("Shouldn't even arrive here")
        }
    }

    fn right_I(&self) -> Vec<(i16, i16)> { // The additions are put as magic numbers because they are inmutable and have no intrinsic meaning
        match self.orientation {
            Orientation::North | Orientation::South => [(self.x + 3, self.y)].into(),
            Orientation::East | Orientation::West => [(self.x, self.y), (self.x, self.y + 1), (self.x, self.y + 2), (self.x, self.y + 3)].into()
        }
    }

    fn right_O(&self) -> Vec<(i16, i16)> {
        [(self.x + 1, self.y), (self.x + 1, self.y + 1)].into()
    }

    fn right_S(&self) -> Vec<(i16, i16)> {
        match self.orientation {
            Orientation::North | Orientation::South => [(self.x + 2, self.y), (self.x + 1, self.y + 1)].into(),
            Orientation::East | Orientation::West => [(self.x, self.y), (self.x + 1, self.y + 1), (self.x + 1, self.x + 2)].into()
        }
    }

    fn right_Z(&self) -> Vec<(i16, i16)> {
        match self.orientation { 
            Orientation::North | Orientation::South => [(self.x + 1, self.y), (self.x + 2, self.y + 1)].into(),
            Orientation::East | Orientation::West => [(self.x + 1, self.y), (self.x + 1, self.y + 1), (self.x, self.y + 2)].into()
        }
    }

    fn right_J(&self) -> Vec<(i16, i16)> {
        match self.orientation {
            Orientation::North => [(self.x, self.y), (self.x + 2, self.y + 1)].into(),
            Orientation::East => [(self.x + 1, self.y), (self.x, self.y + 1), (self.x, self.y + 2)].into(),
            Orientation::South => [(self.x + 2, self.y), (self.x + 2, self.y + 1)].into(),
            Orientation::West => [(self.x + 1, self.y), (self.x + 1, self.y + 1), (self.x + 1, self.y + 2)].into()
        }
    }

    fn right_L(&self) -> Vec<(i16, i16)> {
        match self.orientation {
            Orientation::North => [(self.x + 2, self.y), (self.x + 2, self.y + 1)].into(),
            Orientation::East => [(self.x, self.y), (self.x, self.y + 1), (self.x + 1, self.y + 2)].into(),
            Orientation::South => [(self.x + 2, self.y), (self.x, self.y + 1)].into(),
            Orientation::West => [(self.x + 1, self.y), (self.x + 1, self.y + 1), (self.x + 1, self.y + 2)].into()
        }
    }

    fn right_T(&self) -> Vec<(i16, i16)> {
        match self.orientation {
            Orientation::North => [(self.x + 1, self.y), (self.x + 2, self.y + 1)].into(),
            Orientation::East => [(self.x, self.y), (self.x + 1, self.y + 1), (self.x, self.y + 2)].into(),
            Orientation::South => [(self.x + 1, self.y + 1), (self.x + 2, self.y)].into(),
            Orientation::West => [(self.x + 1, self.y), (self.x + 1, self.y + 1), (self.x + 1, self.y + 2)].into()
        }
    }
    

    pub fn get_bottom_facing_sides(&self) -> Vec<(i16, i16)> {
        match self.piece {
            Piece::I => self.bottom_I(),
            Piece::O => self.bottom_O(),
            Piece::S => self.bottom_S(),
            Piece::Z => self.bottom_Z(),
            Piece::J => self.bottom_J(),
            Piece::L => self.bottom_L(),
            Piece::T => self.bottom_T(),
            _ => panic!("Shouldn't even arrive here")
        }
    }

    fn bottom_I(&self) -> Vec<(i16, i16)> {
        match self.orientation {
            Orientation::North |  Orientation::South => [(self.x, self.y), (self.x + 1, self.y), (self.x + 2, self.y), (self.x + 3, self.y)].into(),
            Orientation::East | Orientation::West => [(self.x, self.y + 3)].into()
        }
    }

    fn bottom_O(&self) -> Vec<(i16, i16)> {
        [(self.x, self.y + 1), (self.x + 1, self.y + 1)].into()
    }

    fn bottom_S(&self) -> Vec<(i16, i16)> {
        match self.orientation {
            Orientation::North | Orientation::South => [(self.x, self.y + 1), (self.x + 1, self.y + 1), (self.x + 2, self.y)].into(),
            Orientation::East | Orientation::West => [(self.x, self.y + 1), (self.x + 1, self.y + 2)].into()
        }
    }

    fn bottom_Z(&self) -> Vec<(i16, i16)> {
        match self.orientation {
            Orientation::North | Orientation::South => [(self.x, self.y), (self.x + 1, self.y + 1), (self.x + 2, self.y + 1)].into(),
            Orientation::East | Orientation::West => [(self.x + 1, self.y + 1), (self.x, self.y + 2)].into()
        }
    }

    fn bottom_J(&self) -> Vec<(i16, i16)> {
        match self.orientation {
            Orientation::North => [(self.x, self.y + 1), (self.x + 1, self.y + 1), (self.x + 2, self.y + 1)].into(),
            Orientation::East => [(self.x + 1, self.y), (self.x, self.y + 2)].into(),
            Orientation::South => [(self.x, self.y), (self.x + 1, self.y), (self.x + 2, self.y + 1)].into(),
            Orientation::West => [(self.x, self.y + 2), (self.x + 1, self.y + 2)].into()
        }
    }

    fn bottom_L(&self) -> Vec<(i16, i16)> {
        match self.orientation {
            Orientation::North => [(self.x, self.y + 1), (self.x + 1, self.y + 1), (self.x + 2, self.y + 1)].into(),
            Orientation::East => [(self.x, self.y + 2), (self.x + 1, self.y + 2)].into(),
            Orientation::South => [(self.x, self.y + 1), (self.x + 1, self.y), (self.x + 2, self.y)].into(),
            Orientation::West => [(self.x, self.y), (self.x + 1, self.y + 2)].into()
        }
    }

    fn bottom_T(&self) -> Vec<(i16, i16)> {
        match self.orientation {
            Orientation::North => [(self.x, self.y + 1), (self.x + 1, self.y + 1), (self.x + 2, self.y + 1)].into(),
            Orientation::East => [(self.x + 1, self.y + 1), (self.x, self.y + 2)].into(),
            Orientation::South => [(self.x, self.y), (self.x + 1, self.y + 1), (self.x + 2, self.y)].into(),
            Orientation::West => [(self.x, self.y + 1), (self.x + 1, self.y + 2)].into()
        }
    }


    pub fn get_left_facing_sides(&self) -> Vec<(i16, i16)> {
        match self.piece {
            Piece::I => self.left_I(),
            Piece::O => self.left_O(),
            Piece::S => self.left_S(),
            Piece::Z => self.left_Z(),
            Piece::J => self.left_J(),
            Piece::L => self.left_L(),
            Piece::T => self.left_T(),
            _ => panic!("Shouldn't even arrive here")
        }
    }

    fn left_I(&self) -> Vec<(i16, i16)> {
        match self.orientation {
            Orientation::North | Orientation::South => [(self.x, self.y)].into(),
            Orientation::East | Orientation::West => [(self.x, self.y), (self.x, self.y + 1), (self.x, self.y + 2), (self.x, self.y + 3)].into()
        }
    }

    fn left_O(&self) -> Vec<(i16, i16)> {
        [(self.x, self.y), (self.x, self.y + 1)].into()
    }

    fn left_S(&self) -> Vec<(i16, i16)> {
        match self.orientation {
            Orientation::North | Orientation::South => [(self.x , self.y + 1), (self.x + 1, self.y)].into(),
            Orientation::East | Orientation::West => [(self.x, self.y), (self.x, self.y + 1), (self.x + 1, self.y + 2)].into()
        }
    }

    fn left_Z(&self) -> Vec<(i16, i16)> {
        match self.orientation {
            Orientation::North | Orientation::South => [(self.x, self.y), (self.x + 1, self.y + 1)].into(),
            Orientation::East | Orientation::West => [(self.x + 1, self.y), (self.x, self.y + 1), (self.x, self.y + 2)].into()
        }
    }

    fn left_J(&self) -> Vec<(i16, i16)> {
        match self.orientation {
            Orientation::North => [(self.x, self.y), (self.x, self.y + 1)].into(),
            Orientation::East => [(self.x, self.y), (self.x, self.y + 1), (self.x, self.y + 2)].into(),
            Orientation::South => [(self.x, self.y), (self.x + 2, self.y + 1)].into(),
            Orientation::West => [(self.x + 1, self.y), (self.x + 1, self.y + 1), (self.x, self.y + 2)].into()
        }
    }

    fn left_L(&self) -> Vec<(i16, i16)> {
        match self.orientation {
            Orientation::North => [(self.x + 2, self.y), (self.x, self.y + 1)].into(),
            Orientation::East => [(self.x, self.y), (self.x, self.y + 1), (self.x, self.y + 2)].into(),
            Orientation::South => [(self.x, self.y), (self.x, self.y + 1)].into(),
            Orientation::West => [(self.x, self.y), (self.x + 1, self.y + 1), (self.x + 1, self.y + 2)].into()
        }
    }

    fn left_T(&self) -> Vec<(i16, i16)> {
        match self.orientation {
            Orientation::North => [(self.x + 1, self.y), (self.x, self.y + 1)].into(),
            Orientation::East => [(self.x, self.y), (self.x, self.y + 1), (self.x, self.y + 2)].into(),
            Orientation::South => [(self.x, self.y), (self.x + 1, self.y + 1)].into(),
            Orientation::West => [(self.x + 1, self.y), (self.x, self.y + 1), (self.x + 1, self.y + 2)].into()
        }
    }

    pub fn get_coords(&self) -> Vec<(i16, i16)> {
        match self.piece {
            Piece::I => self.coords_I(),
            Piece::O => self.coords_O(),
            Piece::S => self.coords_S(),
            Piece::Z => self.coords_Z(),
            Piece::J => self.coords_J(),
            Piece::L => self.coords_L(),
            Piece::T => self.coords_T(),
            _ => panic!("Shouldn't even arrive here")
        }
    }

    fn coords_I(&self) -> Vec<(i16, i16)> {
        match self.orientation {
            Orientation::North | Orientation::South => [(self.x, self.y), (self.x + 1, self.y), (self.x + 2, self.y), (self.x + 3, self.y)].into(),
            Orientation::East | Orientation::West => [(self.x, self.y), (self.x, self.y + 1), (self.x, self.y + 2), (self.x, self.y + 3)].into()
        }
    }

    fn coords_O(&self) -> Vec<(i16, i16)> {
        [(self.x, self.y), (self.x + 1, self.y), (self.x, self.y + 1), (self.x + 1, self.y + 1)].into()
    }

    fn coords_S(&self) -> Vec<(i16, i16)> {
        match self.orientation {
            Orientation::North | Orientation::South => [(self.x + 1, self.y), (self.x + 2, self.y), (self.x, self.y + 1), (self.x + 1, self.y + 1)].into(),
            Orientation::East | Orientation::West => [(self.x, self.y), (self.x, self.y + 1), (self.x + 1, self.y + 1), (self.x + 1, self.y + 2)].into()
        }
    }

    fn coords_Z(&self) -> Vec<(i16, i16)> {
        match self.orientation {
            Orientation::North | Orientation::South => [(self.x, self.y), (self.x + 1, self.y), (self.x + 1, self.y + 1), (self.x + 2, self.y + 1)].into(),
            Orientation::East | Orientation::West => [(self.x + 1, self.y), (self.x, self.y + 1), (self.x + 1, self.y + 1), (self.x, self.y + 2)].into()
        }
    }

    fn coords_J(&self) -> Vec<(i16, i16)> {
        match self.orientation {
            Orientation::North => [(self.x, self.y), (self.x, self.y + 1), (self.x + 1, self.y + 1), (self.x + 2, self.y + 1)].into(),
            Orientation::East => [(self.x, self.y), (self.x + 1, self.y), (self.x, self.y + 1), (self.x, self.y + 2)].into(),
            Orientation::South => [(self.x, self.y), (self.x + 1, self.y), (self.x + 2, self.y), (self.x + 2, self.y + 1)].into(),
            Orientation::West => [(self.x + 1, self.y), (self.x + 1, self.y + 1), (self.x, self.y + 2), (self.x + 1, self.y + 2)].into()
        }
    }

    fn coords_L(&self) -> Vec<(i16, i16)> {
        match self.orientation {
            Orientation::North => [(self.x + 2, self.y), (self.x, self.y + 1), (self.x + 1, self.y + 1), (self.x + 2, self.y + 1)].into(),
            Orientation::East => [(self.x, self.y), (self.x, self.y + 1), (self.x, self.y + 2), (self.x + 1, self.y + 2)].into(),
            Orientation::South => [(self.x, self.y), (self.x + 1, self.y ), (self.x + 2, self.y), (self.x, self.y + 1)].into(),
            Orientation::West => [(self.x, self.y), (self.x + 1, self.y), (self.x + 1, self.y + 1), (self.x + 1, self.y + 2)].into()
        }
    }

    fn coords_T(&self) -> Vec<(i16, i16)> {
        match self.orientation {
            Orientation::North => [(self.x + 1, self.y), (self.x, self.y + 1), (self.x + 1, self.y + 1), (self.x + 2, self.y + 1)].into(),
            Orientation::East => [(self.x, self.y), (self.x, self.y + 1), (self.x + 1, self.y + 1), (self.x, self.y + 2)].into(),
            Orientation::South => [(self.x, self.y), (self.x + 1, self.y), (self.x + 2, self.y), (self.x + 1, self.y + 1)].into(),
            Orientation::West => [(self.x + 1, self.y), (self.x, self.y + 1), (self.x + 1, self.y + 1), (self.x + 1, self.y + 2)].into()
        }
    }

}
