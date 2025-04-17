use serde::{Deserialize, Serialize};

use super::{board::local_board::ClearLinePattern, game_options::GameOptions};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct GameInfo {
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
}

impl GameInfo {
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

    pub fn piece_moves(&self) -> u32 {
        self.piece_moves
    }

    pub fn spins(&self) -> u32 {
        self.spins
    }

    pub fn lines_cleared(&self) -> u32 {
        self.lines_cleared
    }

    pub fn pieces_used(&self) -> u32 {
        self.pieces_used
    }

    pub fn singles(&self) -> u32 {
        self.singles
    }

    pub fn doubles(&self) -> u32 {
        self.doubles
    }

    pub fn triples(&self) -> u32 {
        self.triples
    }

    pub fn tetrises(&self) -> u32 {
        self.tetrises
    }

    pub fn tspins(&self) -> u32 {
        self.tspins
    }

    pub fn tspin_singles(&self) -> u32 {
        self.tspin_singles
    }

    pub fn tspin_doubles(&self) -> u32 {
        self.tspin_doubles
    }

    pub fn tspin_triples(&self) -> u32 {
        self.tspin_triples
    }

    pub fn minitspins(&self) -> u32 {
        self.minitspins
    }

    pub fn minitspin_singles(&self) -> u32 {
        self.minitspin_singles
    }

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
    pub fn piece_used(&mut self) {
        self.pieces_used += 1;
    }
    pub fn piece_moved(&mut self) {
        self.piece_moves += 1;
    }
    pub fn spinned(&mut self) {
        self.spins += 1;
    }
    pub fn register_final_info(&mut self, time: u64, points: u64, level: u16) {
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
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
enum GameTypeInfo {
    Classic(ClassicGameInfo),
    Lines(LinesGameInfo),
    Blitz(BlitzGameInfo),
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy)]
struct ClassicGameInfo {
    time_endured: u64,
    points: u64,
    level_reached: u16,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy)]
struct LinesGameInfo {
    time_endured: u64,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy)]
struct BlitzGameInfo {
    points: u64,
}
