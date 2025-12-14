use lighty_auth::{Authenticator, azuriom::AzuriomAuth};
use super::types::AuthResult;

#[tauri::command]
pub async fn authenticate_azuriom(url: String, username: String, password: String) -> Result<AuthResult, String> {
    let mut auth = AzuriomAuth::new(url, username, password);

    #[cfg(feature = "events")]
    let profile = auth.authenticate(None).await.map_err(|e| e.to_string())?;

    #[cfg(not(feature = "events"))]
    let profile = auth.authenticate().await.map_err(|e| e.to_string())?;

    Ok(profile.into())
}
