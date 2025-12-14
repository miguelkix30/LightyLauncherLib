use lighty_auth::{Authenticator, microsoft::MicrosoftAuth};
use super::types::AuthResult;

#[tauri::command]
pub async fn authenticate_microsoft(client_id: String) -> Result<AuthResult, String> {
    let mut auth = MicrosoftAuth::new(client_id);

    #[cfg(feature = "events")]
    let profile = auth.authenticate(None).await.map_err(|e| e.to_string())?;

    #[cfg(not(feature = "events"))]
    let profile = auth.authenticate().await.map_err(|e| e.to_string())?;

    Ok(profile.into())
}
