use serde::{Deserialize, Serialize};
use lighty_auth::UserProfile;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthConfig {
    pub auth_type: String,
    pub username: Option<String>,
    pub azuriom_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthResult {
    pub username: String,
    pub uuid: String,
    pub access_token: Option<String>,
}

impl From<UserProfile> for AuthResult {
    fn from(profile: UserProfile) -> Self {
        Self {
            username: profile.username,
            uuid: profile.uuid,
            access_token: profile.access_token,
        }
    }
}
