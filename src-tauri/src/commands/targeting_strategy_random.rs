use tauri::Runtime;

#[tauri::command]
pub async fn targeting_strategy_random<R: Runtime>(app: tauri::AppHandle<R>, window: tauri::Window<R>) -> String {
    "".to_string()
}