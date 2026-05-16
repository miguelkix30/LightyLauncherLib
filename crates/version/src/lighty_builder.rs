use std::path::{Path, PathBuf};

use lighty_core::AppState;
use lighty_loaders::types::{Loader, VersionInfo};

/// Builder for LightyUpdater-managed instances.
///
/// Unlike [`super::VersionBuilder`], `loader_version` here holds the
/// LightyUpdater server URL: the actual loader and Minecraft version
/// are fetched from that server at install time. Default paths come
/// from the global [`AppState`] (call [`AppState::init`] first).
#[derive(Debug, Clone)]
pub struct LightyVersionBuilder {
    pub name: String,
    pub server_url: String,
    pub minecraft_version: Option<String>,
    pub loader: Option<Loader>,
    pub game_dirs: PathBuf,
    pub java_dirs: PathBuf,
}

impl LightyVersionBuilder {
    /// Creates a new `LightyVersionBuilder`.
    ///
    /// `server_url` is the LightyUpdater server endpoint; the loader
    /// and Minecraft version are resolved from its response at install
    /// time. Panics if [`AppState::init`] hasn't been called.
    pub fn new(name: &str, server_url: &str) -> Self {
        Self {
            name: name.to_string(),
            server_url: server_url.to_string(),
            minecraft_version: None,
            loader: None,
            game_dirs: AppState::data_dir().join(name),
            java_dirs: AppState::config_dir().join("jre"),
        }
    }
}

impl VersionInfo for LightyVersionBuilder {
    type LoaderType = Loader;

    fn name(&self) -> &str {
        &self.name
    }

    fn loader_version(&self) -> &str {
        &self.server_url
    }

    fn minecraft_version(&self) -> &str {
        self.minecraft_version.as_ref().map_or("", String::as_str)
    }

    fn game_dirs(&self) -> &Path {
        &self.game_dirs
    }

    fn java_dirs(&self) -> &Path {
        &self.java_dirs
    }

    fn loader(&self) -> &Self::LoaderType {
        self.loader.as_ref().unwrap_or(&Loader::LightyUpdater)
    }
}

// Impl for references to allow passing &LightyVersionBuilder
impl<'b> VersionInfo for &'b LightyVersionBuilder {
    type LoaderType = Loader;

    fn name(&self) -> &str {
        &self.name
    }

    fn loader_version(&self) -> &str {
        &self.server_url
    }

    fn minecraft_version(&self) -> &str {
        self.minecraft_version.as_ref().map_or("", String::as_str)
    }

    fn game_dirs(&self) -> &Path {
        &self.game_dirs
    }

    fn java_dirs(&self) -> &Path {
        &self.java_dirs
    }

    fn loader(&self) -> &Self::LoaderType {
        self.loader.as_ref().unwrap_or(&Loader::LightyUpdater)
    }
}
