use tauri::Manager;
use init_trace::initialize;
mod commands;
mod game;
mod init_trace;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    initialize();
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_webview_window("main").unwrap();
            main_window.minimize().unwrap();
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::start_game::start_game,
            commands::clockwise_rotation::clockwise_rotation,
            commands::left_move::left_move,
            commands::right_move::right_move,
            commands::counter_clockwise_rotation::counter_clockwise_rotation,
            commands::full_rotation::full_rotation,
            commands::save_piece::save_piece,
            commands::hard_drop::hard_drop,
            commands::soft_drop::soft_drop,
            commands::forfeit_game::forfeit_game,
            commands::retry_game::retry_game,
            commands::targeting_strategy_even::targeting_strategy_even,
            commands::targeting_strategy_eliminations::targeting_strategy_eliminations,
            commands::targeting_strategy_random::targeting_strategy_random,
            commands::targeting_strategy_payback::targeting_strategy_payback,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
