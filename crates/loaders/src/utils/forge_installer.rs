// Copyright (c) 2025 Hamadi
// Licensed under the MIT License

//! Serde mirrors of the `install_profile.json` and `version.json` files
//! embedded in every modern Forge-family installer JAR.
//!
//! "Modern" here means 1.13+ where the installer ships an `install_profile`
//! that drives processors plus a `version.json` that describes the runtime
//! layout. NeoForge (forked from Forge in 2023) reuses the exact same
//! format. Legacy Forge (1.12.2 and below) uses a completely different
//! shape and lives elsewhere.
//!
//! [`ForgeInstallProfile`] models `install_profile.json`;
//! [`ForgeVersionManifest`] models `version.json`.

use serde::Deserialize;
use std::collections::HashMap;

/// `install_profile.json` — declares processors and the libraries they
/// need (including the runtime-required `forge:universal` artifact).
///
/// Several fields are optional because the install_profile format has
/// evolved across Forge eras: 1.14.4 (`spec: 0`) omits `mirrorList` and
/// `serverJarPath`; 1.20.1+ (`spec: 1`) includes them.
#[derive(Debug, Deserialize, Clone)]
pub struct ForgeInstallProfile {
    pub spec: i32,
    pub profile: String,
    pub version: String,
    pub icon: String,
    pub minecraft: String,
    pub json: String,
    pub logo: String,
    pub welcome: String,
    #[serde(rename = "mirrorList", default)]
    pub mirror_list: String,
    #[serde(rename = "hideExtract", default)]
    pub hide_extract: bool,
    pub data: HashMap<String, DataEntry>,
    pub processors: Vec<Processor>,
    pub libraries: Vec<ForgeLibrary>,
    #[serde(rename = "serverJarPath", default)]
    pub server_jar_path: String,
}

/// One `data` entry — paired client/server substitution values.
#[derive(Debug, Deserialize, Clone)]
pub struct DataEntry {
    pub client: String,
    pub server: String,
}

/// A single install processor (Java program to run during install).
#[derive(Debug, Deserialize, Clone)]
pub struct Processor {
    #[serde(default)]
    pub sides: Vec<String>,
    pub jar: String,
    pub classpath: Vec<String>,
    pub args: Vec<String>,
    #[serde(default)]
    pub outputs: HashMap<String, String>,
}

/// One Maven library entry (used by both `install_profile.json` and
/// `version.json` library lists).
#[derive(Debug, Deserialize, Clone)]
pub struct ForgeLibrary {
    pub name: String,
    pub downloads: LibraryDownloads,
}

/// Download metadata for a [`ForgeLibrary`].
#[derive(Debug, Deserialize, Clone)]
pub struct LibraryDownloads {
    pub artifact: Artifact,
}

/// Resolved Maven artifact: URL, path, hash, size.
#[derive(Debug, Deserialize, Clone)]
pub struct Artifact {
    pub sha1: String,
    pub size: u64,
    pub url: String,
    pub path: String,
}

// ============= VERSION.JSON =============

/// `version.json` — runtime classpath/main-class/arguments layout.
///
/// `arguments` and `minecraft_arguments` are both optional because the
/// version.json schema changed across eras:
/// - 1.13+ ships a structured `arguments` object.
/// - Back-ported modern installers for 1.12.2 (e.g. Forge 14.23.5.2860)
///   keep the legacy single-string `minecraftArguments` even though the
///   surrounding install_profile is the modern schema.
#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct ForgeVersionManifest {
    pub id: String,
    pub time: String,
    #[serde(rename = "releaseTime")]
    pub release_time: String,
    #[serde(rename = "type")]
    pub release_type: String,
    #[serde(rename = "mainClass")]
    pub main_class: String,
    #[serde(rename = "inheritsFrom")]
    pub inherits_from: String,
    #[serde(default)]
    pub arguments: Option<ForgeArguments>,
    #[serde(rename = "minecraftArguments", default)]
    pub minecraft_arguments: Option<String>,
    pub libraries: Vec<ForgeLibrary>,
}

/// Game/JVM argument lists from `version.json`.
///
/// `jvm` is optional because older Forge installers (e.g. 1.14.4) only
/// declare a `game` list — the JVM arguments are inherited from the
/// vanilla manifest in that era.
#[derive(Debug, Deserialize, Clone)]
pub struct ForgeArguments {
    pub game: Vec<String>,
    #[serde(default)]
    pub jvm: Vec<String>,
}
