use std::{
    fmt::Debug,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use board::{Board, local_board::LocalBoard, remote_board::RemoteBoard};
use game_options::GameOptions;
use pieces::Piece;
use queue::local_queue::LocalQueue;
use strategy::Strategy;
use tauri::{AppHandle, Emitter};
use tokio::sync::mpsc::{self, Receiver, Sender};

pub mod board;
mod pieces;
pub mod queue;
mod strategy;

const HELD_PIECE_EMIT: &str = "held_piece_emit";
const QUEUE_EMIT: &str = "queue_emit";
const STRATEGY_EMIT: &str = "strategy_emit";
const BOARD_STATE_EMIT: &str = "board_state_emit";
const NUMBER_OF_PIECES_IN_QUEUE_TO_EMIT: usize = 5;

const BUFFER_STATE_FOR_NUMBERS: &str = "EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE";
const STATE_FOR_NUMBER_3: &str = "EEEEEEEEEEEEEGGGGEEEEEGEEEEGEEEGEEEEEEGEEEEEEEEEGEEEEEEEEEGEEEEEEEEEGEEEEEEEEGEEEEEEEEEGEEEEEEEEGEEEEEEEEEGEEEEEEEEEEGEEEEEEEEEGEEEEEEEEEEGEEEEEEEEEGEEEEEEEEEGEEGEEEEEEGEEEGEEEEGEEEEEGGGGEEEEEEEEEEEEE";
const STATE_FOR_NUMBER_2: &str = "EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEGGGGEEEEEGEEEEGEEEGEEEEEEGEEEEEEEEEGEEEEEEEEEGEEEEEEEEGEEEEEEEEGEEEEEEEEGEEEEEEEEGEEEEEEEEGEEEEEEEEGEEEEEEEEGEEEEEEEEEGEEEEEEEEEGGGGGGGGEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE";
const STATE_FOR_NUMBER_1: &str = "EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEGEEEEEEEEGGEEEEEEEGEGEEEEEEGEEGEEEEEEEEEGEEEEEEEEEGEEEEEEEEEGEEEEEEEEEGEEEEEEEEEGEEEEEEEEEGEEEEEEEEEGEEEEEEEEEGEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE";

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
    first_level_commands: Receiver<FirstLevelCommands>,
}

impl Game {
    pub fn new(
        options: GameOptions,
        app: AppHandle,
        receiver: Receiver<FirstLevelCommands>,
    ) -> Self {
        Game {
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
            first_level_commands: receiver,
        }
    }

    pub async fn start_game(&mut self) {
        if self.game_started {
            return;
        }
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
        Self::tick_loop(tx, rx_points).await;
        self.game_started = true;
        loop {
            let mut should_update = false;
            if rx.try_recv().is_ok() {
                self.local_board.next_tick();
                should_update = true;
            }
            while let Ok(command) = self.first_level_commands.try_recv() {
                if !self.game_started {
                    continue;
                }
                match command {
                    FirstLevelCommands::RightMove => self.local_board.move_right(),
                    FirstLevelCommands::LeftMove => self.local_board.move_left(),
                    FirstLevelCommands::ClockWiseRotation => self.local_board.rotation_clockwise(),
                    FirstLevelCommands::CounterClockWiseRotation => {
                        self.local_board.rotation_counterclockwise()
                    }
                    FirstLevelCommands::HardDrop => self.local_board.hard_drop(),
                    FirstLevelCommands::SoftDrop => self.local_board.soft_drop(),
                    FirstLevelCommands::SavePiece => self.local_board.save_piece(),
                    FirstLevelCommands::FullRotation => self.local_board.rotation_full(),
                }
                self.state_emit();
            }
            if should_update {
                self.state_emit();
            }
            tokio::time::sleep(Duration::from_micros(16_666)).await;
        }
    }
    async fn tick_loop(sender: Sender<bool>, receiver: Receiver<u128>) {
        tokio::spawn(async move {
            let mut points = 0;
            loop {
                tokio::time::sleep(Duration::from_secs(1)).await;
                sender.send(true).await.unwrap();
            }
        });
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

    fn emit_held_piece(&self, piece: Piece) {
        self.app.emit(HELD_PIECE_EMIT, piece).unwrap();
    }

    fn queue_emit(&self, pieces: [Piece; NUMBER_OF_PIECES_IN_QUEUE_TO_EMIT]) {
        self.app.emit(QUEUE_EMIT, pieces).unwrap();
    }

    fn startegy_emit(&self, strategy: Strategy) {
        self.app.emit(STRATEGY_EMIT, strategy).unwrap();
    }

    fn state_emit(&self) {
        self.app
            .emit(BOARD_STATE_EMIT, self.local_board.board_state())
            .unwrap();
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
// * - Other boards state -> not yet implemented
pub mod game_options;
