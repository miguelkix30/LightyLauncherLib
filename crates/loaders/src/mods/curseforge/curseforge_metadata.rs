// Copyright (c) 2025 Hamadi
// Licensed under the MIT License

//! Serde mirrors of the CurseForge Core-API JSON payloads.
//!
//! Documented at <https://docs.curseforge.com/>. Every response is
//! wrapped in `{ "data": … }` so [`CurseForgeEnvelope`] is the entry
//! point for both single-file and file-list endpoints.

use serde::Deserialize;

/// Modloader codes returned in CurseForge file metadata.
pub const MOD_LOADER_FORGE: u8 = 1;
pub const MOD_LOADER_FABRIC: u8 = 4;
pub const MOD_LOADER_QUILT: u8 = 5;
pub const MOD_LOADER_NEOFORGE: u8 = 6;

/// Hash algorithm codes inside [`CurseForgeHash::algo`].
pub const HASH_ALGO_SHA1: u8 = 1;
#[allow(dead_code)] // documented for completeness, unused today
pub const HASH_ALGO_MD5: u8 = 2;

/// Dependency relation codes inside [`CurseForgeDependency::relation_type`].
#[allow(dead_code)]
pub const DEP_EMBEDDED_LIBRARY: u8 = 1;
#[allow(dead_code)]
pub const DEP_OPTIONAL: u8 = 2;
pub const DEP_REQUIRED: u8 = 3;
#[allow(dead_code)]
pub const DEP_TOOL: u8 = 4;
#[allow(dead_code)]
pub const DEP_INCOMPATIBLE: u8 = 5;
#[allow(dead_code)]
pub const DEP_INCLUDE: u8 = 6;

/// Standard `{ "data": <T> }` envelope every endpoint returns.
#[derive(Debug, Deserialize)]
pub struct CurseForgeEnvelope<T> {
    pub data: T,
}

/// Single file entry returned by `GET /mods/{modId}/files` or
/// `GET /mods/{modId}/files/{fileId}`.
#[derive(Debug, Deserialize)]
#[allow(dead_code)] // some fields kept for diagnostics / future use
pub struct CurseForgeFile {
    pub id: u32,
    #[serde(rename = "modId")]
    pub mod_id: u32,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "fileName")]
    pub file_name: String,
    /// `None` when the project disables third-party distribution —
    /// the resolver surfaces [`crate::utils::error::QueryError::ModDistributionForbidden`]
    /// in that case.
    #[serde(rename = "downloadUrl", default)]
    pub download_url: Option<String>,
    #[serde(rename = "fileLength", default)]
    pub file_length: u64,
    #[serde(default)]
    pub hashes: Vec<CurseForgeHash>,
    #[serde(default)]
    pub dependencies: Vec<CurseForgeDependency>,
}

/// File checksum. Multiple algos may be listed; the resolver picks the
/// SHA1 entry to match the launcher's existing SHA1-based verifier.
#[derive(Debug, Deserialize)]
pub struct CurseForgeHash {
    pub value: String,
    pub algo: u8,
}

/// Declared mod dependency. `relation_type` follows the constants
/// above — only `DEP_REQUIRED` is followed by the BFS resolver.
#[derive(Debug, Deserialize)]
pub struct CurseForgeDependency {
    #[serde(rename = "modId")]
    pub mod_id: u32,
    #[serde(rename = "relationType")]
    pub relation_type: u8,
}
