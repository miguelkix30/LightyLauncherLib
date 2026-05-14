// Copyright (c) 2025 Hamadi
// Licensed under the MIT License

//! File verification and cache checking utilities

use std::path::PathBuf;
use tokio::fs;
use lighty_core::verify_file_sha1;

/// Returns whether the file at `path` needs to be (re-)downloaded.
///
/// `true` if the file is missing, or the SHA1 doesn't match `sha1` (in
/// which case the stale file is removed). `false` if the file exists and
/// either has no expected hash or matches the expected one.
pub async fn needs_download(path: &PathBuf, sha1: Option<&String>, name: &str) -> bool {
    if !path.exists() {
        return true;
    }

    if let Some(hash) = sha1 {
        match verify_file_sha1(path, hash).await {
            Ok(true) => false,
            _ => {
                lighty_core::trace_warn!("[Installer] SHA1 mismatch for {}, re-downloading...", name);
                let _ = fs::remove_file(path).await;
                true
            }
        }
    } else {
        false
    }
}
