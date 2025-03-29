use init_trace::initialize;
use tauri::Manager;
mod commands;
pub mod game;
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
            commands::start_game,
            commands::clockwise_rotation,
            commands::left_move,
            commands::right_move,
            commands::counter_clockwise_rotation,
            commands::full_rotation,
            commands::save_piece,
            commands::hard_drop,
            commands::soft_drop,
            commands::forfeit_game,
            commands::retry_game,
            commands::targeting_strategy_even,
            commands::targeting_strategy_eliminations,
            commands::targeting_strategy_random,
            commands::targeting_strategy_payback,
            commands::pause_game,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
