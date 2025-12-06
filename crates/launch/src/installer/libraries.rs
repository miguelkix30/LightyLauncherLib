// Copyright (c) 2025 Hamadi
// Licensed under the MIT License

//! Library installation module

use tracing::info;
use lighty_loaders::types::{VersionInfo, version_metadata::Library};
use lighty_core::time_it;
use crate::errors::InstallerResult;
use super::verifier::needs_download;
use super::downloader::download_with_concurrency_limit;

/// Verifies and downloads missing or corrupted libraries
pub async fn verify_and_download_libraries(
    version: &impl VersionInfo,
    libraries: &[Library],
) -> InstallerResult<()> {
    let parent_path = version.game_dirs().join("libraries");
    let mut tasks = Vec::new();

    for lib in libraries {
        let Some(url) = &lib.url else { continue };
        let Some(path_str) = &lib.path else { continue };

        let path = parent_path.join(path_str);

        if needs_download(&path, lib.sha1.as_ref(), &lib.name).await {
            tasks.push((url.clone(), path));
        }
    }

    if tasks.is_empty() {
        info!("[Installer] ✓ All libraries already cached and verified");
        return Ok(());
    }

    info!("[Installer] Downloading {} libraries...", tasks.len());
    time_it!("Libraries download", {
        download_with_concurrency_limit(tasks).await?
    });
    info!("[Installer] ✓ Libraries installed");
    Ok(())
}
