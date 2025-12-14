// Copyright (c) 2025 Hamadi
// Licensed under the MIT License

//! Core extraction events

use serde::{Deserialize, Serialize};

/// Core extraction events
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event")]
pub enum CoreEvent {
    /// Archive extraction started
    ExtractionStarted {
        archive_type: String, // "ZIP" | "TAR.GZ"
        file_count: usize,
        destination: String,
    },
    /// Extraction progress
    ExtractionProgress {
        files_extracted: usize,
        total_files: usize,
    },
    /// Extraction completed
    ExtractionCompleted {
        archive_type: String,
        files_extracted: usize,
    },
}
