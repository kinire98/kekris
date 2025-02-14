use tauri::Runtime;

#[tauri::command]
pub async fn save_piece<R: Runtime>(app: tauri::AppHandle<R>, window: tauri::Window<R>) -> String {
    "".to_string()
}