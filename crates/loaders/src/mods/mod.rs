// Copyright (c) 2025 Hamadi
// Licensed under the MIT License

//! Mod-source clients (Modrinth, CurseForge) and a generic dependency
//! resolver that converts user mod requests into the launcher's pivot
//! [`crate::types::version_metadata::Mods`] entries.
//!
//! Each provider lives in its own subdirectory ([`modrinth`],
//! [`curseforge`]) with a fetch module + a metadata module — same
//! split as `loaders/forge/forge.rs` + `forge_legacy_metadata.rs`
//! elsewhere in the project.
//!
//! [`request`] holds the user-facing request enum and the dedup key
//! the resolver uses; [`resolver`] is the source-agnostic BFS that
//! walks declared `required` dependencies transitively.

pub mod request;

#[cfg(any(feature = "modrinth", feature = "curseforge"))]
pub mod resolver;

#[cfg(feature = "modrinth")]
pub mod modrinth;

#[cfg(feature = "curseforge")]
pub mod curseforge;

pub use request::{ModKey, ModRequest, ModSource};
