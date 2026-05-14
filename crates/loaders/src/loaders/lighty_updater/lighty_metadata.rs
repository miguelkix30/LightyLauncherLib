//! Serde types describing a LightyUpdater server response.
//!
//! [`ServersResponse`] is the top-level listing the server returns;
//! [`ServerInfo`] is one entry inside it; [`LightyMetadata`] is the
//! optional override document the server publishes for each instance.

use serde::{Deserialize, Serialize};

/// Server response listing every instance the LightyUpdater publishes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServersResponse {
    servers: Vec<ServerInfo>,
}

impl ServersResponse {
    /// Finds the server entry matching `name`, if any.
    pub fn find_by_name(&self, name: &str) -> Option<&ServerInfo> {
        self.servers.iter().find(|s| s.name == name)
    }
}

/// Per-server info entry returned by the LightyUpdater listing endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    name: String,
    loader: String,
    loader_version: String,
    minecraft_version: String,
    url: String,
    last_update: String,
}

impl ServerInfo {
    /// Retourne le nom du serveur
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Retourne le type de loader (vanilla, fabric, quilt, etc.)
    pub fn loader(&self) -> &str {
        &self.loader
    }

    /// Retourne la version du loader
    pub fn loader_version(&self) -> &str {
        &self.loader_version
    }

    /// Retourne la version Minecraft
    pub fn minecraft_version(&self) -> &str {
        &self.minecraft_version
    }

    /// Retourne l'URL du serveur
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Returns the timestamp of the last server update.
    pub fn last_update(&self) -> &str {
        &self.last_update
    }
}

//STRUCTURE OF LIGHTY_UPDATER METADATA
/// Metadata document returned by a LightyUpdater server.
///
/// Every field is optional: the LightyUpdater server may supply only the
/// overrides it cares about, and the base loader (vanilla / fabric / quilt
/// / ...) fills in everything else.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct LightyMetadata {
    #[serde(skip)]  // server_info comes from a separate request, never serialized into LightyMetadata
    pub server_info: Option<ServerInfo>,
    pub main_class: Option<MainClass>,
    pub java_version: Option<JavaVersion>,
    pub arguments: Option<Arguments>,
    pub libraries: Option<Vec<Library>>,
    pub natives: Option<Vec<Native>>,
    pub client: Option<Client>,
    pub assets: Option<Vec<Asset>>,
    pub mods: Option<Vec<Mod>>,
}

impl Default for LightyMetadata {
    fn default() -> Self {
        Self {
            server_info: None,
            main_class: None,
            java_version: None,
            arguments: None,
            libraries: None,
            natives: None,
            client: None,
            assets: None,
            mods: None,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MainClass {
    pub main_class: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JavaVersion {
    pub major_version: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Arguments {
    pub game: Vec<String>,
    pub jvm: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Library {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mod {
    pub name: String,
    pub url: String,
    pub path: String,
    pub sha1: String,
    pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Native {
    pub name: String,
    pub url: String,
    pub path: String,
    pub sha1: String,
    pub size: u64,
    pub os: String,  // "windows", "linux", or "macos"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Client {
    pub name: String,
    pub url: String,
    pub path: String,
    pub sha1: String,
    pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub hash: String,
    pub size: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
