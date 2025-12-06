// Copyright (c) 2025 Hamadi
// Licensed under the MIT License

//! Native libraries installation and extraction module

use std::path::PathBuf;
use zip::ZipArchive;
use tokio::fs;
use tracing::{error, info};
use lighty_loaders::types::{VersionInfo, version_metadata::Native};
use lighty_core::{mkdir, time_it};
use futures::future::try_join_all;
use crate::errors::{InstallerError, InstallerResult};
use super::verifier::needs_download;
use super::downloader::download_with_concurrency_limit;

/// Verifies, downloads and extracts native libraries
pub async fn verify_and_download_natives(
    version: &impl VersionInfo,
    natives: &[Native],
) -> InstallerResult<()> {
    if natives.is_empty() {
        return Ok(());
    }

    let libraries_path = version.game_dirs().join("libraries");
    let natives_extract_path = version.game_dirs().join("natives");

    // Clean natives folder on each installation
    if natives_extract_path.exists() {
        let _ = fs::remove_dir_all(&natives_extract_path).await;
    }
    mkdir!(natives_extract_path);

    // Separate into two phases: download then extraction
    let mut download_tasks = Vec::new();
    let mut extract_paths = Vec::new();

    for native in natives {
        let Some(url) = &native.url else { continue };
        let Some(path_str) = &native.path else { continue };

        let jar_path = libraries_path.join(path_str);

        if needs_download(&jar_path, native.sha1.as_ref(), &native.name).await {
            download_tasks.push((url.clone(), jar_path.clone()));
        }

        extract_paths.push(jar_path);
    }

    // Download missing natives
    if !download_tasks.is_empty() {
        info!("[Installer] Downloading {} natives...", download_tasks.len());
        time_it!("Natives download", {
            download_with_concurrency_limit(download_tasks).await?
        });
        info!("[Installer] ✓ Natives downloaded");
    } else {
        info!("[Installer] ✓ All natives already cached and verified");
    }

    // Extract all natives in parallel
    if !extract_paths.is_empty() {
        info!("[Installer] Extracting {} natives...", extract_paths.len());
        let extraction_tasks: Vec<_> = extract_paths
            .into_iter()
            .map(|jar_path| extract_native(jar_path, natives_extract_path.clone()))
            .collect();

        time_it!("Natives extraction", try_join_all(extraction_tasks).await?);
        info!("[Installer] ✓ Natives extracted");
    }

    Ok(())
}

/// Extracts a native JAR using memory-mapped I/O
async fn extract_native(jar_path: PathBuf, natives_dir: PathBuf) -> InstallerResult<()> {
    tokio::task::spawn_blocking(move || {
        let file = std::fs::File::open(&jar_path)?;
        //TODO: Replace memmap2 with async_zip from lighty-core
        let mmap = unsafe { memmap2::Mmap::map(&file)? };

        let cursor = std::io::Cursor::new(&mmap[..]);
        let mut archive = ZipArchive::new(cursor)?;

        // Extract native files
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let file_name = file.name().to_string();

            if is_native_file(&file_name) {
                let dest_path = natives_dir.join(
                    std::path::Path::new(&file_name)
                        .file_name()
                        .unwrap_or_default()
                );

                let mut dest_file = std::fs::File::create(&dest_path)?;
                std::io::copy(&mut file, &mut dest_file)?;
            }
        }

        Ok::<_, InstallerError>(())
    })
    .await
    .map_err(|e| InstallerError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?
}

/// Checks if a file is a native library
#[inline]
fn is_native_file(filename: &str) -> bool {
    const NATIVE_EXTENSIONS: &[&str] = &[".dll", ".so", ".dylib", ".jnilib"];

    let filename_lower = filename.to_lowercase();

    NATIVE_EXTENSIONS.iter().any(|ext| filename_lower.ends_with(ext))
        || filename_lower.contains(".so.")
}
