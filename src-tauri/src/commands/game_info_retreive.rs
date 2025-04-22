use crate::{
    models::emit_game_info::EmitGameInfo,
    persistence::retreive_game_info::{
        retreive_last_blitz_info, retreive_last_classic_info, retreive_last_games_info,
        retreive_last_lines_info,
    },
};

#[tauri::command]
pub async fn retreive_game_info() -> EmitGameInfo {
    retreive_last_games_info().await
}

#[tauri::command]
pub async fn retreive_classic_game_info() -> EmitGameInfo {
    retreive_last_classic_info().await
}

#[tauri::command]
pub async fn retreive_lines_game_info() -> EmitGameInfo {
    retreive_last_lines_info().await
}

#[tauri::command]
pub async fn retreive_blitz_game_info() -> EmitGameInfo {
    retreive_last_blitz_info().await
}
