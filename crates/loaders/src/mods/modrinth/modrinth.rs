// Copyright (c) 2025 Hamadi
// Licensed under the MIT License

//! Modrinth Labrinth-API client.
//!
//! Public REST endpoint at `https://api.modrinth.com/v2`. No key
//! required; Modrinth asks every consumer to send a custom `User-Agent`
//! so requests can be identified and not aggressively rate-limited.
//!
//! Wire-format structs live in [`super::modrinth_metadata`].

use lighty_core::hosts::HTTP_CLIENT as CLIENT;

use crate::types::version_metadata::Mods;
use crate::types::Loader;
use crate::utils::error::QueryError;

use super::modrinth_metadata::{ModrinthDependency, ModrinthFile, ModrinthVersion};
use crate::mods::request::ModRequest;

const BASE_URL: &str = "https://api.modrinth.com/v2";
const USER_AGENT: &str = concat!(
    "Lighty-Launcher/",
    env!("CARGO_PKG_VERSION"),
    " (https://github.com/Lighty-Launcher/LightyLauncherLib)"
);

const PROVIDER: &str = "modrinth";

/// Resolves a single `ModRequest::Modrinth { … }` into a pivot
/// [`Mods`] entry + the list of `required` dependencies to enqueue.
///
/// Hits at most one HTTP endpoint:
/// - `GET /version/{id}` when a specific version was pinned;
/// - `GET /project/{slug}/version?loaders=…&game_versions=…` otherwise
///   (Modrinth filters server-side and returns the list date-desc;
///   we take the first).
pub async fn fetch(
    request: &ModRequest,
    minecraft_version: &str,
    loader: &Loader,
) -> Result<(Mods, Vec<ModRequest>), QueryError> {
    let (id_or_slug, pinned_version) = match request {
        ModRequest::Modrinth { id_or_slug, version } => (id_or_slug.clone(), version.clone()),
        _ => unreachable!("modrinth::fetch called with a non-Modrinth request"),
    };

    let version = if let Some(version_id) = pinned_version {
        fetch_pinned_version(&version_id).await?
    } else {
        let loader_tag = loader_tag(loader)?;
        fetch_latest_compatible(&id_or_slug, minecraft_version, loader_tag).await?
    };

    let pivot = into_pivot(&id_or_slug, &version)?;
    let dependencies = collect_required_dependencies(&version);

    Ok((pivot, dependencies))
}

/// Maps a [`Loader`] to its Modrinth loader-tag string.
///
/// Returns an error for loaders Modrinth doesn't host
/// (Vanilla, OptiFine, LightyUpdater).
fn loader_tag(loader: &Loader) -> Result<&'static str, QueryError> {
    match loader {
        Loader::Fabric => Ok("fabric"),
        Loader::Forge => Ok("forge"),
        Loader::NeoForge => Ok("neoforge"),
        Loader::Quilt => Ok("quilt"),
        Loader::Vanilla | Loader::Optifine | Loader::LightyUpdater => {
            Err(QueryError::UnsupportedLoader(format!(
                "Modrinth doesn't host mods for {:?} instances",
                loader
            )))
        }
    }
}

async fn fetch_pinned_version(version_id: &str) -> Result<ModrinthVersion, QueryError> {
    let url = format!("{}/version/{}", BASE_URL, version_id);
    let response = CLIENT.get(&url).header("User-Agent", USER_AGENT).send().await?;
    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Err(QueryError::ModNotFound {
            provider: PROVIDER,
            id: version_id.to_string(),
        });
    }
    Ok(response.error_for_status()?.json::<ModrinthVersion>().await?)
}

async fn fetch_latest_compatible(
    slug: &str,
    minecraft_version: &str,
    loader_tag: &str,
) -> Result<ModrinthVersion, QueryError> {
    // Query-string arrays use JSON encoding per Modrinth's docs.
    // Built by hand rather than via reqwest's `.query()` to avoid
    // pulling in `serde_urlencoded` — both values are simple enough.
    let loaders_param = url_encode(&format!("[\"{}\"]", loader_tag));
    let game_versions_param = url_encode(&format!("[\"{}\"]", minecraft_version));
    let url = format!(
        "{}/project/{}/version?loaders={}&game_versions={}",
        BASE_URL, slug, loaders_param, game_versions_param
    );

    let response = CLIENT.get(&url).header("User-Agent", USER_AGENT).send().await?;
    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Err(QueryError::ModNotFound {
            provider: PROVIDER,
            id: slug.to_string(),
        });
    }
    let versions = response.error_for_status()?.json::<Vec<ModrinthVersion>>().await?;
    versions
        .into_iter()
        .next()
        .ok_or_else(|| QueryError::ModIncompatible {
            provider: PROVIDER,
            id: slug.to_string(),
            mc: minecraft_version.to_string(),
            loader: loader_tag.to_string(),
        })
}

/// Picks the binary file from a version and converts it into the
/// launcher's pivot [`Mods`] type.
///
/// `path` is the bare filename — the mods installer prefixes it with
/// `{runtime_dir}/mods/`.
fn into_pivot(slug: &str, version: &ModrinthVersion) -> Result<Mods, QueryError> {
    let file = primary_file(&version.files).ok_or_else(|| QueryError::MissingField {
        field: format!("file in modrinth version {}", version.id),
    })?;
    Ok(Mods {
        name: format!("{}-{}", slug, version.version_number),
        url: Some(file.url.clone()),
        path: Some(file.filename.clone()),
        sha1: Some(file.hashes.sha1.clone()),
        size: Some(file.size),
    })
}

/// Selects the build artifact among a version's `files` array.
///
/// Most Modrinth versions ship one primary file plus optional source /
/// dev jars — `primary == true` is the one to install.
fn primary_file(files: &[ModrinthFile]) -> Option<&ModrinthFile> {
    files.iter().find(|f| f.primary).or_else(|| files.first())
}

/// Picks every `required` dependency of `version`, dropping the rest.
///
/// Deps that only carry a `version_id` (no `project_id`) need an extra
/// lookup to resolve their project — we skip those with a debug log
/// rather than spend a request on a corner case.
fn collect_required_dependencies(version: &ModrinthVersion) -> Vec<ModRequest> {
    version
        .dependencies
        .iter()
        .filter(|dep| dep.dependency_type == "required")
        .filter_map(|dep| modrequest_from(dep, &version.id))
        .collect()
}

fn modrequest_from(dep: &ModrinthDependency, parent_version: &str) -> Option<ModRequest> {
    let project_id = dep.project_id.clone().or_else(|| {
        lighty_core::trace_debug!(
            parent_version = %parent_version,
            "Skipping Modrinth dep with no project_id"
        );
        None
    })?;
    Some(ModRequest::Modrinth {
        id_or_slug: project_id,
        version: dep.version_id.clone(),
    })
}

/// Minimal URL-encoder for the few characters Modrinth's query
/// arrays use (`[`, `]`, `"`, `,`). Avoids pulling a urlencoding dep.
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
