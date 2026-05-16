// Copyright (c) 2025 Hamadi
// Licensed under the MIT License

//! Serde mirrors of the Modrinth Labrinth-API JSON payloads.
//!
//! Documented at <https://docs.modrinth.com/api/>. Only the fields the
//! resolver actually reads are deserialized — the API ships a large
//! amount of metadata we don't care about (changelog, author, etc.).

use serde::Deserialize;

/// `GET /version/{id}` or one element of
/// `GET /project/{slug}/version`.
#[derive(Debug, Deserialize)]
pub struct ModrinthVersion {
    pub id: String,
    pub version_number: String,
    pub files: Vec<ModrinthFile>,
    #[serde(default)]
    pub dependencies: Vec<ModrinthDependency>,
}

/// One downloadable asset attached to a version (jar, source jar, …).
#[derive(Debug, Deserialize)]
pub struct ModrinthFile {
    pub url: String,
    pub filename: String,
    pub size: u64,
    pub hashes: ModrinthHashes,
    /// `true` for the build artifact, `false` for source/dev jars.
    /// The resolver picks the `primary` file when present, otherwise
    /// falls back to the first entry.
    #[serde(default)]
    pub primary: bool,
}

/// Hashes Modrinth publishes alongside each file.
#[derive(Debug, Deserialize)]
pub struct ModrinthHashes {
    pub sha1: String,
}

/// One declared dependency.
#[derive(Debug, Deserialize)]
pub struct ModrinthDependency {
    /// Project id (Modrinth XKCD-style id, e.g. `"P7dR8mSH"`). Absent
    /// for "version-only" deps (rare); the resolver skips those with a
    /// debug log rather than do an extra lookup.
    #[serde(default)]
    pub project_id: Option<String>,
    /// Version id pin (e.g. `"PpRTuoEh"`). Absent ⇒ resolve latest.
    #[serde(default)]
    pub version_id: Option<String>,
    /// `"required"` | `"optional"` | `"incompatible"` | `"embedded"`.
    /// Only `"required"` is followed by the BFS.
    pub dependency_type: String,
}
