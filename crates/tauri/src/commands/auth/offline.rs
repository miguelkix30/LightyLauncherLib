use lighty_auth::{Authenticator, offline::OfflineAuth};
use super::types::AuthResult;

#[tauri::command]
pub async fn authenticate_offline(username: String) -> Result<AuthResult, String> {
    let mut auth = OfflineAuth::new(username);

    #[cfg(feature = "events")]
    let profile = auth.authenticate(None).await.map_err(|e| e.to_string())?;

    #[cfg(not(feature = "events"))]
    let profile = auth.authenticate().await.map_err(|e| e.to_string())?;

    Ok(profile.into())
}
