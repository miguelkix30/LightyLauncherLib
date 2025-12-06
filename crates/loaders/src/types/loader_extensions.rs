use crate::types::version_metadata::VersionMetaData;
use crate::utils::error::QueryError;
use async_trait::async_trait;
use std::sync::Arc;

pub type Result<T> = std::result::Result<T, QueryError>;

/// Trait pour étendre VersionBuilder avec les méthodes spécifiques aux loaders
#[async_trait]
pub trait LoaderExtensions {
    async fn get_library(&self) -> Arc<VersionMetaData>;

    // Vanilla
    async fn get_main_class(&self) -> Result<Arc<VersionMetaData>>;
    async fn get_libraries(&self) -> Result<Arc<VersionMetaData>>;
    async fn get_natives(&self) -> Result<Arc<VersionMetaData>>;
    async fn get_java_version(&self) -> Result<Arc<VersionMetaData>>;
    async fn get_assets(&self) -> Result<Arc<VersionMetaData>>;
    async fn get_complete(&self) -> Result<Arc<VersionMetaData>>;

    // Fabric
    async fn get_fabric_libraries(&self) -> Result<Arc<VersionMetaData>>;
    async fn get_fabric_complete(&self) -> Result<Arc<VersionMetaData>>;

    // Quilt
    async fn get_quilt_libraries(&self) -> Result<Arc<VersionMetaData>>;
    async fn get_quilt_complete(&self) -> Result<Arc<VersionMetaData>>;

    // NeoForge
    async fn get_neoforge_complete(&self) -> Result<Arc<VersionMetaData>>;

    // LightyUpdater
    async fn get_lighty_updater_complete(&self) -> Result<Arc<VersionMetaData>>;
}
