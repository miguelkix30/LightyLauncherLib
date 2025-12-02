use thiserror::Error;

/// Errors related to Java Runtime Environment (JRE) operations
#[derive(Debug, Error)]
pub enum JreError {
    #[error("JRE not found at {path}")]
    NotFound { path: String },

    #[error("Invalid JRE structure in directory")]
    InvalidStructure,

    #[error("Download failed: {0}")]
    Download(String),

    #[error("Unsupported operating system for JRE installation")]
    UnsupportedOS,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Extraction failed: {0}")]
    Extraction(String),
}

/// Errors related to Java runtime execution
#[derive(Debug, Error)]
pub enum JavaRuntimeError {
    #[error("Java runtime not found at {path}")]
    NotFound { path: String },

    #[error("Process exited with non-zero exit code: {code}")]
    NonZeroExit { code: i32 },

    #[error("Failed to capture process I/O - stdout/stderr not configured")]
    IoCaptureFailure,

    #[error("Process spawn error: {0}")]
    Spawn(#[from] std::io::Error),

    #[error("Process terminated by signal")]
    SignalTerminated,
}

/// Errors related to Java distribution management
#[derive(Debug, Error)]
pub enum DistributionError {
    #[error("Unsupported Java version {version} for distribution {distribution}")]
    UnsupportedVersion { version: u32, distribution: String },

    #[error("System error: {0}")]
    System(#[from] crate::utils::SystemError),
}

/// Type alias for JRE operations results
pub type JreResult<T> = Result<T, JreError>;

/// Type alias for Java runtime operations results
pub type JavaRuntimeResult<T> = Result<T, JavaRuntimeError>;

/// Type alias for Java distribution operations results
pub type DistributionResult<T> = Result<T, DistributionError>;
