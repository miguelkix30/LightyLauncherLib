// Copyright (c) 2025 Hamadi
// Licensed under the MIT License

//! Source-agnostic BFS resolver.
//!
//! Walks the user-provided `ModRequest`s, fetches each one through its
//! corresponding client, follows `required` dependencies recursively,
//! and produces the pivot `Vec<Mods>` the launch crate feeds into the
//! standard mods installer.

use std::collections::{HashSet, VecDeque};

use crate::types::version_metadata::Mods;
use crate::types::Loader;
use crate::utils::error::QueryError;

use super::request::{ModKey, ModRequest};

/// Optional progress callback fired during the BFS.
///
/// `(source, identifier)` is reported for each request the resolver
/// pulls; `(parent, dependency)` for each transitively-enqueued dep.
/// The launch crate uses this to emit `ModResolve*` events.
pub struct ResolveCallbacks<'a> {
    pub on_fetch: &'a (dyn Fn(&'static str, &str) + Send + Sync),
    pub on_dependency: &'a (dyn Fn(&str, &str) + Send + Sync),
}

/// Resolves every user request + its transitive `required` dependencies
/// into a flat list of pivot [`Mods`] entries, deduplicated by
/// `(source, project-id)`.
pub async fn resolve(
    requests: &[ModRequest],
    mc: &str,
    loader: &Loader,
    callbacks: Option<&ResolveCallbacks<'_>>,
) -> Result<Vec<Mods>, QueryError> {
    let mut queue: VecDeque<ModRequest> = requests.iter().cloned().collect();
    let mut visited: HashSet<ModKey> = HashSet::new();
    let mut out: Vec<Mods> = Vec::new();

    while let Some(req) = queue.pop_front() {
        let key = ModKey::from(&req);
        if !visited.insert(key.clone()) {
            continue;
        }

        if let Some(cb) = callbacks {
            (cb.on_fetch)(key.source.as_str(), &key.id);
        }

        let (pivot, deps) = fetch_one(&req, mc, loader).await?;
        let parent_name = pivot.name.clone();
        out.push(pivot);

        for dep in deps {
            let dep_key = ModKey::from(&dep);
            if visited.contains(&dep_key) {
                continue;
            }
            if let Some(cb) = callbacks {
                (cb.on_dependency)(&parent_name, &dep_key.id);
            }
            queue.push_back(dep);
        }
    }

    Ok(out)
}

/// Single-request dispatch — picks the right client.
///
/// Cfg-gated so disabled sources still parse (so users can build with
/// only `modrinth` or only `curseforge`) — but a request for a disabled
/// source is rejected at runtime rather than silently dropped.
async fn fetch_one(
    req: &ModRequest,
    mc: &str,
    loader: &Loader,
) -> Result<(Mods, Vec<ModRequest>), QueryError> {
    match req {
        #[cfg(feature = "modrinth")]
        ModRequest::Modrinth { .. } => super::modrinth::fetch(req, mc, loader).await,

        #[cfg(not(feature = "modrinth"))]
        ModRequest::Modrinth { id_or_slug, .. } => Err(QueryError::UnsupportedLoader(format!(
            "Modrinth support is disabled (cargo feature 'modrinth' not enabled) — \
             cannot fetch '{}'",
            id_or_slug
        ))),

        #[cfg(feature = "curseforge")]
        ModRequest::CurseForge { .. } => super::curseforge::fetch(req, mc, loader).await,

        #[cfg(not(feature = "curseforge"))]
        ModRequest::CurseForge { mod_id, .. } => Err(QueryError::UnsupportedLoader(format!(
            "CurseForge support is disabled (cargo feature 'curseforge' not enabled) — \
             cannot fetch mod #{}",
            mod_id
        ))),
    }
}
