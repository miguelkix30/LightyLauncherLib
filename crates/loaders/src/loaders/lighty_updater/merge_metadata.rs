use crate::types::{VersionInfo, Loader, LoaderExtensions};
use crate::types::version_metadata::{Version, VersionMetaData};
use crate::utils::error::QueryError;

pub type Result<T> = std::result::Result<T, QueryError>;

/// Merge les métadonnées de Lighty Updater avec un autre loader
/// selon le loader spécifié dans ServerInfo
pub async fn merge_metadata<V: VersionInfo>(version: &V, loader: &str) -> Result<Version> {
    lighty_core::trace_debug!("[merge_metadata] START with loader={}", loader);

    // Map string loader to Loader enum
    let loader_type = match loader {
        "vanilla" => Loader::Vanilla,
        "fabric" => Loader::Fabric,
        "quilt" => Loader::Quilt,
        "neoforge" => Loader::NeoForge,
        _ => {
            lighty_core::trace_error!("[merge_metadata] Unknown loader: {}", loader);
            return Err(QueryError::UnsupportedLoader(
                format!("Unknown loader '{}' - please check your LightyUpdater config", loader)
            ))
        }
    };

    lighty_core::trace_debug!("[merge_metadata] Loader mapped: {:?}", loader_type);

    // Create temporary VersionInfo with the correct loader
    let temp_version = TempVersionInfo {
        name: version.name().to_string(),
        loader_version: version.loader_version().to_string(),
        minecraft_version: version.minecraft_version().to_string(),
        game_dirs: version.game_dirs().to_path_buf(),
        java_dirs: version.java_dirs().to_path_buf(),
        loader: loader_type,
    };

    // Use get_metadata() to fetch the builder data
    lighty_core::trace_debug!("[merge_metadata] Fetching base loader data using get_metadata()...");
    let metadata = temp_version.get_metadata().await?;

    // Extract Version from VersionMetaData
    let merged_metadata = match &*metadata {
        VersionMetaData::Version(version) => version.clone(),
        _ => {
            lighty_core::trace_error!("❌ [merge_metadata] Expected Version metadata");
            return Err(QueryError::UnsupportedLoader(
                "Failed to extract Version from metadata".to_string()
            ))
        }
    };

    lighty_core::trace_debug!("[merge_metadata] Base loader data fetched successfully");
    Ok(merged_metadata)
}

/// Temporary VersionInfo implementation for merge operations
#[derive(Clone)]
struct TempVersionInfo {
    name: String,
    loader_version: String,
    minecraft_version: String,
    game_dirs: std::path::PathBuf,
    java_dirs: std::path::PathBuf,
    loader: Loader,
}

impl VersionInfo for TempVersionInfo {
    type LoaderType = Loader;

    fn name(&self) -> &str {
        &self.name
    }

    fn loader_version(&self) -> &str {
        &self.loader_version
    }

    fn minecraft_version(&self) -> &str {
        &self.minecraft_version
    }

    fn game_dirs(&self) -> &std::path::Path {
        &self.game_dirs
    }

    fn java_dirs(&self) -> &std::path::Path {
        &self.java_dirs
    }

    fn loader(&self) -> &Self::LoaderType {
        &self.loader
    }
}
