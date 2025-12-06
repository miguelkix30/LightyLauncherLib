
use lighty_loaders::types::VersionInfo;
use std::path::{Path, PathBuf};
use once_cell::sync::Lazy;
use directories::ProjectDirs;

/// Builder pour LightyUpdater avec une sémantique claire
#[derive(Debug, Clone)]
pub struct LightyVersionBuilder<'a> {
    pub name: String,
    pub server_url: String,
    pub minecraft_version: String,
    pub project_dirs: &'a Lazy<ProjectDirs>,
    pub game_dirs: PathBuf,
    pub java_dirs: PathBuf,
}

impl<'a> LightyVersionBuilder<'a> {
    pub fn new(
        name: &str,
        server_url: &str,
        minecraft_version: &str,
        project_dirs: &'a Lazy<ProjectDirs>,
    ) -> Self {
        Self {
            name: name.to_string(),
            server_url: server_url.to_string(),
            minecraft_version: minecraft_version.to_string(),
            project_dirs,
            game_dirs: project_dirs.data_dir().join(name),
            java_dirs: project_dirs.config_dir().to_path_buf().join("jre"),
        }
    }
}


impl<'a> VersionInfo for LightyVersionBuilder<'a> {
    // LightyVersionBuilder n'a pas de loader traditionnel, utilise le type unit
    type LoaderType = String;

    fn name(&self) -> &str {
        &self.name
    }

    fn loader_version(&self) -> &str {
        &self.server_url
    }

    fn minecraft_version(&self) -> &str {
        &self.minecraft_version
    }

    fn game_dirs(&self) -> &Path {
        &self.game_dirs
    }

    fn java_dirs(&self) -> &Path {
        &self.java_dirs
    }

    fn loader(&self) -> &Self::LoaderType {
        // LightyUpdater utilise l'URL du serveur comme "loader"
        &self.server_url
    }
}

// Impl for references to allow passing &LightyVersionBuilder
impl<'a, 'b> VersionInfo for &'b LightyVersionBuilder<'a> {
    type LoaderType = String;

    fn name(&self) -> &str {
        &self.name
    }

    fn loader_version(&self) -> &str {
        &self.server_url
    }

    fn minecraft_version(&self) -> &str {
        &self.minecraft_version
    }

    fn game_dirs(&self) -> &Path {
        &self.game_dirs
    }

    fn java_dirs(&self) -> &Path {
        &self.java_dirs
    }

    fn loader(&self) -> &Self::LoaderType {
        &self.server_url
    }
}
