pub(crate) mod system;
pub(crate) mod macros;
pub(crate) mod hosts;
pub(crate) mod download;
pub(crate) mod extract;
pub(crate) mod errors;

// Re-export error types for easy access
pub use errors::{
    SystemError, SystemResult,
    ExtractError, ExtractResult,
    DownloadError, DownloadResult,
};