// Copyright (c) 2025 Hamadi
// Licensed under the MIT License

//! JRE Download and Installation
//!
//! This module handles downloading and extracting Java Runtime Environments.
//! Implementation is based on standard Rust async patterns and public APIs.

use std::io::Cursor;
use std::path::{Path, PathBuf};
use crate::errors::{JreError, JreResult};
use path_absolutize::Absolutize;
use tokio::fs;
use tokio::time::{sleep, Duration};

use lighty_core::system::{OperatingSystem, OS};
use lighty_core::download::download_file;
use lighty_core::DownloadError;
use lighty_core::extract::{tar_gz_extract, zip_extract};

use super::JavaDistribution;

#[cfg(feature = "events")]
use lighty_event::{EventBus, Event, JavaEvent};

/// Locates an existing Java binary in the runtime directory
///
/// Searches for the java executable in the expected directory structure
/// based on the distribution and version. Automatically uses fallback
/// distribution for unsupported version/platform combinations.
///
/// # Arguments
/// * `runtimes_folder` - Base directory containing installed JREs
/// * `distribution` - The Java distribution to locate
/// * `version` - Java major version number
///
/// # Returns
/// Absolute path to the java binary, or error if not found
pub async fn find_java_binary(
    runtimes_folder: &Path,
    distribution: &JavaDistribution,
    version: &u8,
) -> JreResult<PathBuf> {
    // Check if we need a fallback distribution
    let effective_distribution = distribution
        .get_fallback(*version)
        .unwrap_or_else(|| distribution.clone());

    let runtime_dir = build_runtime_path(runtimes_folder, &effective_distribution, version);

    let binary_path = locate_binary_in_directory(&runtime_dir).await?;

    // Ensure execution permissions on Unix systems
    #[cfg(unix)]
    ensure_executable_permissions(&binary_path).await?;

    Ok(binary_path.absolutize()?.to_path_buf())
}

/// Downloads and installs a JRE to the specified directory (with events feature)
///
/// # Arguments
/// * `runtimes_folder` - Base directory for JRE installation
/// * `distribution` - Java distribution to download
/// * `version` - Java major version number
/// * `on_progress` - Callback for download progress (bytes_downloaded, total_bytes)
/// * `event_bus` - Optional event bus for emitting events
///
/// # Returns
/// Path to the installed java binary
#[cfg(feature = "events")]
pub async fn jre_download<F>(
    runtimes_folder: &Path,
    distribution: &JavaDistribution,
    version: &u8,
    on_progress: F,
    event_bus: Option<&EventBus>,
) -> JreResult<PathBuf>
where
    F: Fn(u64, u64),
{
    // Check if we need a fallback distribution
    let effective_distribution = distribution
        .get_fallback(*version)
        .unwrap_or_else(|| distribution.clone());

    let runtime_dir = build_runtime_path(runtimes_folder, &effective_distribution, version);

    // Clean existing installation (retry on transient IO errors)
    prepare_installation_directory_with_retry(&runtime_dir).await?;

    let download_urls = build_download_candidates(&effective_distribution, version)
        .await
        .map_err(|e| JreError::Download(format!("Failed to get download URL: {}", e)))?;
    let primary_url = download_urls
        .first()
        .ok_or_else(|| JreError::Download("No download URLs available".to_string()))?;

    // Emit JavaDownloadStarted event
    let mut expected_total_bytes = 0;
    if let Some(bus) = event_bus {
        // Get total bytes first
        let response = lighty_core::hosts::HTTP_CLIENT
            .get(primary_url)
            .header("accept-encoding", "identity")
            .send()
            .await
            .map_err(|e| JreError::Download(format!("Failed to check file size: {}", e)))?;

        let encoding = response
            .headers()
            .get("content-encoding")
            .and_then(|value| value.to_str().ok())
            .unwrap_or("identity");
        let is_identity = encoding.eq_ignore_ascii_case("identity");
        let total_bytes = if is_identity {
            response.content_length().unwrap_or(0)
        } else {
            0
        };

        expected_total_bytes = total_bytes;

        bus.emit(Event::Java(JavaEvent::JavaDownloadStarted {
            distribution: effective_distribution.get_name().to_string(),
            version: *version,
            total_bytes,
        }));
    }

    // Download JRE archive with progress tracking
    let archive_bytes = {
        let event_bus_ref = event_bus;
        let progress_cb = |current: u64, total: u64| {
            on_progress(current, total);

            if let Some(bus) = event_bus_ref {
                if current > 0 {
                    let effective_total = if total > 0 {
                        total
                    } else {
                        expected_total_bytes
                    };
                    let percent = if effective_total > 0 {
                        let raw = (current as f64 / effective_total as f64) * 100.0;
                        raw.min(100.0)
                    } else {
                        0.0
                    };

                    bus.emit(Event::Java(JavaEvent::JavaDownloadProgress {
                        percent: percent as f32,
                    }));
                }
            }
        };

        download_with_retries(&download_urls, &progress_cb).await?
    };

    // Emit JavaDownloadCompleted event
    if let Some(bus) = event_bus {
        bus.emit(Event::Java(JavaEvent::JavaDownloadCompleted {
            distribution: effective_distribution.get_name().to_string(),
            version: *version,
        }));
    }

    // Emit JavaExtractionStarted event
    if let Some(bus) = event_bus {
        bus.emit(Event::Java(JavaEvent::JavaExtractionStarted {
            distribution: effective_distribution.get_name().to_string(),
            version: *version,
        }));
    }

    // Extract archive based on OS
    extract_archive(
        &archive_bytes,
        &runtime_dir,
        event_bus,
    ).await?;

    // Locate and return the java binary
    let binary_path = find_java_binary(runtimes_folder, &effective_distribution, version).await?;

    // Emit JavaExtractionCompleted event
    if let Some(bus) = event_bus {
        bus.emit(Event::Java(JavaEvent::JavaExtractionCompleted {
            distribution: effective_distribution.get_name().to_string(),
            version: *version,
            binary_path: binary_path.to_string_lossy().to_string(),
        }));
    }

    Ok(binary_path)
}

