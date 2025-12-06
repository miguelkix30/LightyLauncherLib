// Copyright (c) 2025 Hamadi
// Licensed under the MIT License

//! Mod installation module

use tracing::info;
use lighty_loaders::types::{VersionInfo, version_metadata::Mods};
use lighty_core::time_it;
use crate::errors::InstallerResult;
use super::verifier::needs_download;
use super::downloader::download_with_concurrency_limit;

/// Verifies and downloads missing or corrupted mods
pub async fn verify_and_download_mods(
    version: &impl VersionInfo,
    mods: &[Mods],
) -> InstallerResult<()> {
    let parent_path = version.game_dirs().join("mods");
    let mut tasks = Vec::new();

    for _mod in mods {
        let Some(url) = &_mod.url else { continue };
        let Some(path_str) = &_mod.path else { continue };

        let path = parent_path.join(path_str);

        if needs_download(&path, _mod.sha1.as_ref(), &_mod.name).await {
            tasks.push((url.clone(), path));
        }
    }

    if tasks.is_empty() {
        info!("[Installer] ✓ All mods already cached and verified");
        return Ok(());
    }

    info!("[Installer] Downloading {} mods...", tasks.len());
    time_it!("Mods download", {
        download_with_concurrency_limit(tasks).await?
    });
    info!("[Installer] ✓ Mods installed");
    Ok(())
}
