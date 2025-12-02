use crate::loaders::lighty_updater::lighty_updater::{LIGHTY_UPDATER, LightyQuery};
use crate::loaders::neoforge::neoforge::{NeoForgeQuery, NEOFORGE};
use crate::loaders::quilt::quilt::{QuiltQuery, QUILT};
use crate::loaders::fabric::fabric::{FabricQuery, FABRIC};
use crate::loaders::vanilla::vanilla::{VanillaQuery, VANILLA};
use lighty_version::version_metadata::VersionMetaData;
use crate::utils::error::QueryError;
use std::sync::Arc;
use std::result::Result as StdResult;
use directories::ProjectDirs;
use once_cell::sync::Lazy;

pub type Result<T> = StdResult<T, QueryError>;

#[derive(Debug, Clone)]
pub enum Loader {
    Fabric,
    NeoForge,
    Optifine,
    Quilt,
    Vanilla,
    Forge,
    LightyUpdater,
}

/// Structure principale d'une version Minecraft
use std::path::PathBuf;

pub struct Version<'a> {
    pub name: String,
    pub loader: Loader,
    pub loader_version: String,
    pub minecraft_version: String,
    pub project_dirs: &'a Lazy<ProjectDirs>,
    pub game_dirs: PathBuf,
    pub java_dirs: PathBuf,
}

impl<'a> Clone for Version<'a> {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            loader: self.loader.clone(),
            loader_version: self.loader_version.clone(),
            minecraft_version: self.minecraft_version.clone(),
            project_dirs: self.project_dirs,
            game_dirs: self.game_dirs.clone(),
            java_dirs: self.java_dirs.clone(),
        }
    }
}

impl<'a> Version<'a> {
    pub fn new(
        name: &str,
        loader: Loader,
        loader_version: &str,
        minecraft_version: &str,
        project_dirs: &'a Lazy<ProjectDirs>,
    ) -> Self {
        Self {
            name: name.to_string(),
            loader,
            loader_version: loader_version.to_string(),
            minecraft_version: minecraft_version.to_string(),
            project_dirs,
            game_dirs: project_dirs.data_dir().join(name),
            java_dirs: project_dirs.config_dir().to_path_buf().join("jre"),
        }
    }


    pub async fn get_library(&self) -> Arc<VersionMetaData> {
        VANILLA.get(self,VanillaQuery::Libraries).await
            .expect("Failed to fetch vanilla libraries from manifest - version metadata unavailable")
    }
}


use crate::define_getters;

define_getters! {
    Version, std::sync::Arc<VersionMetaData>,

    // Vanilla
    get_main_class, VANILLA, VanillaQuery::MainClass;
    get_libraries, VANILLA, VanillaQuery::Libraries;
    get_natives, VANILLA, VanillaQuery::Natives;
    get_java_version, VANILLA, VanillaQuery::JavaVersion;
    get_assets, VANILLA, VanillaQuery::Assets;
    get_complete,VANILLA, VanillaQuery::VanillaBuilder;

    // Fabric
    get_fabric_libraries, FABRIC, FabricQuery::Libraries;
    get_fabric_complete, FABRIC, FabricQuery::FabricBuilder;

    // Quilt
    get_quilt_libraries, QUILT, QuiltQuery::Libraries;
    get_quilt_complete, QUILT, QuiltQuery::QuiltBuilder;
    
    //Neoforge
    get_neoforge_complete, NEOFORGE, NeoForgeQuery::NeoForgeBuilder;

    get_lighty_updater_complete, LIGHTY_UPDATER, LightyQuery::LightyBuilder;
    
}










