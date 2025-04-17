use std::sync::Arc;

use tauri::AppHandle;
use tokio::sync::{
    Mutex, OnceCell,
    mpsc::{self, Sender},
};

use crate::game::{FirstLevelCommands, Game, GameControl, game_options::GameOptions};

static FIRST_LEVEL_CHANNEL: OnceCell<Arc<Mutex<Sender<FirstLevelCommands>>>> =
    OnceCell::const_new();
static GAME_CONTROL_CHANNEL: OnceCell<Arc<Mutex<Sender<GameControl>>>> = OnceCell::const_new();

#[tauri::command]
pub async fn clockwise_rotation() {
    if let Some(channel) = FIRST_LEVEL_CHANNEL.get() {
        let _ = channel
            .lock()
            .await
            .send(FirstLevelCommands::ClockWiseRotation)
            .await;
    }
}

#[tauri::command]
pub async fn counter_clockwise_rotation() {
    if let Some(channel) = FIRST_LEVEL_CHANNEL.get() {
        let _ = channel
            .lock()
            .await
            .send(FirstLevelCommands::CounterClockWiseRotation)
            .await;
    }
}

#[tauri::command]
pub async fn forfeit_game() {
    if let Some(channel) = GAME_CONTROL_CHANNEL.get() {
        let _ = channel.lock().await.send(GameControl::Forfeit).await;
    }
}

#[tauri::command]
pub async fn full_rotation() {
    if let Some(channel) = FIRST_LEVEL_CHANNEL.get() {
        let _ = channel
            .lock()
            .await
            .send(FirstLevelCommands::FullRotation)
            .await;
    }
}

#[tauri::command]
pub async fn hard_drop() {
    if let Some(channel) = FIRST_LEVEL_CHANNEL.get() {
        let _ = channel
            .lock()
            .await
            .send(FirstLevelCommands::HardDrop)
            .await;
    }
}

#[tauri::command]
pub async fn left_move() {
    if let Some(channel) = FIRST_LEVEL_CHANNEL.get() {
        let _ = channel
            .lock()
            .await
            .send(FirstLevelCommands::LeftMove)
            .await;
    }
}

#[tauri::command]
pub async fn retry_game(app: AppHandle, options: GameOptions) {
    if let Some(channel) = GAME_CONTROL_CHANNEL.get() {
        let _ = channel.lock().await.send(GameControl::Retry).await;
        tokio::spawn(async move {
            start_game(app, options).await;
        });
    }
}

#[tauri::command]
pub async fn right_move() {
    if let Some(channel) = FIRST_LEVEL_CHANNEL.get() {
        let _ = channel
            .lock()
            .await
            .send(FirstLevelCommands::RightMove)
            .await;
    }
}

#[tauri::command]
pub async fn save_piece() {
    if let Some(channel) = FIRST_LEVEL_CHANNEL.get() {
        let _ = channel
            .lock()
            .await
            .send(FirstLevelCommands::SavePiece)
            .await;
    }
}

#[tauri::command]
pub async fn soft_drop() {
    if let Some(channel) = FIRST_LEVEL_CHANNEL.get() {
        let _ = channel
            .lock()
            .await
            .send(FirstLevelCommands::SoftDrop)
            .await;
    }
}

#[tauri::command]
pub async fn start_game(app: AppHandle, options: GameOptions) {
    let (tx, rx) = mpsc::channel(256);
    let (control_tx, control_rx) = mpsc::channel(256);
    if let Some(channel) = FIRST_LEVEL_CHANNEL.get() {
        let mut locked = channel.lock().await;
        *locked = tx;
    } else {
        FIRST_LEVEL_CHANNEL.set(Arc::new(Mutex::new(tx))).unwrap();
    }
    if let Some(channel) = GAME_CONTROL_CHANNEL.get() {
        let mut locked = channel.lock().await;
        *locked = control_tx;
    } else {
        GAME_CONTROL_CHANNEL
            .set(Arc::new(Mutex::new(control_tx)))
            .unwrap();
    }
    tokio::spawn(async move {
        let mut game = Game::new(options, app, rx, control_rx);
        game.start_game().await;
    });
}

#[tauri::command]
pub async fn targeting_strategy_eliminations() {
    if let Some(channel) = FIRST_LEVEL_CHANNEL.get() {
        // channel.clone().send(FirstLevelCommands::SoftDrop);
    }
}

#[tauri::command]
pub async fn targeting_strategy_even() {
    if let Some(channel) = FIRST_LEVEL_CHANNEL.get() {
        // channel.lock().await.targeting_strategy_elimination();
    }
}

#[tauri::command]
pub async fn targeting_strategy_payback() {
    if let Some(channel) = FIRST_LEVEL_CHANNEL.get() {
        // channel.lock().await.targeting_strategy_elimination();
    }
}

#[tauri::command]
pub async fn targeting_strategy_random() {
    if let Some(channel) = FIRST_LEVEL_CHANNEL.get() {
        // channel.lock().await.targeting_strategy_elimination();
    }
}
