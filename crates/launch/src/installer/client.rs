// Copyright (c) 2025 Hamadi
// Licensed under the MIT License

//! Client JAR installation module

use tracing::info;
use lighty_loaders::types::{VersionInfo, version_metadata::Client};
use lighty_core::time_it;
use crate::errors::InstallerResult;
use super::verifier::needs_download;
use super::downloader::download_large_file;

/// Verifies and downloads the client JAR if necessary
pub async fn verify_and_download_client(
    version: &impl VersionInfo,
    client: Option<&Client>,
) -> InstallerResult<()> {
    let Some(client) = client else {
        return Ok(());
    };

    let Some(url) = &client.url else {
        return Ok(());
    };

    let client_path = version.game_dirs().join(format!("{}.jar", version.name()));

    if !needs_download(&client_path, client.sha1.as_ref(), "Client JAR").await {
        info!("[Installer] ✓ Client JAR already cached and verified");
        return Ok(());
    }

    info!("[Installer] Downloading client JAR...");
    time_it!("Client download", download_large_file(url.clone(), client_path).await?);
    info!("[Installer] ✓ Client JAR installed");
    Ok(())
}
