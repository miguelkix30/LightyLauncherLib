use lighty_loaders::types::{Loader, VersionInfo};
use std::path::{Path, PathBuf};
use once_cell::sync::Lazy;
use directories::ProjectDirs;

/// Builder pour LightyUpdater avec une sémantique claire
#[derive(Debug, Clone)]
pub struct LightyVersionBuilder<'a> {
    pub name: String,
    pub server_url: String,
    pub minecraft_version: Option<String>,
    pub loader: Option<Loader>,
    pub project_dirs: &'a Lazy<ProjectDirs>,
    pub game_dirs: PathBuf,
    pub java_dirs: PathBuf,
}

impl<'a> LightyVersionBuilder<'a> {
    pub fn new(
        name: &str,
        server_url: &str,
        project_dirs: &'a Lazy<ProjectDirs>,
    ) -> Self {
        Self {
            name: name.to_string(),
            server_url: server_url.to_string(),
            minecraft_version: None,
            loader: None,
            project_dirs,
            game_dirs: project_dirs.data_dir().join(name),
            java_dirs: project_dirs.config_dir().to_path_buf().join("jre"),
        }
    }
}

impl<'a> VersionInfo for LightyVersionBuilder<'a> {
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
impl<'a, 'b> VersionInfo for &'b LightyVersionBuilder<'a> {
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
