// Copyright (c) 2025 Hamadi
// Licensed under the MIT License

//! Forge / NeoForge install-processors wrappers.
//!
//! For each loader the flow is:
//! 1. Locate the cached installer JAR (loader-side helper).
//! 2. Extract any `/maven/...` artifacts bundled in the installer
//!    (Forge only; NeoForge doesn't ship them).
//! 3. Check the SHA1 marker — skip if the processors already ran against
//!    this exact installer.
//! 4. Run the processors through the shared executor.
//! 5. Write the marker on success.
//!
//! Lives in `lighty-launch` because step 4 spawns a JVM (using the
//! [`java_path`] resolved by the runner via `lighty_java`).

use std::path::PathBuf;

use lighty_loaders::types::VersionInfo;
use lighty_loaders::utils::error::QueryError;
use lighty_loaders::utils::forge_installer::ForgeInstallProfile;
use lighty_loaders::utils::maven::fetch_maven_sha1;

use super::processor::run_processors;
#[cfg(feature = "forge")]
use super::processor::extract_maven_bundle_to_libraries;

type Result<T> = std::result::Result<T, QueryError>;

/// Path to the per-installer marker file that records the SHA1 of the
/// installer whose processors last ran successfully.
fn processors_marker_path<V: VersionInfo>(version: &V, dot_dir: &str) -> PathBuf {
    version.game_dirs().join(dot_dir).join(format!(
        "processors-{}-{}.sha1",
        version.minecraft_version(),
        version.loader_version()
    ))
}

/// Runs the modern Forge install processors (≥ 1.13).
///
/// The caller must have already downloaded the install_profile libraries
/// (via the generic library installer pipeline) so the processor JARs
/// and their classpath dependencies are on disk before this is invoked.
#[cfg(feature = "forge")]
pub(crate) async fn run_forge_install_processors<V: VersionInfo>(
    version: &V,
    install_profile: &ForgeInstallProfile,
    java_path: PathBuf,
) -> Result<()> {
    use lighty_loaders::forge::forge::{
        build_installer_url, installer_cache_path, FORGE_EXTRACT_SUBDIR, FORGE_MAVEN,
    };

    lighty_core::trace_info!(loader = "forge", "Checking if processors need to run");

    let installer_path = installer_cache_path(version);
    if !installer_path.exists() {
        return Err(QueryError::Conversion {
            message: "Installer JAR not found. Run fetch_full_data first.".to_string(),
        });
    }

    // Some Forge versions ship runtime artifacts bundled at `/maven/...`
    // inside the installer (forge-shim.jar in 1.21+, forge-universal.jar
    // + forge.jar in 1.14, etc.). Idempotent.
    let libraries_dir = version.game_dirs().join("libraries");
    extract_maven_bundle_to_libraries(&installer_path, &libraries_dir)?;

    let installer_url = build_installer_url(version);
    let marker_path = processors_marker_path(version, ".forge");
    if let Some(expected_sha1) = fetch_maven_sha1(&installer_url).await {
        if let Ok(existing) = std::fs::read_to_string(&marker_path) {
            if existing.trim() == expected_sha1 {
                lighty_core::trace_info!(
                    loader = "forge",
                    "Processors already executed for this installer, skipping"
                );
                return Ok(());
            }
        }
    }

    run_processors(
        version,
        install_profile,
        installer_path,
        FORGE_MAVEN,
        FORGE_EXTRACT_SUBDIR,
        java_path,
    )
    .await?;

    if let Some(expected_sha1) = fetch_maven_sha1(&installer_url).await {
        if let Err(_err) = std::fs::write(&marker_path, expected_sha1) {
            lighty_core::trace_warn!(
                error = %_err,
                loader = "forge",
                "Failed to write processors marker file"
            );
        }
    }

    lighty_core::trace_info!(loader = "forge", "Processors completed successfully");
    Ok(())
}

/// Runs the NeoForge install processors.
///
/// The caller must have already downloaded the install_profile libraries
/// (via the generic library installer pipeline) so the processor JARs
/// and their classpath dependencies are on disk before this is invoked.
#[cfg(feature = "neoforge")]
pub(crate) async fn run_neoforge_install_processors<V: VersionInfo>(
    version: &V,
    install_profile: &ForgeInstallProfile,
    java_path: PathBuf,
) -> Result<()> {
    use lighty_loaders::neoforge::neoforge::{
        build_installer_url, installer_cache_path, NEOFORGE_EXTRACT_SUBDIR, NEOFORGE_MAVEN,
    };

    lighty_core::trace_info!(loader = "neoforge", "Checking if processors need to run");

    let installer_path = installer_cache_path(version);
    if !installer_path.exists() {
        return Err(QueryError::Conversion {
            message: "Installer JAR not found. Run fetch_full_data first.".to_string(),
        });
    }

    let installer_url = build_installer_url(version);
    let marker_path = processors_marker_path(version, ".neoforge");
    if let Some(expected_sha1) = fetch_maven_sha1(&installer_url).await {
        if let Ok(existing) = std::fs::read_to_string(&marker_path) {
            if existing.trim() == expected_sha1 {
                lighty_core::trace_info!(
                    loader = "neoforge",
                    "Processors already executed for this installer, skipping"
                );
                return Ok(());
            }
        }
    }

    run_processors(
        version,
        install_profile,
        installer_path,
        NEOFORGE_MAVEN,
        NEOFORGE_EXTRACT_SUBDIR,
        java_path,
    )
    .await?;

    if let Some(expected_sha1) = fetch_maven_sha1(&installer_url).await {
        if let Err(_err) = std::fs::write(&marker_path, expected_sha1) {
            lighty_core::trace_warn!(
                error = %_err,
                loader = "neoforge",
                "Failed to write processors marker file"
            );
        }
    }

    lighty_core::trace_info!(loader = "neoforge", "Processors completed successfully");
    Ok(())
}
