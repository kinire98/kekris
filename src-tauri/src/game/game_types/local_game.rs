use std::{
    fmt::Debug,
    ops::Range,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use super::super::pieces::Piece;
use super::super::queue::local_queue::LocalQueue;
use super::super::sound::{
    play_line_clear, play_loss, play_piece_drop, play_right_left, play_soft_drop, play_tspin_tetris,
};
use super::super::{
    board::{
        Board, // remote_board::RemoteBoard,
        local_board::{ClearLinePattern, LocalBoard},
    },
    queue::Queue,
};
use tauri::{AppHandle, Emitter};
use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::{
    models::{
        game_commands::{FirstLevelCommands, SecondLevelCommands},
        game_info::GameInfo,
        game_options::GameOptions,
        game_responses::GameResponses,
    },
    persistence::store_game_info,
};

const HELD_PIECE_EMIT: &str = "held_piece_emit";
const QUEUE_EMIT: &str = "queue_emit";
const BOARD_STATE_EMIT: &str = "board_state_emit";
const LINE_CLEARED_EMIT: &str = "line_cleared";
const LINE_CLEARED_INFO_EMIT: &str = "line_cleared_info";
const PIECE_FIXED_EMIT: &str = "piece_fixed";
const POINTS_EMIT: &str = "points";
const GAME_OVER_EMIT: &str = "game_over";
const GAME_WON_EMIT: &str = "game_won";
const TIME_EMIT: &str = "time_emit";
const NUMBER_OF_PIECES_IN_QUEUE_TO_EMIT: u128 = 5;

const BUFFER_STATE_FOR_NUMBERS: &str = "EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE";
const STATE_FOR_NUMBER_3: &str = "EEEEEEEEEEEEEGGGGEEEEEGEEEEGEEEGEEEEEEGEEEEEEEEEGEEEEEEEEEGEEEEEEEEEGEEEEEEEEGEEEEEEEEEGEEEEEEEEGEEEEEEEEEGEEEEEEEEEEGEEEEEEEEEGEEEEEEEEEEGEEEEEEEEEGEEEEEEEEEGEEGEEEEEEGEEEGEEEEGEEEEEGGGGEEEEEEEEEEEEE";
const STATE_FOR_NUMBER_2: &str = "EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEGGGGEEEEEGEEEEGEEEGEEEEEEGEEEEEEEEEGEEEEEEEEEGEEEEEEEEGEEEEEEEEGEEEEEEEEGEEEEEEEEGEEEEEEEEGEEEEEEEEGEEEEEEEEGEEEEEEEEEGEEEEEEEEEGGGGGGGGEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE";
const STATE_FOR_NUMBER_1: &str = "EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEGEEEEEEEEGGEEEEEEEGEGEEEEEEGEEGEEEEEEEEEGEEEEEEEEEGEEEEEEEEEGEEEEEEEEEGEEEEEEEEEGEEEEEEEEEGEEEEEEEEEGEEEEEEEEEGEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE";

const EXTENDED_TIME_LOCK_MILIS: u64 = 500;
const MOVEMENTS_LEFT_RESET: u8 = 15;

#[derive(Debug)]
pub struct LocalGame {
    app: AppHandle,
    number_of_players: u8,
    local_board: LocalBoard,
    // remote_boards: Vec<RemoteBoard>,
    normal: bool,
    lines_40: bool,
    blitz: bool,
    start_time: u64,
    points: u32,
    game_started: bool,
    prev_clear_line_pattern: ClearLinePattern,
    level: u16,
    line_clears: u16,
    real_line_clears: u16,
    game_control: Receiver<GameControl>,
    first_level_commands: Receiver<FirstLevelCommands>,
    second_level_commands: Option<Receiver<SecondLevelCommands>>,
    run: bool,
    last_piece: Piece,
    piece_lowest_y: i16,
    count_movements_enabled: bool,
    movements_left: u8,
    game_info: GameInfo,
    register_info: bool,
    responder: Option<Sender<GameResponses>>,
}

impl LocalGame {
    pub async fn new(
        options: GameOptions,
        app: AppHandle,
        first_level_commands: Receiver<FirstLevelCommands>,
        second_level_commands: Option<Receiver<SecondLevelCommands>>,
        game_control_receiver: Receiver<GameControl>,
        responder: Option<Sender<GameResponses>>,
        queue: impl Queue + 'static,
    ) -> Self {
        LocalGame {
            app,
            number_of_players: options.number_of_players(),
            local_board: LocalBoard::new(queue),
            // remote_boards: Vec::new(),
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
            real_line_clears: 0,
            first_level_commands,
            second_level_commands,
            game_control: game_control_receiver,
            run: true,
            last_piece: Piece::Ghost,
            piece_lowest_y: -20,
            count_movements_enabled: false,
            movements_left: MOVEMENTS_LEFT_RESET,
            game_info: GameInfo::new(options),
            register_info: false,
            responder,
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

    async fn game_loop(&mut self) {
        let (tx, mut rx) = mpsc::channel(32);
        let (tx_points, rx_points) = mpsc::channel(32);
        let (tx_extended_lock, mut rx_extended_lock) = mpsc::channel(32);
        Self::tick_loop(tx, rx_points).await;
        self.queue_emit();
        self.state_emit();
        self.game_started = true;
        let mut forfeited = false;
        self.start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards 🗿🤙")
            .as_secs();
        let mut prev_time = self.start_time;
        while self.first_level_commands.try_recv().is_ok() {} // Empty possible orders given before start
        // Game loop
        while self.run {
            // First of all execute the critical checks first
            self.critical_checks(
                &tx_points,
                &mut rx_extended_lock,
                tx_extended_lock.clone(),
                &mut rx,
            )
            .await;
            // Control operations such as forfeit and retry
            // If one of these execute it doesnt make sense to check for the rest
            while let Ok(control) = self.game_control.try_recv() {
                match control {
                    GameControl::Forfeit => {
                        forfeited = true;
                        self.run = false;
                    }
                    GameControl::Retry => {
                        if self.number_of_players == 1 {
                            self.run = false;
                        }
                    }
                }
            }
            // Checks of the movements
            self.first_level_checks(
                &tx_points,
                &mut rx_extended_lock,
                tx_extended_lock.clone(),
                &mut rx,
            )
            .await;

            self.second_level_checks(
                &tx_points,
                &mut rx_extended_lock,
                tx_extended_lock.clone(),
                &mut rx,
            )
            .await;
            // Makes sure that executes max 120 times per second, no need for more
            // and it avoids CPU overuse
            tokio::time::sleep(Duration::from_micros(8_333)).await;
            // Time calculations
            let cur_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards 🗿🤙")
                .as_secs();
            if cur_time > prev_time {
                prev_time = cur_time;
                self.time_emit(cur_time);
                // if self.blitz && self.local_board.game_won(self.get_win_condition()) {
                // self.game_won_emit();
                // self.run = false;
                // }
            }
        }
        if forfeited {
            self.game_over_emit(true);
        }
        if self.register_info {
            self.register_info().await;
        }
    }
    /// In this method there are checks crucial for the gameplay,
    /// related to piece fixation and countdown. Generally, anything
    /// that can make you lose the game
    async fn critical_checks(
        &mut self,
        tx_points: &Sender<u16>,
        rx_extended_lock: &mut Receiver<i16>,
        tx_extended_lock: Sender<i16>,
        rx: &mut Receiver<bool>,
    ) {
        // Register the lowest point of y
        if self.local_board.piece_y() > self.piece_lowest_y {
            self.piece_lowest_y = self.local_board.piece_y();
        }
        // If the movements left drop to 0 and are enabled, piece gets fixed
        if self.movements_left == 0 && self.count_movements_enabled {
            self.local_board.hard_drop();
            self.state_emit();
            self.piece_fixed(tx_points).await;
        }
        // If the max time for the piece fixed has passed, and the current y coordinate is
        // greater than or equal than the lowest point of y then the piece gets fixed
        while let Ok(y) = rx_extended_lock.try_recv() {
            if y >= self.piece_lowest_y && self.count_movements_enabled {
                self.local_board.hard_drop();
                self.state_emit();
                self.piece_fixed(tx_points).await;
            }
        }
        // If the piece is at bottom (that meeaning that it cannot go down) it starts the countdown
        // for movements and time
        if self.local_board.piece_at_bottom() {
            self.count_movements_enabled = true;
            self.movements_left = MOVEMENTS_LEFT_RESET;
            self.piece_lowest_y = self.local_board.piece_y();
            Self::extended_lock_down(tx_extended_lock.clone(), self.piece_lowest_y).await;
        }
        // Game tick
        if rx.try_recv().is_ok() && !self.count_movements_enabled {
            self.last_piece = self.local_board.cur_piece();
            if self.local_board.next_tick() {
                self.piece_fixed(tx_points).await;
            }
            self.state_emit();
        }
    }
    /// Checks for piece movements
    async fn first_level_checks(
        &mut self,
        tx_points: &Sender<u16>,
        rx_extended_lock: &mut Receiver<i16>,
        tx_extended_lock: Sender<i16>,
        rx: &mut Receiver<bool>,
    ) {
        while let Ok(command) = self.first_level_commands.try_recv() {
            if !self.game_started {
                continue;
            }
            match command {
                FirstLevelCommands::RightMove => {
                    if self.local_board.move_right() {
                        self.count_movements();
                        play_right_left(self.app.clone()).await;
                        self.game_info.piece_moved();
                    }
                }
                FirstLevelCommands::LeftMove => {
                    if self.local_board.move_left() {
                        self.count_movements();
                        play_right_left(self.app.clone()).await;
                        self.game_info.piece_moved();
                    }
                }
                FirstLevelCommands::ClockWiseRotation => {
                    self.local_board.rotation_clockwise();
                    self.game_info.spinned();
                }
                FirstLevelCommands::CounterClockWiseRotation => {
                    self.local_board.rotation_counterclockwise();
                    self.game_info.spinned();
                }
                FirstLevelCommands::HardDrop => {
                    self.local_board.hard_drop();
                    self.piece_fixed(tx_points).await;
                    self.state_emit();
                    play_piece_drop(self.app.clone()).await;
                }
                FirstLevelCommands::SoftDrop => {
                    self.local_board.soft_drop();
                    play_soft_drop(self.app.clone()).await;
                }
                FirstLevelCommands::SavePiece => {
                    let piece = self.local_board.held_piece();
                    self.local_board.save_piece();
                    self.count_movements_enabled = false;
                    if piece != self.local_board.held_piece() {
                        self.emit_held_piece();
                        self.queue_emit();
                    }
                }
                FirstLevelCommands::FullRotation => {
                    self.local_board.rotation_full();
                    self.game_info.spinned();
                }
            }
            self.state_emit();
            self.critical_checks(tx_points, rx_extended_lock, tx_extended_lock.clone(), rx)
                .await;
        }
    }

    async fn second_level_checks(
        &mut self,
        tx_points: &Sender<u16>,
        rx_extended_lock: &mut Receiver<i16>,
        tx_extended_lock: Sender<i16>,
        rx: &mut Receiver<bool>,
    ) {
        if self.second_level_commands.is_none() {
            return;
        }
        if let Ok(command) = self.second_level_commands.as_mut().unwrap().try_recv() {
            match command {
                SecondLevelCommands::QueueSync => (),
                SecondLevelCommands::TrashReceived(_) => (),
                SecondLevelCommands::StrategyChange(_strategy) => {
                    todo!()
                }
                SecondLevelCommands::Won => todo!(),
            }
            self.first_level_checks(tx_points, rx_extended_lock, tx_extended_lock, rx)
                .await;
        }
    }

    /// Counts movements for the piece fixation
    fn count_movements(&mut self) {
        if self.count_movements_enabled {
            self.movements_left -= 1;
        }
    }
    /// This keeps track of the time. Receives the level of the game to execute the loop faster
    async fn tick_loop(sender: Sender<bool>, mut receiver: Receiver<u16>) {
        // !  This thread shouldn't abruptly die, it's not fine if it dies
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
                match sender.send(true).await {
                    Ok(_) => (),
                    Err(_) => break,
                }
            }
        });
    }
    /// This keeps the track for the extended piece lock down
    async fn extended_lock_down(sender: Sender<i16>, lowest_y: i16) {
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(EXTENDED_TIME_LOCK_MILIS)).await;
            let _ = sender.send(lowest_y).await;
        });
    }
    /// Checks for piece fixed. Emits the state necessary to the frontend, checks if game has been los or won,
    /// and performs operations for, checking lines cleareance
    async fn piece_fixed(&mut self, sender: &Sender<u16>) {
        self.piece_lowest_y = -20;
        self.count_movements_enabled = false;
        self.movements_left = MOVEMENTS_LEFT_RESET;
        self.queue_emit();
        self.piece_fixed_emit();
        self.check_line_cleared().await;
        self.game_info.piece_used();

        let game_over = self.local_board.game_over();
        let game_won = self.local_board.game_won(self.get_win_condition());
        if game_won {
            self.game_won_emit();
            self.run = false;
            self.register_info = true;
            let now_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards 🗿🤙")
                .as_secs();
            self.game_info
                .register_final_info(now_time - self.start_time, self.points, self.level);
        } else if game_over {
            play_loss(self.app.clone()).await;
            self.run = false;
            if self.normal {
                self.game_over_emit(false);
                self.register_info = true;
                let now_time = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards 🗿🤙")
                    .as_secs();
                self.game_info.register_final_info(
                    now_time - self.start_time,
                    self.points,
                    self.level,
                );
            } else {
                self.game_over_emit(true);
            }
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
                >= 120
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

    /// Checks if the line has been cleared and performs the neccesary calculations
    async fn check_line_cleared(&mut self) {
        let pattern = self.local_board.clear_line_pattern();
        if pattern != ClearLinePattern::None {
            self.points_calculation(pattern);
            self.lines_awarded_calculation(pattern);
            if self.line_clears >= self.level * 5 {
                self.level += 1;
                self.line_clears = 0;
            }
            if self.normal || self.lines_40 {
                self.line_emit(pattern);
            } else {
                self.points_emit();
            }
            match pattern {
                ClearLinePattern::TSpinDouble
                | ClearLinePattern::TSpinTriple
                | ClearLinePattern::Tetris => play_tspin_tetris(self.app.clone()).await,
                ClearLinePattern::None => (),
                _ => play_line_clear(self.app.clone()).await,
            }
            self.game_info.line_cleared(pattern);
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
        } * self.level as u32;
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
            } * self.level as u32;
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
        self.real_line_clears += match pattern {
            ClearLinePattern::None => 0,
            ClearLinePattern::Single => 1,
            ClearLinePattern::Double => 2,
            ClearLinePattern::Triple => 3,
            ClearLinePattern::Tetris => 4,
            ClearLinePattern::TSpin => 0,
            ClearLinePattern::TSpinSingle => 1,
            ClearLinePattern::TSpinDouble => 2,
            ClearLinePattern::TSpinTriple => 3,
            ClearLinePattern::MiniTSpin => 0,
            ClearLinePattern::MiniTSpinSingle => 1,
        };
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
        let payload = if self.lines_40 {
            format!("{}/{}", self.real_line_clears, 40)
        } else {
            format!("{}/{}", self.line_clears, self.level * 5)
        };
        self.app.emit(LINE_CLEARED_INFO_EMIT, payload).unwrap();
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
    fn game_over_emit(&self, forfeited: bool) {
        self.app.emit(GAME_OVER_EMIT, forfeited).unwrap();
    }
    fn time_emit(&self, now_secs: u64) {
        let total_secs = now_secs - self.start_time;
        let seconds = total_secs % 60;
        let minutes = (total_secs / 60) % 60;
        let hours = total_secs / 3600;
        self.app
            .emit(
                TIME_EMIT,
                format!("{:02}:{:02}:{:02}", hours, minutes, seconds),
            )
            .unwrap();
    }
    async fn register_info(&mut self) {
        let info = self.game_info;

        tokio::spawn(async move {
            store_game_info::store_game_info(info).await;
        });
    }
}
#[derive(Debug)]
pub enum GameControl {
    Retry,
    Forfeit,
}

//  pub enum ThirdLevelCommands {}
// enum FourthLevelCommands {}
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
