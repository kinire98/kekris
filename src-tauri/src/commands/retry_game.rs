use tauri::Runtime;

#[tauri::command]
pub async fn retry_game<R: Runtime>(app: tauri::AppHandle<R>, window: tauri::Window<R>) -> String {
    "".to_string()
}