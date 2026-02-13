use serde::Deserialize;
use std::collections::HashMap;

/// Metadata from install_profile.json inside the Forge installer JAR
#[derive(Debug, Deserialize, Clone)]
pub struct ForgeInstallProfile {
    pub spec: Option<i32>,
    pub profile: Option<String>,
    pub version: String,
    pub path: Option<String>,
    pub minecraft: String,
    pub data: Option<HashMap<String, ForgeDataEntry>>,
    pub processors: Option<Vec<ForgeProcessor>>,
    pub libraries: Vec<ForgeLibraryEntry>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ForgeDataEntry {
    pub client: String,
    pub server: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ForgeProcessor {
    pub sides: Option<Vec<String>>,
    pub jar: String,
    pub classpath: Vec<String>,
    pub args: Vec<String>,
    pub outputs: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ForgeLibraryEntry {
    pub name: String,
    pub downloads: ForgeDownloads,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ForgeDownloads {
    pub artifact: ForgeArtifact,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ForgeArtifact {
    pub path: String,
    pub url: String,
    pub sha1: String,
    pub size: u64,
}

/// Metadata from version.json inside the Forge installer JAR
/// This is the actual version manifest that Minecraft launcher uses
#[derive(Debug, Deserialize, Clone)]
pub struct ForgeVersionManifest {
    pub id: String,
    #[serde(rename = "type")]
    pub version_type: String,
    pub time: String,
    #[serde(rename = "releaseTime")]
    pub release_time: String,
    #[serde(rename = "mainClass")]
    pub main_class: String,
    #[serde(rename = "inheritsFrom")]
    pub inherits_from: Option<String>,
    pub arguments: ForgeArguments,
    pub libraries: Vec<ForgeVersionLibrary>,
    #[serde(rename = "minecraftArguments")]
    pub minecraft_arguments: Option<String>, // Legacy format (pre-1.13)
}

#[derive(Debug, Deserialize, Clone)]
pub struct ForgeArguments {
    #[serde(default)]
    pub game: Vec<ForgeArgument>,
    #[serde(default)]
    pub jvm: Vec<ForgeArgument>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum ForgeArgument {
    Simple(String),
    Conditional(ForgeConditionalArgument),
}

#[derive(Debug, Deserialize, Clone)]
pub struct ForgeConditionalArgument {
    pub rules: Vec<ForgeRule>,
    pub value: ForgeArgumentValue,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum ForgeArgumentValue {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, Deserialize, Clone)]
pub struct ForgeRule {
    pub action: String,
    pub os: Option<ForgeOsRule>,
    pub features: Option<HashMap<String, bool>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ForgeOsRule {
    pub name: Option<String>,
    pub version: Option<String>,
    pub arch: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ForgeVersionLibrary {
    pub name: String,
    pub downloads: Option<ForgeLibraryDownloads>,
    pub url: Option<String>,
    pub rules: Option<Vec<ForgeRule>>,
    pub natives: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ForgeLibraryDownloads {
    pub artifact: Option<ForgeArtifact>,
    pub classifiers: Option<HashMap<String, ForgeArtifact>>,
}