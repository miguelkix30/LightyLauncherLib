use crate::core::AppState;

#[tauri::command]
pub fn init_app_state(qualifier: String, organization: String, application: String) -> Result<(), String> {
    AppState::new(qualifier, organization, application)
        .map(|_| ())
        .map_err(|e| e.to_string())
}
