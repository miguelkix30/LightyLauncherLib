use serde::{Deserialize, Serialize};

// Re-export AppState and errors from lighty-core
pub use lighty_core::{AppState, AppStateError, AppStateResult};

// Re-export from commands
pub use crate::commands::version::LightyVersionConfig;

// ============================================
// TYPES
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionConfig {
    pub name: String,
    pub loader: String,
    pub loader_version: String,
    pub minecraft_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LaunchConfig {
    pub username: String,
    pub uuid: String,
    pub java_distribution: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LaunchResult {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoaderInfo {
    pub name: String,
    pub display_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JavaDistInfo {
    pub name: String,
    pub display_name: String,
}
