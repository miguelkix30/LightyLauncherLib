use crate::tauri::core::AppState;

#[tauri::command]
pub fn get_launcher_path() -> String {
    AppState::get_project_dirs()
        .data_dir()
        .to_string_lossy()
        .to_string()
}
