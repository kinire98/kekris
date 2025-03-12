use std::sync::Arc;

use tauri::{AppHandle, async_runtime::Mutex};
use tokio::sync::OnceCell;

use crate::game::{Game, GameOptions};

static BOARD: OnceCell<Arc<Mutex<Game>>> = OnceCell::const_new();

#[tauri::command]
pub async fn clockwise_rotation() -> String {
    if let Some(board) = BOARD.get() {
        board.lock().await.clockwise_rotation().await
    } else {
        String::new()
    }
}

#[tauri::command]
pub async fn counter_clockwise_rotation() -> String {
    if let Some(board) = BOARD.get() {
        board.lock().await.counter_clockwise_rotation().await
    } else {
        String::new()
    }
}

#[tauri::command]
pub async fn forfeit_game() {
    if let Some(board) = BOARD.get() {
        board.lock().await.forfeit_game();
    }
}

#[tauri::command]
pub async fn full_rotation() -> String {
    if let Some(board) = BOARD.get() {
        board.lock().await.full_rotation().await
    } else {
        String::new()
    }
}

#[tauri::command]
pub async fn hard_drop() -> String {
    if let Some(board) = BOARD.get() {
        board.lock().await.hard_drop().await
    } else {
        String::new()
    }
}

#[tauri::command]
pub async fn left_move() -> String {
    if let Some(board) = BOARD.get() {
        board.lock().await.hard_drop().await
    } else {
        String::new()
    }
}

#[tauri::command]
pub async fn retry_game() {
    if let Some(board) = BOARD.get() {
        board.lock().await.retry_game();
    }
}

#[tauri::command]
pub async fn right_move() -> String {
    if let Some(board) = BOARD.get() {
        board.lock().await.right_move().await
    } else {
        String::new()
    }
}

#[tauri::command]
pub async fn save_piece() {
    if let Some(board) = BOARD.get() {
        board.lock().await.save_piece().await;
    }
}

#[tauri::command]
pub async fn soft_drop() -> String {
    if let Some(board) = BOARD.get() {
        board.lock().await.soft_drop().await
    } else {
        String::new()
    }
}

#[tauri::command]
pub async fn start_game(app: AppHandle, options: GameOptions) {
    let game = Game::new(options, app);
    BOARD.set(Arc::new(Mutex::new(game))).unwrap();
}

#[tauri::command]
pub async fn targeting_strategy_eliminations() {
    if let Some(board) = BOARD.get() {
        board.lock().await.targeting_strategy_elimination().await;
    }
}

#[tauri::command]
pub async fn targeting_strategy_even() {
    if let Some(board) = BOARD.get() {
        board.lock().await.targeting_strategy_even().await;
    }
}

#[tauri::command]
pub async fn targeting_strategy_payback() {
    if let Some(board) = BOARD.get() {
        board.lock().await.targeting_strategy_payback().await;
    }
}

#[tauri::command]
pub async fn targeting_strategy_random() {
    if let Some(board) = BOARD.get() {
        board.lock().await.targeting_strategy_random().await;
    }
}
