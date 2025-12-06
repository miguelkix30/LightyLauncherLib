use lighty_loaders::loaders::lighty_updater::lighty_updater::{LIGHTY_UPDATER, LightyQuery};
use lighty_loaders::loaders::neoforge::neoforge::{NeoForgeQuery, NEOFORGE};
use lighty_loaders::loaders::quilt::quilt::{QuiltQuery, QUILT};
use lighty_loaders::loaders::fabric::fabric::{FabricQuery, FABRIC};
use lighty_loaders::loaders::vanilla::vanilla::{VanillaQuery, VANILLA};
use lighty_loaders::types::version_metadata::VersionMetaData;
use lighty_loaders::types::{Loader, LoaderExtensions};
use async_trait::async_trait;
use std::sync::Arc;

type Result<T> = std::result::Result<T, lighty_loaders::utils::error::QueryError>;

#[async_trait]
impl<'a> LoaderExtensions for crate::VersionBuilder<'a, Loader> {
    async fn get_library(&self) -> Arc<VersionMetaData> {
        VANILLA.get(self, VanillaQuery::Libraries).await
            .expect("Failed to fetch vanilla libraries from manifest - version metadata unavailable")
    }

    async fn get_main_class(&self) -> Result<Arc<VersionMetaData>> {
        VANILLA.get(&self, VanillaQuery::MainClass).await
    }

    async fn get_libraries(&self) -> Result<Arc<VersionMetaData>> {
        VANILLA.get(&self, VanillaQuery::Libraries).await
    }

    async fn get_natives(&self) -> Result<Arc<VersionMetaData>> {
        VANILLA.get(&self, VanillaQuery::Natives).await
    }

    async fn get_java_version(&self) -> Result<Arc<VersionMetaData>> {
        VANILLA.get(&self, VanillaQuery::JavaVersion).await
    }

    async fn get_assets(&self) -> Result<Arc<VersionMetaData>> {
        VANILLA.get(&self, VanillaQuery::Assets).await
    }

    async fn get_complete(&self) -> Result<Arc<VersionMetaData>> {
        VANILLA.get(&self, VanillaQuery::VanillaBuilder).await
    }

    async fn get_fabric_libraries(&self) -> Result<Arc<VersionMetaData>> {
        FABRIC.get(&self, FabricQuery::Libraries).await
    }

    async fn get_fabric_complete(&self) -> Result<Arc<VersionMetaData>> {
        FABRIC.get(&self, FabricQuery::FabricBuilder).await
    }

    async fn get_quilt_libraries(&self) -> Result<Arc<VersionMetaData>> {
        QUILT.get(&self, QuiltQuery::Libraries).await
    }

    async fn get_quilt_complete(&self) -> Result<Arc<VersionMetaData>> {
        QUILT.get(&self, QuiltQuery::QuiltBuilder).await
    }

    async fn get_neoforge_complete(&self) -> Result<Arc<VersionMetaData>> {
        NEOFORGE.get(&self, NeoForgeQuery::NeoForgeBuilder).await
    }

    async fn get_lighty_updater_complete(&self) -> Result<Arc<VersionMetaData>> {
        LIGHTY_UPDATER.get(&self, LightyQuery::LightyBuilder).await
    }
}
