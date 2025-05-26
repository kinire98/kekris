use serde::{Deserialize, Serialize};

use super::game_options::GameOptions;
use crate::game::board::local_board::ClearLinePattern;

/// `GameInfo` stores general information about a game session, including statistics applicable to all game types.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, sqlx::FromRow, PartialEq, Eq)]
pub struct GameInfo {
    /// The number of times a piece was moved.
    piece_moves: u32,
    /// The number of spins performed.
    spins: u32,
    /// The number of lines cleared.
    lines_cleared: u32,
    /// The number of pieces used.
    pieces_used: u32,
    /// The number of single line clears.
    singles: u32,
    /// The number of double line clears.
    doubles: u32,
    /// The number of triple line clears.
    triples: u32,
    /// The number of tetris line clears.
    tetrises: u32,
    /// The number of T-spins performed.
    tspins: u32,
    /// The number of T-spin single line clears.
    tspin_singles: u32,
    /// The number of T-spin double line clears.
    tspin_doubles: u32,
    /// The number of T-spin triple line clears.
    tspin_triples: u32,
    /// The number of mini T-spins performed.
    minitspins: u32,
    /// The number of mini T-spin single line clears.
    minitspin_singles: u32,
    /// Specific information for the particular game type.
    specific_info: GameTypeInfo,
}

impl GameInfo {
    /// Creates a new `GameInfo` instance based on the provided `GameOptions`.
    pub fn new(options: GameOptions) -> Self {
        let type_info = match (
            options.is_normal(),
            options.is_lines_40(),
            options.is_blitz(),
        ) {
            (true, false, false) => GameTypeInfo::Classic(ClassicGameInfo::default()),
            (false, true, false) => GameTypeInfo::Lines(LinesGameInfo::default()),
            (false, false, true) => GameTypeInfo::Blitz(BlitzGameInfo::default()),
            _ => panic!("Invalid state"),
        };
        GameInfo {
            piece_moves: 0,
            spins: 0,
            lines_cleared: 0,
            pieces_used: 0,
            singles: 0,
            doubles: 0,
            triples: 0,
            tetrises: 0,
            tspins: 0,
            tspin_singles: 0,
            tspin_doubles: 0,
            tspin_triples: 0,
            minitspins: 0,
            minitspin_singles: 0,
            specific_info: type_info,
        }
    }
    /// Creates a new `GameInfo` instance with specific values.
    #[allow(clippy::too_many_arguments)]
    pub fn new_from(
        piece_moves: u32,
        spins: u32,
        lines_cleared: u32,
        pieces_used: u32,
        singles: u32,
        doubles: u32,
        triples: u32,
        tetrises: u32,
        tspins: u32,
        tspin_singles: u32,
        tspin_doubles: u32,
        tspin_triples: u32,
        minitspins: u32,
        minitspin_singles: u32,
        specific_info: GameTypeInfo,
    ) -> Self {
        Self {
            piece_moves,
            spins,
            lines_cleared,
            pieces_used,
            singles,
            doubles,
            triples,
            tetrises,
            tspins,
            tspin_singles,
            tspin_doubles,
            tspin_triples,
            minitspins,
            minitspin_singles,
            specific_info,
        }
    }

    /// Returns the number of piece moves.
    pub fn piece_moves(&self) -> u32 {
        self.piece_moves
    }

    /// Returns the number of spins.
    pub fn spins(&self) -> u32 {
        self.spins
    }

    /// Returns the number of lines cleared.
    pub fn lines_cleared(&self) -> u32 {
        self.lines_cleared
    }

    /// Returns the number of pieces used.
    pub fn pieces_used(&self) -> u32 {
        self.pieces_used
    }

    /// Returns the number of singles.
    pub fn singles(&self) -> u32 {
        self.singles
    }

    /// Returns the number of doubles.
    pub fn doubles(&self) -> u32 {
        self.doubles
    }

    /// Returns the number of triples.
    pub fn triples(&self) -> u32 {
        self.triples
    }

    /// Returns the number of tetrises.
    pub fn tetrises(&self) -> u32 {
        self.tetrises
    }

    /// Returns the number of tspins.
    pub fn tspins(&self) -> u32 {
        self.tspins
    }

    /// Returns the number of tspin singles.
    pub fn tspin_singles(&self) -> u32 {
        self.tspin_singles
    }

    /// Returns the number of tspin doubles.
    pub fn tspin_doubles(&self) -> u32 {
        self.tspin_doubles
    }

    /// Returns the number of tspin triples.
    pub fn tspin_triples(&self) -> u32 {
        self.tspin_triples
    }

    /// Returns the number of minitspins.
    pub fn minitspins(&self) -> u32 {
        self.minitspins
    }

    /// Returns the number of minitspin singles.
    pub fn minitspin_singles(&self) -> u32 {
        self.minitspin_singles
    }

