use tauri::Runtime;

#[tauri::command]
pub async fn hard_drop<R: Runtime>(app: tauri::AppHandle<R>, window: tauri::Window<R>) -> String {
    "".to_string()
}