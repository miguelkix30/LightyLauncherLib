pub mod system;
pub mod macros;
pub mod hosts;
pub mod download;
pub mod extract;
pub mod hash;
pub mod errors;

// Re-export error types for easy access
pub use errors::{
    SystemError, SystemResult,
    ExtractError, ExtractResult,
    DownloadError, DownloadResult,
};

// Re-export hash types for easy access
pub use hash::{
    HashError, HashResult,
    verify_file_sha1, verify_file_sha1_streaming,
    calculate_file_sha1_sync, verify_file_sha1_sync,
};