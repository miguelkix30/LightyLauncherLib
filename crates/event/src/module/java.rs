// Copyright (c) 2025 Hamadi
// Licensed under the MIT License

//! Java (JRE) events

use serde::{Deserialize, Serialize};

/// Java (JRE) events
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event")]
pub enum JavaEvent {
    /// JRE not installed
    JavaNotFound {
        distribution: String,
        version: u8,
    },
    /// JRE already installed (skip download)
    JavaAlreadyInstalled {
        distribution: String,
        version: u8,
        binary_path: String,
    },
    /// JRE download started
    JavaDownloadStarted {
        distribution: String,
        version: u8,
        total_bytes: u64,
    },
    /// JRE download progress
    JavaDownloadProgress {
        bytes: u64,
    },
    /// JRE download completed
    JavaDownloadCompleted {
        distribution: String,
        version: u8,
    },
    /// JRE extraction started
    JavaExtractionStarted {
        distribution: String,
        version: u8,
    },
    /// JRE extraction progress
    JavaExtractionProgress {
        files_extracted: usize,
        total_files: usize,
    },
    /// JRE extraction completed
    JavaExtractionCompleted {
        distribution: String,
        version: u8,
        binary_path: String,
    },
}
