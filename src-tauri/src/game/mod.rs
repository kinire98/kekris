use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use board::local_board::LocalBoard;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;

pub mod board;
mod pieces;
pub mod queue;
mod strategy;

#[derive(Debug)]
pub struct Game {
    app: AppHandle,
    local_board: LocalBoard,
}

impl Game {
    pub fn new(options: GameOptions, app: AppHandle) -> Self {
        todo!()
    }

    pub async fn clockwise_rotation(&mut self) -> String {
        todo!()
    }

    pub async fn counter_clockwise_rotation(&mut self) -> String {
        todo!()
    }

    pub fn forfeit_game(&self) {
        todo!()
    }

    pub async fn full_rotation(&mut self) -> String {
        todo!()
    }

    pub async fn hard_drop(&mut self) -> String {
        todo!()
    }

    pub async fn left_move(&mut self) -> String {
        todo!()
    }

    pub fn retry_game(&self) {
        todo!()
    }

    pub async fn right_move(&mut self) -> String {
        todo!()
    }

    pub async fn soft_drop(&mut self) -> String {
        todo!()
    }

    pub async fn targeting_strategy_elimination(&mut self) {
        todo!()
    }

    pub async fn targeting_strategy_even(&mut self) {
        todo!()
    }

    pub async fn targeting_strategy_random(&mut self) {
        todo!()
    }

    pub async fn targeting_strategy_payback(&mut self) {
        todo!()
    }

    pub async fn save_piece(&mut self) {
        todo!()
    }

    fn win_condition_normal(game_lost: bool, lines_cleared: u32) -> bool {
        false
    }
}

#[derive(Serialize, Deserialize)]
pub struct GameOptions {
    number_of_players: u8,
    lines_40: bool,
    blitz: bool,
    normal: bool,
}
impl Default for GameOptions {
    fn default() -> Self {
        Self {
            number_of_players: 1,
            lines_40: false,
            blitz: false,
            normal: true,
        }
    }
}
impl GameOptions {
    pub fn normal(&mut self) {
        self.lines_40 = false;
        self.blitz = false;
        self.normal = true;
    }
    pub fn blitz(&mut self) {
        self.lines_40 = false;
        self.blitz = true;
        self.normal = false;
    }
    pub fn lines_40(&mut self) {
        self.lines_40 = true;
        self.blitz = false;
        self.normal = false;
    }
    pub fn single_player(&mut self) {
        self.number_of_players = 1;
    }
    pub fn multi_player(&mut self, players: u8) {
        if players < 2 {
            panic!("In multiplayer should be more than one player");
        }
        self.number_of_players = players;
    }
}
