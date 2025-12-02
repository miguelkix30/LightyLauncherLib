pub mod system;
pub mod macros;
pub mod hosts;
pub mod download;
pub mod extract;
pub mod errors;

// Re-export error types for easy access
pub use errors::{
    SystemError, SystemResult,
    ExtractError, ExtractResult,
    DownloadError, DownloadResult,
};