    /// Updates the line clear counts based on the given `ClearLinePattern`.
    pub fn line_cleared(&mut self, pattern: ClearLinePattern) {
        match pattern {
            ClearLinePattern::None => (),
            ClearLinePattern::Single => {
                self.lines_cleared += 1;
                self.singles += 1;
            }
            ClearLinePattern::Double => {
                self.lines_cleared += 2;
                self.doubles += 1;
            }
            ClearLinePattern::Triple => {
                self.lines_cleared += 3;
                self.triples += 1;
            }
            ClearLinePattern::Tetris => {
                self.lines_cleared += 4;
                self.tetrises += 1;
            }
            ClearLinePattern::TSpin => self.tspins += 1,
            ClearLinePattern::TSpinSingle => {
                self.lines_cleared += 1;
                self.tspin_singles += 1;
            }
            ClearLinePattern::TSpinDouble => {
                self.lines_cleared += 2;
                self.tspin_doubles += 1;
            }
            ClearLinePattern::TSpinTriple => {
                self.lines_cleared += 3;
                self.tspin_triples += 1;
            }
            ClearLinePattern::MiniTSpin => self.minitspins += 1,
            ClearLinePattern::MiniTSpinSingle => {
                self.lines_cleared += 1;
                self.minitspin_singles += 1;
            }
        }
    }
    /// Increments the number of pieces used.
    pub fn piece_used(&mut self) {
        self.pieces_used += 1;
    }
    /// Increments the number of piece moves.
    pub fn piece_moved(&mut self) {
        self.piece_moves += 1;
    }
    /// Increments the number of spins.
    pub fn spinned(&mut self) {
        self.spins += 1;
    }
    /// Registers the final information for the game, such as time, points, and level.
    pub fn register_final_info(&mut self, time: u64, points: u32, level: u16) {
        match &mut self.specific_info {
            GameTypeInfo::Classic(classic_game_info) => {
                classic_game_info.level_reached = level;
                classic_game_info.points = points;
                classic_game_info.time_endured = time;
            }
            GameTypeInfo::Lines(lines_game_info) => {
                lines_game_info.time_endured = time;
            }
            GameTypeInfo::Blitz(blitz_game_info) => {
                blitz_game_info.points = points;
            }
        }
    }
    /// Returns the specific game type information.
    pub fn type_of_info(&self) -> GameTypeInfo {
        self.specific_info
    }
}

/// `GameTypeInfo` is an enum that represents the specific information for each game type.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameTypeInfo {
    /// Represents information for a classic game.
    Classic(ClassicGameInfo),
    /// Represents information for a lines game.
    Lines(LinesGameInfo),
    /// Represents information for a blitz game.
    Blitz(BlitzGameInfo),
}

/// `ClassicGameInfo` stores information specific to classic Tetris games.
#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy, sqlx::FromRow, PartialEq, Eq)]
pub struct ClassicGameInfo {
    /// The time endured in the game.
    time_endured: u64,
    /// The points earned in the game.
    points: u32,
    /// The level reached in the game.
    level_reached: u16,
}

impl ClassicGameInfo {
    /// Creates a new `ClassicGameInfo` instance.
    pub fn new(time_endured: u64, points: u32, level_reached: u16) -> Self {
        ClassicGameInfo {
            time_endured,
            points,
            level_reached,
        }
    }
    /// Returns the time endured in the game.
    pub fn time_endured(&self) -> u32 {
        self.time_endured as u32
    }

    /// Returns the points earned in the game.
    pub fn points(&self) -> u32 {
        self.points
    }

    /// Returns the level reached in the game.
    pub fn level_reached(&self) -> u16 {
        self.level_reached
    }
}

/// `LinesGameInfo` stores information specific to 40-lines Tetris games.
#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy, sqlx::FromRow, PartialEq, Eq)]
pub struct LinesGameInfo {
    /// The time endured in the game.
    time_endured: u64,
}

impl LinesGameInfo {
    /// Creates a new `LinesGameInfo` instance.
    pub fn new(time_endured: u64) -> Self {
        LinesGameInfo { time_endured }
    }
    /// Returns the time endured in the game.
    pub fn time_endured(&self) -> u32 {
        self.time_endured as u32
    }
}

/// `BlitzGameInfo` stores information specific to Blitz Tetris games.
#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy, sqlx::FromRow, PartialEq, Eq)]
pub struct BlitzGameInfo {
    /// The points earned in the game.
    points: u32,
}

impl BlitzGameInfo {
    /// Creates a new `BlitzGameInfo` instance.
    pub fn new(points: u32) -> Self {
        BlitzGameInfo { points }
    }
    /// Returns the points earned in the game.
    pub fn points(&self) -> u32 {
        self.points
    }
}
