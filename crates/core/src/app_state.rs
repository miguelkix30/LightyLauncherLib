use directories::ProjectDirs;
use once_cell::sync::{Lazy, OnceCell};
use crate::errors::{AppStateError, AppStateResult};

// ============================================
// STATIC INITIALIZATION
// ============================================

static PROJECT_DIRS: OnceCell<ProjectDirs> = OnceCell::new();

fn get_project_dirs_clone() -> ProjectDirs {
    PROJECT_DIRS
        .get()
        .cloned()
        .unwrap_or_else(|| {
            panic!("AppState::get_project_dirs() called before AppState::new()")
        })
}

static LAZY_PROJECT_DIRS: Lazy<ProjectDirs> = Lazy::new(get_project_dirs_clone);

// ============================================
// APP STATE
// ============================================

pub struct AppState;

impl AppState {
    pub fn new(qualifier: String, organization: String, application: String) -> AppStateResult<Self> {
        let project_dirs = ProjectDirs::from(&qualifier, &organization, &application)
            .ok_or(AppStateError::ProjectDirsCreation)?;

        PROJECT_DIRS.set(project_dirs)
            .map_err(|_| AppStateError::NotInitialized)?;

        Ok(Self)
    }

    pub fn get_project_dirs() -> &'static Lazy<ProjectDirs> {
        &LAZY_PROJECT_DIRS
    }
}
