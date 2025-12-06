// Copyright (c) 2025 Hamadi
// Licensed under the MIT License

//! Game installation orchestration module
//!
//! This module coordinates the installation of all game components:
//! - Libraries (game dependencies)
//! - Natives (platform-specific binaries)
//! - Client JAR (game executable)
//! - Assets (textures, sounds, etc.)
//! - Mods (optional modifications)

mod downloader;
mod verifier;
mod libraries;
mod mods;
mod natives;
mod client;
mod assets;

use tracing::{error, info};
use lighty_loaders::types::{VersionInfo, version_metadata::Version};
use lighty_core::{mkdir, time_it};
use crate::errors::InstallerResult;

// Re-export the trait
pub use self::installer_trait::Installer;

mod installer_trait {
    use super::*;

    /// Installation trait for version builders
    pub trait Installer {
        async fn install(&self, builder: &Version) -> InstallerResult<()>;
    }
}

impl<T: VersionInfo> Installer for T {
    /// Installs all dependencies in parallel
    async fn install(&self, builder: &Version) -> InstallerResult<()> {
        info!("[Installer] Starting installation for {}", self.name());

        time_it!("Total installation", {
            create_directories(self).await;

            // Verify and download in parallel
            tokio::try_join!(
                libraries::verify_and_download_libraries(self, &builder.libraries),
                natives::verify_and_download_natives(self,builder.natives.as_deref().unwrap_or(&[])),
                mods::verify_and_download_mods(self, builder.mods.as_deref().unwrap_or(&[])),
                client::verify_and_download_client(self, builder.client.as_ref()),
                assets::verify_and_download_assets(self, builder.assets.as_ref()),
            )?;
        });

        info!("[Installer] Installation completed successfully!");
        Ok(())
    }
}

/// Creates necessary installation directories
async fn create_directories(version: &impl VersionInfo) {
    let parent_path = version.game_dirs().to_path_buf();
    mkdir!(parent_path.join("libraries"));
    mkdir!(parent_path.join("natives"));
    mkdir!(parent_path.join("assets").join("objects"));
    mkdir!(parent_path.join("mods"));
}
