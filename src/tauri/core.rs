use directories::ProjectDirs;
use once_cell::sync::{Lazy, OnceCell};
use serde::{Deserialize, Serialize};

// ============================================
// STATIC INITIALIZATION
// ============================================

static PROJECT_DIRS: OnceCell<ProjectDirs> = OnceCell::new();

fn get_project_dirs_clone() -> ProjectDirs {
    PROJECT_DIRS
        .get()
        .expect("AppState not initialized")
        .clone()
}

static LAZY_PROJECT_DIRS: Lazy<ProjectDirs> = Lazy::new(get_project_dirs_clone);

// ============================================
// APP STATE
// ============================================

pub struct AppState;

impl AppState {
    pub fn new(qualifier: String, organization: String, application: String) -> Self {
        PROJECT_DIRS.get_or_init(|| {
            ProjectDirs::from(&qualifier, &organization, &application)
                .expect("Failed to create project directories")
        });

        Self
    }

    pub fn get_project_dirs() -> &'static Lazy<ProjectDirs> {
        &LAZY_PROJECT_DIRS
    }
}

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
