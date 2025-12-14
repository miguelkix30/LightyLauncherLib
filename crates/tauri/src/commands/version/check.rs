use crate::core::AppState;

#[tauri::command]
pub fn check_version_exists(version_name: String) -> bool {
    AppState::get_project_dirs()
        .data_dir()
        .join(&version_name)
        .exists()
}
