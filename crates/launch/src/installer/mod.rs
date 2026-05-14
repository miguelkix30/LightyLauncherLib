// Copyright (c) 2025 Hamadi
// Licensed under the MIT License

pub mod installer;
pub mod config;
mod downloader;
mod verifier;
// pub(crate) so the launch pipeline can feed the NeoForge install_profile
// libraries through the same parallel-download/retry/SHA1 logic.
pub(crate) mod libraries;
mod mods;
mod natives;
mod client;
mod assets;

// Re-export the Installer trait
pub use installer::Installer;
