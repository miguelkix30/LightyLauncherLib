// Copyright (c) 2025 Hamadi
// Licensed under the MIT License

//! CurseForge Core-API client.
//!
//! Base at `https://api.curseforge.com/v1`. Requires an API key set via
//! [`set_api_key`] before any call (env-var auto-loading is intentionally
//! NOT performed — callers control when the key is wired in). Failing
//! to set it before a [`fetch`] call produces a clear runtime error.
//!
//! Wire-format structs live in [`super::curseforge_metadata`].

use std::sync::RwLock;

use once_cell::sync::Lazy;

use lighty_core::hosts::HTTP_CLIENT as CLIENT;

use crate::types::version_metadata::Mods;
use crate::types::Loader;
use crate::utils::error::QueryError;

use super::curseforge_metadata::{
    CurseForgeDependency, CurseForgeEnvelope, CurseForgeFile, DEP_REQUIRED, HASH_ALGO_SHA1,
    MOD_LOADER_FABRIC, MOD_LOADER_FORGE, MOD_LOADER_NEOFORGE, MOD_LOADER_QUILT,
};
use crate::mods::request::ModRequest;

const BASE_URL: &str = "https://api.curseforge.com/v1";
const PROVIDER: &str = "curseforge";

/// Process-wide API key, set before the first [`fetch`] runs.
///
/// Stored in an `RwLock` rather than reading an env var so callers can
/// load secrets from a config file / vault / GUI prompt at whatever
/// moment suits them.
static API_KEY: Lazy<RwLock<Option<String>>> = Lazy::new(|| RwLock::new(None));

/// Configures the CurseForge API key used by every subsequent fetch.
///
/// Call once, before any [`crate::mods::request::ModRequest::CurseForge`]
/// reaches [`fetch`]. Calling twice overrides the previous key.
pub fn set_api_key(key: impl Into<String>) {
    let mut guard = API_KEY.write().expect("CurseForge API key lock poisoned");
    *guard = Some(key.into());
}

/// Resolves a `ModRequest::CurseForge { … }` into a pivot [`Mods`]
/// entry plus the list of required dependencies (relation type 3) to
/// enqueue.
pub async fn fetch(
    request: &ModRequest,
    minecraft_version: &str,
    loader: &Loader,
) -> Result<(Mods, Vec<ModRequest>), QueryError> {
    let (mod_id, pinned_file) = match request {
        ModRequest::CurseForge { mod_id, file_id } => (*mod_id, *file_id),
        _ => unreachable!("curseforge::fetch called with a non-CurseForge request"),
    };

    let api_key = read_api_key()?;

    let file = if let Some(file_id) = pinned_file {
        fetch_pinned_file(mod_id, file_id, &api_key).await?
    } else {
        let mod_loader_code = mod_loader_code(loader)?;
        fetch_latest_compatible(mod_id, minecraft_version, mod_loader_code, &api_key).await?
    };

    let pivot = into_pivot(&file)?;
    let dependencies = collect_required_dependencies(&file);

    Ok((pivot, dependencies))
}

/// Returns the configured key or a clear error pointing at [`set_api_key`].
fn read_api_key() -> Result<String, QueryError> {
    let guard = API_KEY.read().expect("CurseForge API key lock poisoned");
    guard.clone().ok_or_else(|| QueryError::Conversion {
        message: "CurseForge API key not configured. Call \
                  `lighty_loaders::mods::curseforge::set_api_key(...)` \
                  before launching an instance with `.with_curseforge(...)`"
            .to_string(),
    })
}

/// Maps a [`Loader`] to its CurseForge numeric `modLoaderType` code.
fn mod_loader_code(loader: &Loader) -> Result<u8, QueryError> {
    match loader {
        Loader::Fabric => Ok(MOD_LOADER_FABRIC),
        Loader::Forge => Ok(MOD_LOADER_FORGE),
        Loader::NeoForge => Ok(MOD_LOADER_NEOFORGE),
        Loader::Quilt => Ok(MOD_LOADER_QUILT),
        Loader::Vanilla | Loader::Optifine | Loader::LightyUpdater => {
            Err(QueryError::UnsupportedLoader(format!(
                "CurseForge doesn't host mods for {:?} instances",
                loader
            )))
        }
    }
}

