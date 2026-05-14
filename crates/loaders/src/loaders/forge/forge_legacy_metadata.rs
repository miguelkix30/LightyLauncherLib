// Copyright (c) 2025 Hamadi
// Licensed under the MIT License

//! Serde mirrors of the `install_profile.json` shipped inside legacy
//! Forge installer JARs (Minecraft 1.4.x → 1.12.2).
//!
//! Legacy Forge bundles everything into a single `install_profile.json`
//! with two top-level blocks:
//! - `install` — installer metadata: which universal JAR ships inside
//!   the installer ZIP and where to place it on disk.
//! - `versionInfo` — the runtime profile: `mainClass`,
//!   `minecraftArguments` (single legacy-style string), library list,
//!   asset index id.
//!
//! Unlike modern Forge (1.13+) there is no separate `version.json`,
//! no `processors`, and no `data` substitution table.

use std::collections::HashMap;

use serde::Deserialize;

use crate::loaders::vanilla::vanilla_metadata::Rule;

/// Top-level `install_profile.json` for legacy Forge installers.
#[derive(Debug, Deserialize, Clone)]
pub struct ForgeLegacyInstallProfile {
    pub install: ForgeLegacyInstallBlock,
    #[serde(rename = "versionInfo")]
    pub version_info: ForgeLegacyVersionInfo,
}

/// The `install` block — describes the universal JAR shipped inside
/// the installer ZIP and where it should land in the libraries layout.
#[derive(Debug, Deserialize, Clone)]
pub struct ForgeLegacyInstallBlock {
    /// Profile name (typically `"forge"`).
    #[serde(rename = "profileName")]
    pub profile_name: String,

    /// Profile target id (e.g. `"1.7.10-Forge10.13.4.1614-1.7.10"`).
    pub target: String,

    /// Maven coordinates of the universal JAR
    /// (e.g. `"net.minecraftforge:forge:1.7.10-10.13.4.1614-1.7.10"`).
    pub path: String,

    /// Forge build version (e.g. `"Forge10.13.4.1614"`).
    pub version: String,

    /// Filename of the universal JAR inside the installer ZIP
    /// (e.g. `"forge-1.7.10-10.13.4.1614-1.7.10-universal.jar"`).
    #[serde(rename = "filePath")]
    pub file_path: String,

    /// Minecraft version targeted (e.g. `"1.7.10"`).
    pub minecraft: String,
}

/// The `versionInfo` block — Mojang-style profile description.
#[derive(Debug, Deserialize, Clone)]
pub struct ForgeLegacyVersionInfo {
    /// Profile id (matches `install.target`).
    pub id: String,

    /// Vanilla version this profile inherits from
    /// (matches `install.minecraft`). Optional — some very old profiles
    /// omit it and rely on the launcher to resolve from `install.minecraft`.
    #[serde(rename = "inheritsFrom", default)]
    pub inherits_from: Option<String>,

    /// Asset index id (typically matches the MC version, e.g. `"1.7.10"`).
    #[serde(default)]
    pub assets: Option<String>,

    /// Main class — usually `net.minecraft.launchwrapper.Launch`.
    #[serde(rename = "mainClass")]
    pub main_class: String,

    /// Legacy single-string command line with `${...}` placeholders.
    #[serde(rename = "minecraftArguments")]
    pub minecraft_arguments: String,

    /// Required libraries (resolved via Maven coordinates).
    pub libraries: Vec<ForgeLegacyLibrary>,
}

/// One entry in `versionInfo.libraries`.
///
/// Unlike modern Forge there is no `downloads.artifact` block; only the
/// Maven coordinate (`name`) and an optional `url` base. The Maven path
/// has to be reconstructed from the coordinate.
#[derive(Debug, Deserialize, Clone)]
pub struct ForgeLegacyLibrary {
    /// Maven coordinate `group:artifact:version`.
    pub name: String,

    /// Base Maven URL. When absent, the launcher falls back to the
    /// Forge Maven for Forge-owned artifacts and to Mojang's libraries
    /// Maven for vanilla shared artifacts.
    #[serde(default)]
    pub url: Option<String>,

    /// Whether this library is required on the client side.
    /// Absent ⇒ defaults to `true`.
    #[serde(default = "default_true")]
    pub clientreq: bool,

    /// Whether this library is required on the server side.
    /// Absent ⇒ defaults to `true`. Unused by the launcher.
    #[serde(default = "default_true")]
    #[allow(dead_code)]
    pub serverreq: bool,

    /// OS rules — same shape as Mojang's vanilla library rules.
    /// When present, the launcher evaluates them and skips the entry
    /// when the current OS isn't allowed.
    #[serde(default)]
    pub rules: Option<Vec<Rule>>,

    /// Natives classifier map (`os → classifier-name`). When present,
    /// the entry has no bare JAR — only the natives-classifier JARs
    /// exist on Maven and they're picked up by the natives extractor,
    /// not the library installer.
    #[serde(default)]
    pub natives: Option<HashMap<String, String>>,
}

fn default_true() -> bool {
    true
}
