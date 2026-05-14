//! Global application state: qualifier / organization / application
//! identifiers and the derived [`directories::ProjectDirs`] used to locate
//! per-platform config and data directories.
//!
//! [`AppState::new`] must be called once at startup, before any function
//! that reads [`AppState::get_project_dirs`] or related getters.

use directories::ProjectDirs;
use once_cell::sync::{Lazy, OnceCell};
use crate::errors::{AppStateError, AppStateResult};

// ============================================
// STATIC INITIALIZATION
// ============================================

static PROJECT_DIRS: OnceCell<ProjectDirs> = OnceCell::new();
static ORGANIZATION: OnceCell<String> = OnceCell::new();
static QUALIFIER: OnceCell<String> = OnceCell::new();
static APPLICATION: OnceCell<String> = OnceCell::new();

fn get_project_dirs_clone() -> ProjectDirs {
    PROJECT_DIRS
        .get()
        .cloned()
        .expect("AppState::get_project_dirs() called before AppState::new()")
}

static LAZY_PROJECT_DIRS: Lazy<ProjectDirs> = Lazy::new(get_project_dirs_clone);

// ============================================
// APP STATE
// ============================================

/// Zero-sized handle to the global application state.
///
/// Holds no data — every method is associated, reading from process-wide
/// `OnceCell` storage. The instance returned by [`Self::new`] is only used
/// to enforce "called at least once" at the type level.
pub struct AppState;

impl AppState {
    /// Initialize the application state with launcher information
    ///
    /// # Arguments
    /// - `qualifier`: Reverse domain name notation (e.g., "com", "fr")
    /// - `organization`: Organization name (e.g., "MyCompany", ".LightyLauncher")
    /// - `application`: Application name (optional, can be empty)
    ///
    /// # Returns
    /// - `Ok(AppState)` on success
    /// - `Err(AppStateError)` if initialization fails
    pub fn new(qualifier: String, organization: String, application: String) -> AppStateResult<Self> {
        let project_dirs = ProjectDirs::from(&qualifier, &organization, &application)
            .ok_or(AppStateError::ProjectDirsCreation)?;

        PROJECT_DIRS.set(project_dirs)
            .map_err(|_| AppStateError::NotInitialized)?;

        QUALIFIER.set(qualifier)
            .map_err(|_| AppStateError::NotInitialized)?;

        ORGANIZATION.set(organization)
            .map_err(|_| AppStateError::NotInitialized)?;

        APPLICATION.set(application)
            .map_err(|_| AppStateError::NotInitialized)?;

        Ok(Self)
    }

    /// Returns the project directories (lazy-initialized).
    ///
    /// Panics if accessed before [`Self::new`] has been called.
    pub fn get_project_dirs() -> &'static Lazy<ProjectDirs> {
        &LAZY_PROJECT_DIRS
    }

    /// Get the application name for use in game arguments
    ///
    /// Uses the organization name, stripping leading dots/periods.
    /// Falls back to "LightyLauncher" if not initialized.
    ///
    /// # Example
    /// - Organization: ".LightyLauncher" → "LightyLauncher"
    /// - Organization: "MyCompany" → "MyCompany"
    pub fn get_app_name() -> String {
        ORGANIZATION
            .get()
            .map(|org| org.trim_start_matches('.').to_string())
            .unwrap_or_else(|| "LightyLauncher".to_string())
    }

    /// Get the application version from the package version
    ///
    /// Uses the compile-time CARGO_PKG_VERSION.
    pub fn get_app_version() -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    /// Returns the organization name, or `None` if [`Self::new`] hasn't been called.
    pub fn get_organization() -> Option<&'static String> {
        ORGANIZATION.get()
    }

    /// Returns the qualifier, or `None` if [`Self::new`] hasn't been called.
    pub fn get_qualifier() -> Option<&'static String> {
        QUALIFIER.get()
    }

    /// Returns the application name, or `None` if [`Self::new`] hasn't been called.
    pub fn get_application() -> Option<&'static String> {
        APPLICATION.get()
    }
}
