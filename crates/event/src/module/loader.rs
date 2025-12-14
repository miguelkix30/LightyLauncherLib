// Copyright (c) 2025 Hamadi
// Licensed under the MIT License

//! Loader events (Vanilla, Fabric, Forge, etc.)

use serde::{Deserialize, Serialize};

/// Loader events (Vanilla, Fabric, Forge, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event")]
pub enum LoaderEvent {
    /// Fetching loader manifest
    FetchingData {
        loader: String,
        minecraft_version: String,
        loader_version: String,
    },
    /// Loader manifest retrieved
    DataFetched {
        loader: String,
        minecraft_version: String,
        loader_version: String,
    },
    /// Version not found (404)
    ManifestNotFound {
        loader: String,
        minecraft_version: String,
        loader_version: String,
        error: String,
    },
    /// Using cached manifest
    ManifestCached {
        loader: String,
    },
    /// Merging loader data (e.g., Fabric + Vanilla)
    MergingLoaderData {
        base_loader: String,
        overlay_loader: String,
    },
    /// Loader data merge completed
    DataMerged {
        base_loader: String,
        overlay_loader: String,
    },
}
