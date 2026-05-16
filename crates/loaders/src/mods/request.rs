// Copyright (c) 2025 Hamadi
// Licensed under the MIT License

//! User-facing types for the mod resolver.
//!
//! [`ModRequest`] is what the [`crate::types::Loader`]-aware builder
//! accumulates. [`ModSource`] / [`ModKey`] are internal helpers used by
//! the resolver to dedup across transitively-pulled dependencies.

/// A mod the user asked the launcher to install.
///
/// Each variant carries the source-specific identifiers needed to query
/// the corresponding API. `version` / `file_id` are optional pins — if
/// absent, the resolver picks the latest release compatible with the
/// instance's MC version + loader.
#[derive(Debug, Clone)]
pub enum ModRequest {
    /// Modrinth project (public API, no key required).
    Modrinth {
        /// Project slug (`"sodium"`) or project id (`"AANobbMI"`).
        id_or_slug: String,
        /// Specific mod-version id (`"PpRTuoEh"`). `None` ⇒ latest.
        version: Option<String>,
    },
    /// CurseForge project (requires `CURSEFORGE_API_KEY`).
    CurseForge {
        /// Numeric mod id (e.g. `238222` for JEI).
        mod_id: u32,
        /// Specific file id. `None` ⇒ latest compatible.
        file_id: Option<u32>,
    },
}

/// Source tag — used for dedup keys and error messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModSource {
    Modrinth,
    CurseForge,
}

impl ModSource {
    pub fn as_str(self) -> &'static str {
        match self {
            ModSource::Modrinth => "modrinth",
            ModSource::CurseForge => "curseforge",
        }
    }
}

/// Dedup key for the BFS resolver: `(source, project-level identifier)`.
///
/// Version-agnostic — two different versions of the same mod count as
/// the same key (first one wins, BFS order, user requests have priority
/// over transitively-pulled dependencies).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModKey {
    pub source: ModSource,
    pub id: String,
}

impl ModKey {
    pub fn modrinth(id: impl Into<String>) -> Self {
        Self { source: ModSource::Modrinth, id: id.into() }
    }
    pub fn curseforge(mod_id: u32) -> Self {
        Self { source: ModSource::CurseForge, id: mod_id.to_string() }
    }
}

impl From<&ModRequest> for ModKey {
    fn from(req: &ModRequest) -> Self {
        match req {
            ModRequest::Modrinth { id_or_slug, .. } => ModKey::modrinth(id_or_slug.clone()),
            ModRequest::CurseForge { mod_id, .. } => ModKey::curseforge(*mod_id),
        }
    }
}
