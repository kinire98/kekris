use init_trace::initialize;
use persistence::migrations::run_migrations;
use tauri::Manager;
mod commands;
pub mod game;
pub mod globals;
pub mod helpers;
mod init_trace;
pub mod models;
pub mod persistence;
pub mod room;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_webview_window("main").unwrap();
            #[cfg(dev)]
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
            commands::game_info_retreive::retreive_game_info,
            commands::game_info_retreive::retreive_classic_game_info,
            commands::game_info_retreive::retreive_lines_game_info,
            commands::game_info_retreive::retreive_blitz_game_info,
            commands::exit_app::exit_app,
            commands::room_commands::server::create_room,
            commands::room_commands::server::close_room,
            commands::room_commands::server::start_online_game,
            commands::room_commands::client::listen_for_rooms,
            commands::room_commands::client::join_room,
            commands::room_commands::client::leave_room,
            commands::room_commands::client::room_info,
            commands::room_commands::client::stop_search,
            commands::room_commands::can_host_room
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    initialize();
}
