use std::{
    fmt::Debug,
    ops::Range,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use board::{
    local_board::{ClearLinePattern, LocalBoard},
    remote_board::RemoteBoard,
    Board,
};
use game_options::GameOptions;
use pieces::Piece;
use queue::local_queue::LocalQueue;
use sound::SoundPlayer;
use tauri::{AppHandle, Emitter};
use tokio::sync::mpsc::{self, Receiver, Sender};

pub mod board;
mod pieces;
pub mod queue;
mod sound;
mod strategy;

const HELD_PIECE_EMIT: &str = "held_piece_emit";
const QUEUE_EMIT: &str = "queue_emit";
const STRATEGY_EMIT: &str = "strategy_emit";
const BOARD_STATE_EMIT: &str = "board_state_emit";
const LINE_CLEARED_EMIT: &str = "line_cleared";
const LINE_CLEARED_INFO_EMIT: &str = "line_cleared_info";
const HARD_DROP_EMIT: &str = "hard_drop";
const PIECE_FIXED_EMIT: &str = "piece_fixed";
const POINTS_EMIT: &str = "points";
const GAME_OVER_EMIT: &str = "game_over";
const GAME_WON_EMIT: &str = "game_won";
const NUMBER_OF_PIECES_IN_QUEUE_TO_EMIT: u128 = 5;

const BUFFER_STATE_FOR_NUMBERS: &str = "EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE";
const STATE_FOR_NUMBER_3: &str = "EEEEEEEEEEEEEGGGGEEEEEGEEEEGEEEGEEEEEEGEEEEEEEEEGEEEEEEEEEGEEEEEEEEEGEEEEEEEEGEEEEEEEEEGEEEEEEEEGEEEEEEEEEGEEEEEEEEEEGEEEEEEEEEGEEEEEEEEEEGEEEEEEEEEGEEEEEEEEEGEEGEEEEEEGEEEGEEEEGEEEEEGGGGEEEEEEEEEEEEE";
const STATE_FOR_NUMBER_2: &str = "EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEGGGGEEEEEGEEEEGEEEGEEEEEEGEEEEEEEEEGEEEEEEEEEGEEEEEEEEGEEEEEEEEGEEEEEEEEGEEEEEEEEGEEEEEEEEGEEEEEEEEGEEEEEEEEGEEEEEEEEEGEEEEEEEEEGGGGGGGGEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE";
const STATE_FOR_NUMBER_1: &str = "EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEGEEEEEEEEGGEEEEEEEGEGEEEEEEGEEGEEEEEEEEEGEEEEEEEEEGEEEEEEEEEGEEEEEEEEEGEEEEEEEEEGEEEEEEEEEGEEEEEEEEEGEEEEEEEEEGEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE";

const EXTENDED_TIME_LOCK_MILIS: u64 = 500;
const MOVEMENTS_LEFT_RESET: u8 = 15;

#[derive(Debug)]
pub struct Game {
    app: AppHandle,
    local_board: LocalBoard,
    remote_boards: Vec<RemoteBoard>,
    normal: bool,
    lines_40: bool,
    blitz: bool,
    start_time: u64,
    points: u128,
    game_started: bool,
    prev_clear_line_pattern: ClearLinePattern,
    level: u16,
    line_clears: u16,
    first_level_commands: Receiver<FirstLevelCommands>,
    run: bool,
    last_piece: Piece,
    piece_lowest_y: i16,
    count_movements_enabled: bool,
    movements_left: u8,
    //player: SoundPlayer,
}

impl Game {
    pub fn new(
        options: GameOptions,
        app: AppHandle,
        receiver: Receiver<FirstLevelCommands>,
    ) -> Self {
        Game {
            //player: SoundPlayer::new(&app),
            app,
            local_board: LocalBoard::new(LocalQueue::default()),
            remote_boards: Vec::new(),
            normal: options.is_normal(),
            lines_40: options.is_lines_40(),
            blitz: options.is_blitz(),
            start_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards 🗿🤙")
                .as_secs(),
            points: 0,
            game_started: false,
            prev_clear_line_pattern: ClearLinePattern::None,
            level: 1,
            line_clears: 0,
            first_level_commands: receiver,
            run: true,
            last_piece: Piece::Ghost,
            piece_lowest_y: -20,
            count_movements_enabled: false,
            movements_left: MOVEMENTS_LEFT_RESET,
        }
    }

    pub async fn start_game(&mut self) {
        if self.game_started {
            return;
        }
        self.run = true;
        self.local_board = LocalBoard::new(LocalQueue::default());
        self.app
            .emit(
                BOARD_STATE_EMIT,
                format!("{}{}", BUFFER_STATE_FOR_NUMBERS, STATE_FOR_NUMBER_3),
            )
            .unwrap();
        tokio::time::sleep(Duration::from_secs(1)).await;
        self.app
            .emit(
                BOARD_STATE_EMIT,
                format!("{}{}", BUFFER_STATE_FOR_NUMBERS, STATE_FOR_NUMBER_2),
            )
            .unwrap();
        tokio::time::sleep(Duration::from_secs(1)).await;
        self.app
            .emit(
                BOARD_STATE_EMIT,
                format!("{}{}", BUFFER_STATE_FOR_NUMBERS, STATE_FOR_NUMBER_1),
            )
            .unwrap();
        tokio::time::sleep(Duration::from_secs(1)).await;
        self.game_loop().await;
    }

    pub fn forfeit_game(&self) {
        todo!()
    }

    pub fn retry_game(&self) {
        todo!()
    }

    pub fn targeting_strategy_elimination(&mut self) {
        todo!()
    }

    pub fn targeting_strategy_even(&mut self) {
        todo!()
    }

    pub fn targeting_strategy_random(&mut self) {
        todo!()
    }

    pub fn targeting_strategy_payback(&mut self) {
        todo!()
    }

    pub fn save_piece(&mut self) {
        todo!()
    }

    async fn game_loop(&mut self) {
        let (tx, mut rx) = mpsc::channel(32);
        let (tx_points, rx_points) = mpsc::channel(32);
        let (tx_extended_lock, mut rx_extended_lock) = mpsc::channel(32);
        Self::tick_loop(tx, rx_points).await;
        self.queue_emit();
        self.state_emit();
        self.game_started = true;
        while self.first_level_commands.try_recv().is_ok() {} // Empty possible orders given before start
        while self.run {
            if self.local_board.piece_y() > self.piece_lowest_y {
                self.piece_lowest_y = self.local_board.piece_y();
            }
            if self.movements_left == 0 && self.count_movements_enabled {
                self.local_board.hard_drop();
                self.state_emit();
                self.piece_fixed(&tx_points).await;
            }
            while let Ok(y) = rx_extended_lock.try_recv() {
                if y >= self.piece_lowest_y && self.count_movements_enabled {
                    self.local_board.hard_drop();
                    self.state_emit();
                    self.piece_fixed(&tx_points).await;
                }
            }
            if self.local_board.piece_at_bottom() {
                self.count_movements_enabled = true;
                self.movements_left = MOVEMENTS_LEFT_RESET;
                self.piece_lowest_y = self.local_board.piece_y();
                Self::extended_lock_down(tx_extended_lock.clone(), self.piece_lowest_y).await;
            }
            if rx.try_recv().is_ok() && !self.count_movements_enabled {
                self.last_piece = self.local_board.cur_piece();
                if self.local_board.next_tick() {
                    self.piece_fixed(&tx_points).await;
                }
                self.state_emit();
            }
            while let Ok(command) = self.first_level_commands.try_recv() {
                if !self.game_started {
                    continue;
                }
                match command {
                    FirstLevelCommands::RightMove => {
                        self.local_board.move_right();
                        self.count_movements();
                        // self.player.play_right_left().await;
                    }
                    FirstLevelCommands::LeftMove => {
                        self.local_board.move_left();
                        self.count_movements();
                        // self.player.play_right_left().await;
                    }
                    FirstLevelCommands::ClockWiseRotation => self.local_board.rotation_clockwise(),
                    FirstLevelCommands::CounterClockWiseRotation => {
                        self.local_board.rotation_counterclockwise()
                    }
                    FirstLevelCommands::HardDrop => {
                        self.local_board.hard_drop();
                        self.piece_fixed(&tx_points).await;
                        self.state_emit();
                        // self.player.play_piece_drop().await;
                    }
                    FirstLevelCommands::SoftDrop => {
                        self.local_board.soft_drop();
                        // self.player.play_soft_drop().await
                    }
                    FirstLevelCommands::SavePiece => {
                        let piece = self.local_board.held_piece();
                        self.local_board.save_piece();
                        if piece != self.local_board.held_piece() {
                            self.emit_held_piece();
                            self.queue_emit();
                        }
                    }
                    FirstLevelCommands::FullRotation => self.local_board.rotation_full(),
                }
                self.state_emit();
            }
            tokio::time::sleep(Duration::from_micros(16_666)).await;
        }
    }
    fn count_movements(&mut self) {
        if self.count_movements_enabled {
            self.movements_left -= 1;
        }
    }
    async fn tick_loop(sender: Sender<bool>, mut receiver: Receiver<u16>) {
        //  This thread will die, it's fine
        tokio::spawn(async move {
            let mut level = 1;
            loop {
                if let Ok(level_received) = receiver.try_recv() {
                    level = level_received;
                }
                let duration = (level - 1) as f64 * 0.007;
                let duration = 0.8 - duration;
                let duration = (duration.powf((level - 1) as f64) * 1000.0).round() as u64;
                tokio::time::sleep(Duration::from_millis(duration)).await;
                sender.send(true).await.unwrap();
            }
        });
    }
    async fn extended_lock_down(sender: Sender<i16>, lowest_y: i16) {
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(EXTENDED_TIME_LOCK_MILIS)).await;
            sender.send(lowest_y).await.unwrap();
        });
    }
    async fn piece_fixed(&mut self, sender: &Sender<u16>) {
        self.piece_lowest_y = -20;
        self.count_movements_enabled = false;
        self.movements_left = MOVEMENTS_LEFT_RESET;
        self.queue_emit();
        self.piece_fixed_emit();
        self.check_line_cleared().await;

        let game_over = self.local_board.game_over();
        let game_won = self.local_board.game_won(self.get_win_condition());
        if game_won {
            self.game_won_emit();
            self.run = false;
        } else if game_over {
            self.game_over_emit();
            // self.player.play_loss().await;
            self.run = false;
        }
        sender.send(self.level).await.unwrap();
    }
    fn get_win_condition(&self) -> impl Fn(bool, u32) -> bool {
        let seconds = self.start_time;
        let normal_win_condition = move |_game_over, _lines_cleared| false;
        let blitz_time_condition = move |game_over, _lines_cleared| {
            if game_over {
                return false;
            }
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                - seconds
                > 120
        };
        let lines_40_win_condition = move |game_over, lines_cleared| {
            if game_over {
                return false;
            }
            lines_cleared >= 40
        };
        move |game_over, lines_cleared| match (self.normal, self.blitz, self.lines_40) {
            (true, false, false) => normal_win_condition(game_over, lines_cleared),
            (false, true, false) => blitz_time_condition(game_over, lines_cleared),
            (false, false, true) => lines_40_win_condition(game_over, lines_cleared),
            _ => panic!("Invalid state"),
        }
    }

    async fn check_line_cleared(&mut self) {
        let pattern = self.local_board.clear_line_pattern();
        if pattern != ClearLinePattern::None {
            self.points_calculation(pattern);
            self.lines_awarded_calculation(pattern);
            if self.line_clears >= self.level * 5 {
                self.level += 1;
                self.line_clears = 0;
            }
            self.line_emit(pattern);
            self.points_emit();
            // match pattern {
            //     ClearLinePattern::TSpinDouble
            //     | ClearLinePattern::TSpinTriple
            //     | ClearLinePattern::Tetris => self.player.play_tspin_tetris().await,
            //     _ => (),
            // }
        }
        self.prev_clear_line_pattern = pattern;
    }

    fn points_calculation(&mut self, pattern: ClearLinePattern) {
        self.points += match pattern {
            ClearLinePattern::None => 0,
            ClearLinePattern::Single => 100,
            ClearLinePattern::Double => 300,
            ClearLinePattern::Triple => 500,
            ClearLinePattern::Tetris => 800,
            ClearLinePattern::TSpin => 400,
            ClearLinePattern::TSpinSingle => 800,
            ClearLinePattern::TSpinDouble => 1200,
            ClearLinePattern::TSpinTriple => 1600,
            ClearLinePattern::MiniTSpin => 100,
            ClearLinePattern::MiniTSpinSingle => 200,
        } * self.level as u128;
        if pattern == self.prev_clear_line_pattern {
            self.points += match pattern {
                ClearLinePattern::None => 0,
                ClearLinePattern::Single => 50,
                ClearLinePattern::Double => 150,
                ClearLinePattern::Triple => 250,
                ClearLinePattern::Tetris => 400,
                ClearLinePattern::TSpin => 200,
                ClearLinePattern::TSpinSingle => 400,
                ClearLinePattern::TSpinDouble => 600,
                ClearLinePattern::TSpinTriple => 1600,
                ClearLinePattern::MiniTSpin => 50,
                ClearLinePattern::MiniTSpinSingle => 100,
            } * self.level as u128;
        }
    }
    fn lines_awarded_calculation(&mut self, pattern: ClearLinePattern) {
        self.line_clears += match pattern {
            ClearLinePattern::None => 0,
            ClearLinePattern::Single => 1,
            ClearLinePattern::Double => 3,
            ClearLinePattern::Triple => 5,
            ClearLinePattern::Tetris => 8,
            ClearLinePattern::TSpin => 4,
            ClearLinePattern::TSpinSingle => 8,
            ClearLinePattern::TSpinDouble => 12,
            ClearLinePattern::TSpinTriple => 16,
            ClearLinePattern::MiniTSpin => 1,
            ClearLinePattern::MiniTSpinSingle => 2,
        };
        if pattern == self.prev_clear_line_pattern {
            self.points += match pattern {
                ClearLinePattern::None => 0,
                ClearLinePattern::Single => 1,
                ClearLinePattern::Double => 2,
                ClearLinePattern::Triple => 3,
                ClearLinePattern::Tetris => 4,
                ClearLinePattern::TSpin => 1,
                ClearLinePattern::TSpinSingle => 4,
                ClearLinePattern::TSpinDouble => 6,
                ClearLinePattern::TSpinTriple => 16,
                ClearLinePattern::MiniTSpin => 1,
                ClearLinePattern::MiniTSpinSingle => 1,
            };
        }
    }

    fn emit_held_piece(&self) {
        self.app
            .emit(
                HELD_PIECE_EMIT,
                self.local_board
                    .held_piece()
                    .expect("Isn't called until there is a held piece"),
            )
            .unwrap();
    }

    fn queue_emit(&mut self) {
        let range: Range<u128> = self.local_board.piece_num() as u128 + 1
            ..self.local_board.piece_num() as u128 + NUMBER_OF_PIECES_IN_QUEUE_TO_EMIT + 1;
        self.app
            .emit(QUEUE_EMIT, self.local_board.get_pieces(range))
            .unwrap();
    }
    fn state_emit(&self) {
        self.app
            .emit(BOARD_STATE_EMIT, self.local_board.board_state())
            .unwrap();
    }
    fn line_emit(&self, pattern: ClearLinePattern) {
        self.app.emit(LINE_CLEARED_EMIT, pattern).unwrap();
        self.app
            .emit(
                LINE_CLEARED_INFO_EMIT,
                format!("{}/{}", self.line_clears, self.level * 5),
            )
            .unwrap();
    }
    fn piece_fixed_emit(&self) {
        self.app.emit(PIECE_FIXED_EMIT, self.last_piece).unwrap();
    }
    fn points_emit(&self) {
        self.app.emit(POINTS_EMIT, self.points).unwrap();
    }
    fn game_won_emit(&self) {
        self.app.emit(GAME_WON_EMIT, true).unwrap();
    }
    fn game_over_emit(&self) {
        self.app.emit(GAME_OVER_EMIT, true).unwrap();
    }
}
#[derive(Debug)]
pub enum FirstLevelCommands {
    RightMove,
    LeftMove,
    ClockWiseRotation,
    CounterClockWiseRotation,
    HardDrop,
    SoftDrop,
    SavePiece,
    FullRotation,
}
enum SecondLevelCommands {}
enum ThirdLevelCommands {}
enum FourthLevelCommands {}
// * Events to emit
// * - Held piece -> Piece
// * - Queue -> [Piece]
// * - Strategy -> Strategy
// * - Board state
// * - Lost
// * - Won
// * - Piece set
// * - Piece hard dropped
// * - Other boards state -> not yet implemented
pub mod game_options;