/// Downloads and installs a JRE to the specified directory (without events feature)
///
/// # Arguments
/// * `runtimes_folder` - Base directory for JRE installation
/// * `distribution` - Java distribution to download
/// * `version` - Java major version number
/// * `on_progress` - Callback for download progress (bytes_downloaded, total_bytes)
///
/// # Returns
/// Path to the installed java binary
#[cfg(not(feature = "events"))]
pub async fn jre_download<F>(
    runtimes_folder: &Path,
    distribution: &JavaDistribution,
    version: &u8,
    on_progress: F,
) -> JreResult<PathBuf>
where
    F: Fn(u64, u64),
{
    // Check if we need a fallback distribution
    let effective_distribution = distribution
        .get_fallback(*version)
        .unwrap_or_else(|| distribution.clone());

    let runtime_dir = build_runtime_path(runtimes_folder, &effective_distribution, version);

    // Clean existing installation (retry on transient IO errors)
    prepare_installation_directory_with_retry(&runtime_dir).await?;

    let download_urls = build_download_candidates(&effective_distribution, version)
        .await
        .map_err(|e| JreError::Download(format!("Failed to get download URL: {}", e)))?;

    // Download JRE archive
    let archive_bytes = {
        let progress_cb = |current: u64, total: u64| {
            on_progress(current, total);
        };

        download_with_retries(&download_urls, &progress_cb).await?
    };

    // Extract archive based on OS
    extract_archive(&archive_bytes, &runtime_dir).await?;

    // Locate and return the java binary
    find_java_binary(runtimes_folder, &effective_distribution, version).await
}

// ============================================================================
// Private Helper Functions
// ============================================================================

async fn build_download_candidates(
    distribution: &JavaDistribution,
    version: &u8,
) -> JreResult<Vec<String>> {
    let mut urls = Vec::new();

    let primary = distribution
        .get_download_url(version)
        .await
        .map_err(|e| JreError::Download(format!("Failed to get download URL: {}", e)))?;
    urls.push(primary);

    if matches!(distribution, JavaDistribution::Temurin) {
        for candidate in [JavaDistribution::Zulu, JavaDistribution::Liberica] {
            if candidate.supports_version(*version) {
                if let Ok(url) = candidate.get_download_url(version).await {
                    urls.push(url);
                }
            }
        }
    }

    Ok(urls)
}

/// Constructs the runtime installation path for a given distribution and version
fn build_runtime_path(
    runtimes_folder: &Path,
    distribution: &JavaDistribution,
    version: &u8,
) -> PathBuf {
    // Optimized: Build path directly without intermediate String allocation
    let mut path = runtimes_folder.to_path_buf();
    path.push(format!("{}_{}", distribution.get_name(), version));
    path
}

