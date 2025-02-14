use tauri::Runtime;

#[tauri::command]
pub async fn right_move<R: Runtime>(app: tauri::AppHandle<R>, window: tauri::Window<R>) -> String {
    "".to_string()
}