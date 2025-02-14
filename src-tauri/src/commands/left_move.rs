use tauri::Runtime;

#[tauri::command]
pub async fn left_move<R: Runtime>(app: tauri::AppHandle<R>, window: tauri::Window<R>) -> String {
    "".to_string()
}