use std::sync::Arc;

use tokio::{
    net::TcpStream,
    sync::{
        Mutex,
        mpsc::{Receiver, Sender},
    },
};

use crate::{
    commands::game_commands::{FIRST_LEVEL_CHANNEL, GAME_CONTROL_CHANNEL, SECOND_LEVEL_CHANNEL},
    models::{
        game_commands::{FirstLevelCommands, SecondLevelCommands},
        game_responses::GameResponses,
    },
};

use super::local_game::{GameControl, LocalGame};

pub struct ClientOnlineGame {
    socket: Arc<Mutex<TcpStream>>,
    running: bool,
    game_responses: Receiver<GameResponses>,
    tx_commands_second: Sender<SecondLevelCommands>,
}

impl ClientOnlineGame {
    pub fn new(socket: Arc<Mutex<TcpStream>>, pieces_buffer: Vec<Piece>) -> Self {
        let mut local_game = LocalGame::new(
            options,
            app,
            first_level_commands,
            second_level_commands,
            game_control_receiver,
            responder,
            queue,
        );
        Self {
            socket,
            running: true,
        }
    }
    async fn set_channels(
        tx_commands: Sender<FirstLevelCommands>,
        tx_commands_second: Sender<SecondLevelCommands>,
        tx_control: Sender<GameControl>,
    ) {
        if let Some(channel) = FIRST_LEVEL_CHANNEL.get() {
            let mut locked = channel.lock().await;
            *locked = tx_commands;
        } else {
            FIRST_LEVEL_CHANNEL
                .set(Arc::new(Mutex::new(tx_commands)))
                .unwrap();
        }
        if let Some(channel) = SECOND_LEVEL_CHANNEL.get() {
            let mut locked = channel.lock().await;
            *locked = tx_commands_second;
        } else {
            SECOND_LEVEL_CHANNEL
                .set(Arc::new(Mutex::new(tx_commands_second)))
                .unwrap();
        }
        if let Some(channel) = GAME_CONTROL_CHANNEL.get() {
            let mut locked = channel.lock().await;
            *locked = tx_control;
        } else {
            GAME_CONTROL_CHANNEL
                .set(Arc::new(Mutex::new(tx_control)))
                .unwrap();
        }
    }
    pub async fn start(&mut self) {
        while self.running {}
    }
}
