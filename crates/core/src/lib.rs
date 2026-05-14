//! Foundation utilities shared by every Lighty Launcher crate.
//!
//! This crate has no `lighty-*` dependencies; everything else depends on it.
//! It groups the cross-cutting helpers needed throughout the launcher:
//! [`AppState`] for project directories, an HTTP client and host-file guard
//! in [`hosts`], async [`download`] / [`extract`] / [`hash`] helpers, an OS
//! and architecture detection layer in [`system`], and conditional logging
//! [`macros`].

pub mod system;
pub mod macros;
pub mod hosts;
pub mod download;
pub mod extract;
pub mod hash;
pub mod errors;
pub mod app_state;

// Re-export error types for easy access
pub use errors::{
    SystemError, SystemResult,
    ExtractError, ExtractResult,
    DownloadError, DownloadResult,
    AppStateError, AppStateResult,
};

// Re-export hash types for easy access
pub use hash::{
    HashError, HashResult,
    verify_file_sha1, verify_file_sha1_streaming,
    calculate_file_sha1_sync, verify_file_sha1_sync,
    calculate_sha1_bytes, calculate_sha1_bytes_raw,
};

// Re-export app state
pub use app_state::AppState;