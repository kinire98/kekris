use crate::{
    models::emit_game_info::EmitGameInfo, persistence::retreive_game_info::retreive_last_games_info,
};

#[tauri::command]
pub async fn retreive_game_info() -> EmitGameInfo {
    retreive_last_games_info().await
}
