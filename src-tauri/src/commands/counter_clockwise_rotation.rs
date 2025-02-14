use tauri::Runtime;

#[tauri::command]
pub async fn counter_clockwise_rotation<R: Runtime>(app: tauri::AppHandle<R>, window: tauri::Window<R>) -> String {
    "".to_string()
}