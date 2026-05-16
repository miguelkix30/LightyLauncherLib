// Copyright (c) 2025 Hamadi
// Licensed under the MIT License

//! CurseForge Core-API client.
//!
//! - [`curseforge`] — fetch + pivot conversion + `set_api_key` setter.
//! - [`curseforge_metadata`] — serde mirrors of the JSON wire format
//!   and the numeric loader / hash / dependency constants.
//!
//! Public API ([`fetch`], [`set_api_key`]) is re-exported here so
//! callers write `mods::curseforge::fetch(...)` instead of
//! `mods::curseforge::curseforge::fetch(...)`.

pub mod curseforge;
pub mod curseforge_metadata;

pub use curseforge::{fetch, set_api_key};
