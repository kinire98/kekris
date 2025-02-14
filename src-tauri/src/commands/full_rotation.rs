use tauri::Runtime;

#[tauri::command]
pub async fn full_rotation<R: Runtime>(app: tauri::AppHandle<R>, window: tauri::Window<R>) -> String {
    "".to_string()
}