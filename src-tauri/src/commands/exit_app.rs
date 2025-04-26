use tauri::AppHandle;

#[tauri::command]
pub fn exit_app(app: AppHandle) {
    app.exit(0);
}
