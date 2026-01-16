// Copyright (c) 2025 Hamadi
// Licensed under the MIT License

//! Lighty Java - Java Runtime Management
//!
//! This crate provides functionality for managing Java Runtime Environments (JRE)
//! including downloading, installing, and executing Java processes.
//!
//! ## Features
//! - Support for multiple Java distributions (Temurin, GraalVM, Zulu, Liberica)
//! - Cross-platform JRE download and installation
//! - Java process execution with I/O streaming
//! - File size verification for download integrity
//!
//! ## License
//! This implementation is original work licensed under MIT.
//! It does not derive from GPL-licensed code.
//!
//! ## Clean Room Implementation
//! The distribution management system was implemented from scratch using only
//! publicly documented APIs from Adoptium, Oracle, Azul, and Foojay.

mod distribution;
pub mod jre_downloader;
pub mod runtime;
pub mod errors;

use serde::{Deserialize, Serialize};

pub use errors::{
    JreError, JreResult,
    JavaRuntimeError, JavaRuntimeResult,
    DistributionError, DistributionResult,
};

// ============================================================================
// Public Types
// ============================================================================

/// Selection method for Java distribution
#[derive(Deserialize, Serialize, Clone)]
#[serde(tag = "type", content = "value")]
pub enum DistributionSelection {
    #[serde(rename = "automatic")]
    Automatic(String),
    #[serde(rename = "custom")]
    Custom(String),
    #[serde(rename = "manual")]
    Manual(JavaDistribution),
}

impl Default for DistributionSelection {
    fn default() -> Self {
        DistributionSelection::Automatic(String::new())
    }
}

/// Available Java distributions
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub enum JavaDistribution {
    #[serde(rename = "temurin")]
    Temurin,
    #[serde(rename = "graalvm")]
    GraalVM,
    #[serde(rename = "zulu")]
    Zulu,
    #[serde(rename = "liberica")]
    Liberica,
}

impl Default for JavaDistribution {
    fn default() -> Self {
        // Temurin is the default as it supports all Java versions
        JavaDistribution::Temurin
    }
}

impl JavaDistribution {
    /// Returns the canonical name of this distribution
    pub fn get_name(&self) -> &str {
        match self {
            JavaDistribution::Temurin => "temurin",
            JavaDistribution::GraalVM => "graalvm",
            JavaDistribution::Zulu => "zulu",
            JavaDistribution::Liberica => "liberica",
        }
    }

    /// Checks if this distribution supports the given Java version on the current platform
    ///
    /// Some combinations are not available:
    /// - Temurin: No Java 8 for macOS ARM64 (Apple Silicon released after Java 8 EOL)
    /// - GraalVM: Only Java 17+
    pub fn supports_version(&self, version: u8) -> bool {
        use lighty_core::system::{Architecture, OperatingSystem, ARCHITECTURE, OS};

        match self {
            JavaDistribution::Temurin => {
                // No Java 8 for macOS ARM64
                !(version == 8 && OS == OperatingSystem::OSX && ARCHITECTURE == Architecture::AARCH64)
            }
            JavaDistribution::GraalVM => version >= 17,
            JavaDistribution::Zulu | JavaDistribution::Liberica => true,
        }
    }

    /// Returns a fallback distribution if this one doesn't support the version/platform
    ///
    /// Returns `None` if no fallback is needed (current distribution is supported)
    pub fn get_fallback(&self, version: u8) -> Option<JavaDistribution> {
        if self.supports_version(version) {
            return None;
        }

        // Find a compatible distribution
        let candidates = [
            JavaDistribution::Zulu,      // Best fallback: supports all versions on all platforms
            JavaDistribution::Liberica,  // Second choice
            JavaDistribution::Temurin,   // Third choice
        ];

        candidates.into_iter().find(|d| d.supports_version(version))
    }

    /// Gets download URL for the distribution
    ///
    /// Queries the respective API or builds direct download URLs for each distribution.
    pub async fn get_download_url(&self, jre_version: &u8) -> DistributionResult<String> {
        distribution::get_download_url(self, jre_version).await
    }
}