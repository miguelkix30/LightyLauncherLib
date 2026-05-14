// Copyright (c) 2025 Hamadi
// Licensed under the MIT License

//! File verification and cache checking utilities

use std::path::PathBuf;
use tokio::fs;
use lighty_core::verify_file_sha1;

/// Returns whether the file at `path` needs to be (re-)downloaded.
///
/// `true` if the file is missing, zero-byte (corrupt — a stale empty
/// stub from a previously-failed download), or the SHA1 doesn't match
/// `sha1`. `false` if the file exists, has non-zero size, and either
/// has no expected hash or matches the expected one.
pub async fn needs_download(path: &PathBuf, sha1: Option<&String>, name: &str) -> bool {
    if !path.exists() {
        return true;
    }

    // Empty files are a stale artifact of a previous failed download
    // (server returned 200 with empty body, or the writer crashed mid-stream).
    if let Ok(meta) = fs::metadata(path).await {
        if meta.len() == 0 {
            lighty_core::trace_warn!(
                "[Installer] Zero-byte cached file for {}, re-downloading...",
                name
            );
            let _ = fs::remove_file(path).await;
            return true;
        }
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