/// Prepares the installation directory by removing existing files
async fn prepare_installation_directory(runtime_dir: &Path) -> JreResult<()> {
    if runtime_dir.exists() {
        fs::remove_dir_all(runtime_dir).await?;
    }
    fs::create_dir_all(runtime_dir).await?;
    Ok(())
}

async fn prepare_installation_directory_with_retry(runtime_dir: &Path) -> JreResult<()> {
    const MAX_IO_RETRIES: usize = 3;
    const RETRY_DELAY_MS: u64 = 400;

    let mut last_error: Option<JreError> = None;

    for attempt in 1..=MAX_IO_RETRIES {
        match prepare_installation_directory(runtime_dir).await {
            Ok(()) => return Ok(()),
            Err(JreError::Io(err))
                if err.kind() == std::io::ErrorKind::PermissionDenied
                    && attempt < MAX_IO_RETRIES =>
            {
                last_error = Some(JreError::Io(err));
                lighty_core::trace_warn!(
                    "[Java] Permission denied preparing {:?}, retrying ({}/{})",
                    runtime_dir,
                    attempt,
                    MAX_IO_RETRIES
                );
                sleep(Duration::from_millis(RETRY_DELAY_MS)).await;
            }
            Err(err) => return Err(err),
        }
    }

    Err(last_error.unwrap_or_else(|| {
        JreError::Download("Failed to prepare runtime directory".to_string())
    }))
}

async fn download_with_retries<F>(
    download_urls: &[String],
    on_progress: &F,
) -> JreResult<Vec<u8>>
where
    F: Fn(u64, u64),
{
    const MAX_IO_RETRIES: usize = 3;
    const RETRY_DELAY_MS: u64 = 400;

    let mut last_error: Option<DownloadError> = None;

    for url in download_urls {
        for attempt in 1..=MAX_IO_RETRIES {
            let result = download_file(url, |current, total| on_progress(current, total)).await;

            match result {
                Ok(bytes) => return Ok(bytes),
                Err(err) => {
                    let should_retry = matches!(
                        err,
                        DownloadError::Io(ref io) if io.kind() == std::io::ErrorKind::PermissionDenied
                    );

                    last_error = Some(err);

                    if should_retry && attempt < MAX_IO_RETRIES {
                        lighty_core::trace_warn!(
                            "[Java] Permission denied downloading {}, retrying ({}/{})",
                            url,
                            attempt,
                            MAX_IO_RETRIES
                        );
                        sleep(Duration::from_millis(RETRY_DELAY_MS)).await;
                        continue;
                    }

                    break;
                }
            }
        }
    }

    Err(JreError::Download(format!(
        "Download failed: {}",
        last_error
            .map(|e| e.to_string())
            .unwrap_or_else(|| "No candidates available for download".to_string())
    )))
}

/// Extracts the JRE archive based on the operating system (with events feature)
#[cfg(feature = "events")]
async fn extract_archive(
    archive_bytes: &[u8],
    destination: &Path,
    event_bus: Option<&EventBus>,
) -> JreResult<()> {
    let cursor = Cursor::new(archive_bytes);

    match OS {
        OperatingSystem::WINDOWS => {
            zip_extract(cursor, destination, event_bus)
                .await
                .map_err(|e| JreError::Extraction(format!("ZIP extraction failed: {}", e)))?;
        }
        OperatingSystem::LINUX | OperatingSystem::OSX => {
            tar_gz_extract(cursor, destination, event_bus)
                .await
                .map_err(|e| JreError::Extraction(format!("TAR.GZ extraction failed: {}", e)))?;
        }
        OperatingSystem::UNKNOWN => {
            return Err(JreError::UnsupportedOS);
        }
    }

    Ok(())
}

/// Extracts the JRE archive based on the operating system (without events feature)
#[cfg(not(feature = "events"))]
async fn extract_archive(archive_bytes: &[u8], destination: &Path) -> JreResult<()> {
    let cursor = Cursor::new(archive_bytes);

    match OS {
        OperatingSystem::WINDOWS => {
            zip_extract(cursor, destination)
                .await
                .map_err(|e| JreError::Extraction(format!("ZIP extraction failed: {}", e)))?;
        }
        OperatingSystem::LINUX | OperatingSystem::OSX => {
            tar_gz_extract(cursor, destination)
                .await
                .map_err(|e| JreError::Extraction(format!("TAR.GZ extraction failed: {}", e)))?;
        }
        OperatingSystem::UNKNOWN => {
            return Err(JreError::UnsupportedOS);
        }
    }

    Ok(())
}

