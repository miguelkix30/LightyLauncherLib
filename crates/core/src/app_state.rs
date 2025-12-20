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

    /// Get the project directories (lazy-initialized)
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

    /// Get the organization name
    pub fn get_organization() -> Option<&'static String> {
        ORGANIZATION.get()
    }

    /// Get the qualifier
    pub fn get_qualifier() -> Option<&'static String> {
        QUALIFIER.get()
    }

    /// Get the application name
    pub fn get_application() -> Option<&'static String> {
        APPLICATION.get()
    }
}
