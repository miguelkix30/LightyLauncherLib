//! Placeholder serde types for the future Forge loader implementation.
//!
//! The [`super::forge`] module is currently disabled (see `mod.rs`); these
//! types stay in the tree as the skeleton for upcoming work.

#![allow(dead_code)]

use serde::Deserialize;

/// Forge `install_profile.json` (skeleton).
#[derive(Debug, Deserialize, Clone)]
pub struct ForgeMetaData {

}

/// Forge `version.json` (skeleton).
#[derive(Debug, Deserialize, Clone)]
pub struct ForgeVersionMeta {

}