/// Locates the java binary within the extracted JRE directory
///
/// The structure varies by OS and distribution:
/// - Windows: jre_root/bin/java.exe
/// - macOS (bundle): jre_root/Contents/Home/bin/java (Temurin)
/// - macOS (nested bundle): jre_root/*.jre/Contents/Home/bin/java (Zulu Java 8)
/// - macOS (flat): jre_root/bin/java (Liberica tar.gz)
/// - Linux: jre_root/bin/java
async fn locate_binary_in_directory(runtime_dir: &Path) -> JreResult<PathBuf> {
    // Iterate entries to find the best candidate JRE root directory.
    // Prefer directories that contain expected java binaries (bin/java, bin/java.exe,
    // or Contents/Home/bin/java). As a fallback, use the first directory found.
    let mut entries = fs::read_dir(runtime_dir).await?;
    let mut first_dir: Option<PathBuf> = None;
    let mut candidate: Option<PathBuf> = None;

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.is_dir() {
            // Save first directory as fallback
            if first_dir.is_none() {
                first_dir = Some(path.clone());
            }

            // Heuristics: check common binary locations inside the directory
            let looks_like_jre = path.join("bin").join("java").exists()
                || path.join("bin").join("java.exe").exists()
                || path.join("Contents").join("Home").join("bin").join("java").exists();

            if looks_like_jre {
                candidate = Some(path);
                break;
            }
        }
    }

    let jre_root = if let Some(c) = candidate {
        c
    } else if let Some(f) = first_dir {
        f
    } else {
        return Err(JreError::NotFound {
            path: runtime_dir.to_path_buf(),
        });
    };

    // Build path to java binary based on OS
    let java_binary = match OS {
        OperatingSystem::WINDOWS => jre_root.join("bin").join("java.exe"),
        OperatingSystem::OSX => {
            // macOS: Try multiple structures in order of likelihood

            // 1. Direct bundle: jre_root/Contents/Home/bin/java (Temurin, most Zulu versions)
            let bundle_path = jre_root.join("Contents").join("Home").join("bin").join("java");
            if bundle_path.exists() {
                bundle_path
            }
            // 2. Nested .jre bundle: jre_root/*.jre/Contents/Home/bin/java (Zulu Java 8)
            else if let Some(nested) = find_nested_jre_bundle(&jre_root).await {
                nested
            }
            // 3. Flat structure: jre_root/bin/java (Liberica tar.gz)
            else {
                jre_root.join("bin").join("java")
            }
        }
        _ => jre_root.join("bin").join("java"),
    };

    // Verify the binary exists
    if !java_binary.exists() {
        return Err(JreError::NotFound {
            path: java_binary.clone(),
        });
    }

    Ok(java_binary)
}

/// Finds a nested .jre bundle inside the JRE root (Zulu Java 8 on macOS)
#[cfg(target_os = "macos")]
async fn find_nested_jre_bundle(jre_root: &Path) -> Option<PathBuf> {
    let mut entries = fs::read_dir(jre_root).await.ok()?;

    while let Ok(Some(entry)) = entries.next_entry().await {
        let path = entry.path();
        if path.is_dir() {
            let name = path.file_name()?.to_str()?;
            if name.ends_with(".jre") {
                let java_path = path.join("Contents").join("Home").join("bin").join("java");
                if java_path.exists() {
                    return Some(java_path);
                }
            }
        }
    }
    None
}

#[cfg(not(target_os = "macos"))]
async fn find_nested_jre_bundle(_jre_root: &Path) -> Option<PathBuf> {
    None
}

/// Ensures the java binary has execution permissions on Unix systems
#[cfg(unix)]
async fn ensure_executable_permissions(binary_path: &Path) -> JreResult<()> {
    use std::os::unix::fs::PermissionsExt;

    let metadata = fs::metadata(binary_path).await?;
    let current_permissions = metadata.permissions();

    // Check if any execute bit is set (owner, group, or other)
    if current_permissions.mode() & 0o111 == 0 {
        // No execute permissions, set them (rwxr-xr-x)
        let mut new_permissions = current_permissions;
        new_permissions.set_mode(0o755);
        fs::set_permissions(binary_path, new_permissions).await?;
    }

    Ok(())
}
