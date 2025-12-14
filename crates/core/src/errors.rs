use thiserror::Error;

/// Errors related to operating system and architecture detection
#[derive(Debug, Error)]
pub enum SystemError {
    #[error("Unsupported operating system")]
    UnsupportedOS,

    #[error("Unsupported architecture")]
    UnsupportedArchitecture,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Errors related to file extraction (ZIP and tar.gz)
#[derive(Debug, Error)]
pub enum ExtractError {
    #[error("Invalid ZIP entry at index {index}")]
    ZipEntryNotFound { index: usize },

    #[error("Path has no parent directory")]
    InvalidPath,

    #[error("Absolute paths not allowed: {path}")]
    AbsolutePath { path: String },

    #[error("Path traversal detected: {path} escapes base directory")]
    PathTraversal { path: String },

    #[error("File too large: {size} bytes (max: {max})")]
    FileTooLarge { size: u64, max: u64 },

    #[error("ZIP error: {0}")]
    Zip(#[from] async_zip::error::ZipError),

    #[error("TAR error: {0}")]
    Tar(#[from] std::io::Error),
}

/// Errors related to HTTP downloads
#[derive(Debug, Error)]
pub enum DownloadError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Type alias for System operations results
pub type SystemResult<T> = Result<T, SystemError>;

/// Type alias for extraction operations results
pub type ExtractResult<T> = Result<T, ExtractError>;

/// Errors related to application state initialization
#[derive(Debug, Error)]
pub enum AppStateError {
    #[error("Failed to create project directories")]
    ProjectDirsCreation,

    #[error("AppState not initialized")]
    NotInitialized,
}

/// Type alias for download operations results
pub type DownloadResult<T> = Result<T, DownloadError>;

/// Type alias for app state operations results
pub type AppStateResult<T> = Result<T, AppStateError>;