async fn fetch_pinned_file(
    mod_id: u32,
    file_id: u32,
    api_key: &str,
) -> Result<CurseForgeFile, QueryError> {
    let url = format!("{}/mods/{}/files/{}", BASE_URL, mod_id, file_id);
    let response = CLIENT.get(&url).header("x-api-key", api_key).send().await?;
    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Err(QueryError::ModNotFound {
            provider: PROVIDER,
            id: format!("{}/{}", mod_id, file_id),
        });
    }
    let envelope = response
        .error_for_status()?
        .json::<CurseForgeEnvelope<CurseForgeFile>>()
        .await?;
    Ok(envelope.data)
}

async fn fetch_latest_compatible(
    mod_id: u32,
    minecraft_version: &str,
    mod_loader_code: u8,
    api_key: &str,
) -> Result<CurseForgeFile, QueryError> {
    // Manual query-string build to avoid pulling `serde_urlencoded` in.
    let url = format!(
        "{}/mods/{}/files?gameVersion={}&modLoaderType={}",
        BASE_URL,
        mod_id,
        url_encode(minecraft_version),
        mod_loader_code
    );

    let response = CLIENT.get(&url).header("x-api-key", api_key).send().await?;
    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Err(QueryError::ModNotFound {
            provider: PROVIDER,
            id: mod_id.to_string(),
        });
    }
    let envelope = response
        .error_for_status()?
        .json::<CurseForgeEnvelope<Vec<CurseForgeFile>>>()
        .await?;
    envelope
        .data
        .into_iter()
        .next()
        .ok_or_else(|| QueryError::ModIncompatible {
            provider: PROVIDER,
            id: mod_id.to_string(),
            mc: minecraft_version.to_string(),
            loader: mod_loader_code.to_string(),
        })
}

/// Converts a CurseForge file entry into the launcher's pivot [`Mods`]
/// type. `path` is the bare filename — the mods installer prefixes it
/// with `{runtime_dir}/mods/`.
fn into_pivot(file: &CurseForgeFile) -> Result<Mods, QueryError> {
    let url = file
        .download_url
        .clone()
        .ok_or_else(|| QueryError::ModDistributionForbidden {
            id: file.mod_id.to_string(),
        })?;
    let sha1 = file
        .hashes
        .iter()
        .find(|h| h.algo == HASH_ALGO_SHA1)
        .map(|h| h.value.clone());
    Ok(Mods {
        name: format!("{}-{}", file.mod_id, file.id),
        url: Some(url),
        path: Some(file.file_name.clone()),
        sha1,
        size: (file.file_length > 0).then_some(file.file_length),
    })
}

/// Picks every `required` dependency (relation type 3), drops the rest.
fn collect_required_dependencies(file: &CurseForgeFile) -> Vec<ModRequest> {
    file.dependencies
        .iter()
        .filter(|dep| dep.relation_type == DEP_REQUIRED)
        .map(modrequest_from)
        .collect()
}

fn modrequest_from(dep: &CurseForgeDependency) -> ModRequest {
    ModRequest::CurseForge {
        mod_id: dep.mod_id,
        file_id: None,
    }
}

/// Minimal URL-encoder for the characters CurseForge's query string
/// might carry (`gameVersion` has dots, sometimes hyphens — those are
/// RFC3986-unreserved so safe, but build defensively).
fn url_encode(input: &str) -> String {
    let mut out = String::with_capacity(input.len() + 8);
    for byte in input.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char);
            }
            _ => out.push_str(&format!("%{:02X}", byte)),
        }
    }
    out
}
