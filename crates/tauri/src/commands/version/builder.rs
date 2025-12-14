use crate::core::AppState;

#[tauri::command]
pub fn delete_version(version_name: String) -> Result<(), String> {
    let version_path = AppState::get_project_dirs()
        .data_dir()
        .join(&version_name);

    if version_path.exists() {
        std::fs::remove_dir_all(version_path)
            .map_err(|e| format!("Failed to delete version: {}", e))?;
        Ok(())
    } else {
        Err(format!("Version '{}' does not exist", version_name))
    }
}
