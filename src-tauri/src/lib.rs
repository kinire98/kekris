use init_trace::initialize;
use persistence::migrations::run_migrations;
use tauri::Manager;
mod commands;
pub mod game;
mod init_trace;
pub mod persistence;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    initialize();

    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_webview_window("main").unwrap();
            main_window.minimize().unwrap();
            let handle = app.handle().clone();
            tokio::spawn(async move {
                run_migrations(handle).await;
            });
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::game_commands::start_game,
            commands::game_commands::clockwise_rotation,
            commands::game_commands::left_move,
            commands::game_commands::right_move,
            commands::game_commands::counter_clockwise_rotation,
            commands::game_commands::full_rotation,
            commands::game_commands::save_piece,
            commands::game_commands::hard_drop,
            commands::game_commands::soft_drop,
            commands::game_commands::forfeit_game,
            commands::game_commands::retry_game,
            commands::game_commands::targeting_strategy_even,
            commands::game_commands::targeting_strategy_eliminations,
            commands::game_commands::targeting_strategy_random,
            commands::game_commands::targeting_strategy_payback,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
