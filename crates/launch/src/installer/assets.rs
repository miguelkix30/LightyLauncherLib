// Copyright (c) 2025 Hamadi
// Licensed under the MIT License

//! Assets installation module

use tracing::info;
use lighty_loaders::types::{VersionInfo, version_metadata::AssetsFile};
use lighty_core::time_it;
use crate::errors::InstallerResult;
use super::verifier::needs_download;
use super::downloader::download_small_with_concurrency_limit;

/// Verifies and downloads missing or corrupted assets
pub async fn verify_and_download_assets(
    version: &impl VersionInfo,
    assets: Option<&AssetsFile>,
) -> InstallerResult<()> {
    let Some(assets) = assets else {
        return Ok(());
    };

    let parent_path = version.game_dirs().join("assets").join("objects");
    let mut tasks = Vec::new();

    for asset in assets.objects.values() {
        let Some(url) = &asset.url else { continue };

        // Use first 2 characters of hash as subdirectory
        let hash_prefix = &asset.hash[0..2];
        let path = parent_path.join(hash_prefix).join(&asset.hash);

        if needs_download(&path, Some(&asset.hash), &asset.hash).await {
            tasks.push((url.clone(), path));
        }
    }

    if tasks.is_empty() {
        info!("[Installer] ✓ All assets already cached and verified");
        return Ok(());
    }

    info!("[Installer] Downloading {} new assets...", tasks.len());
    time_it!("Assets download", {
        download_small_with_concurrency_limit(tasks).await?
    });
    info!("[Installer] ✓ Assets installed");
    Ok(())
}
