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

use lighty_core::system::{OperatingSystem, OS};
use lighty_core::download::download_file;
use lighty_core::extract::{tar_gz_extract, zip_extract};

use super::JavaDistribution;

/// Locates an existing Java binary in the runtime directory
///
/// Searches for the java executable in the expected directory structure
/// based on the distribution and version.
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
    let runtime_dir = build_runtime_path(runtimes_folder, distribution, version);

    let binary_path = locate_binary_in_directory(&runtime_dir).await?;

    // Ensure execution permissions on Unix systems
    #[cfg(unix)]
    ensure_executable_permissions(&binary_path).await?;

    Ok(binary_path.absolutize()?.to_path_buf())
}

/// Downloads and installs a JRE to the specified directory
///
/// # Arguments
/// * `runtimes_folder` - Base directory for JRE installation
/// * `distribution` - Java distribution to download
/// * `version` - Java major version number
/// * `on_progress` - Callback for download progress (bytes_downloaded, total_bytes)
///
/// # Returns
/// Path to the installed java binary
pub async fn jre_download<F>(
    runtimes_folder: &Path,
    distribution: &JavaDistribution,
    version: &u8,
    on_progress: F,
) -> JreResult<PathBuf>
where
    F: Fn(u64, u64),
{
    let runtime_dir = build_runtime_path(runtimes_folder, distribution, version);

    // Clean existing installation
    prepare_installation_directory(&runtime_dir).await?;

    // Get download URL
    let download_url = distribution
        .get_download_url(version)
        .await
        .map_err(|e| JreError::Download(format!("Failed to get download URL: {}", e)))?;

    // Download JRE archive
    let archive_bytes = download_file(&download_url, on_progress)
        .await
        .map_err(|e| JreError::Download(format!("Download failed: {}", e)))?;

    // Extract archive based on OS
    extract_archive(&archive_bytes, &runtime_dir).await?;

    // Locate and return the java binary
    find_java_binary(runtimes_folder, distribution, version).await
}

// ============================================================================
// Private Helper Functions
// ============================================================================

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

/// Extracts the JRE archive based on the operating system
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
/// The structure varies by OS:
/// - Windows: jre_root/bin/java.exe
/// - macOS: jre_root/Contents/Home/bin/java
/// - Linux: jre_root/bin/java
async fn locate_binary_in_directory(runtime_dir: &Path) -> JreResult<PathBuf> {
    // Find the first subdirectory (JRE root)
    let mut entries = fs::read_dir(runtime_dir).await?;

    let jre_root = entries
        .next_entry()
        .await?
        .ok_or_else(|| JreError::NotFound {
            path: runtime_dir.to_path_buf(),
        })?
        .path();

    // Build path to java binary based on OS
    let java_binary = match OS {
        OperatingSystem::WINDOWS => jre_root.join("bin").join("java.exe"),
        OperatingSystem::OSX => jre_root
            .join("Contents")
            .join("Home")
            .join("bin")
            .join("java"),
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
