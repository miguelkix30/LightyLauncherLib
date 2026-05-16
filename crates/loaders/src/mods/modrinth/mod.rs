// Copyright (c) 2025 Hamadi
// Licensed under the MIT License

//! Modrinth Labrinth-API client.
//!
//! - [`modrinth`] — fetch + pivot conversion.
//! - [`modrinth_metadata`] — serde mirrors of the JSON wire format.
//!
//! The public API ([`fetch`]) is re-exported here so callers write
//! `mods::modrinth::fetch(...)` instead of `mods::modrinth::modrinth::fetch(...)`.

pub mod modrinth;
pub mod modrinth_metadata;

pub use modrinth::fetch;
