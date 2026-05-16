// Copyright (c) 2025 Hamadi
// Licensed under the MIT License

//! LightyLauncher - A modern Minecraft launcher library
//!
//! This library provides everything needed to build a custom Minecraft launcher:
//! - Authentication (Offline, Microsoft, Azuriom) + trait-based extensibility
//! - Java runtime management
//! - Version metadata handling (Vanilla, Fabric, Quilt, Forge, NeoForge, LightyUpdater)
//! - Game installation and launching
//! - Event system for progress tracking
//!
//! ## Quick Start
//!
//! ```no_run
//! use lighty_launcher::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     AppState::init("MyLauncher")?;
//!
//!     // Authenticate
//!     let mut auth = auth::OfflineAuth::new("Player");
//!     let profile = auth.authenticate().await?;
//!
//!     // Build and launch
//!     let mut version = version::VersionBuilder::new(
//!         "my-instance",
//!         Loader::Vanilla,
//!         "",
//!         "1.21.1",
//!     );
//!
//!     version.launch(&profile, JavaDistribution::Temurin)
//!         .run()
//!         .await?;
//!
//!     Ok(())
//! }
//! ```

// ============================================================================
// Authentication Module
// ============================================================================

pub mod auth {
    //! Authentication providers and utilities
    //!
    //! ## Built-in Providers
    //! - `OfflineAuth` - No network required
    //! - `MicrosoftAuth` - OAuth 2.0 via Microsoft
    //! - `AzuriomAuth` - Azuriom CMS integration
    //!
    //! ## Custom Authentication
    //! Implement the `Authenticator` trait to create your own provider.
    //! See the [`lighty_auth`](https://docs.rs/lighty-auth) documentation for examples.

    pub use lighty_auth::{
        Authenticator,
        UserProfile,
        UserRole,
        AuthProvider,
        AuthResult,
        AuthError,
        generate_offline_uuid,
        offline::OfflineAuth,
        microsoft::MicrosoftAuth,
        azuriom::AzuriomAuth,
    };
}

// ============================================================================
// Events Module
// ============================================================================

#[cfg(feature = "events")]
pub mod event {
    //! Event system for tracking launcher operations
    //!
    //! Provides real-time progress updates for:
    //! - Authentication
    //! - Java runtime downloads
    //! - Game installation
    //! - Loader metadata fetching
    //! - Archive extraction

    pub use lighty_event::{
        EventBus,
        EventReceiver,
        Event,
        AuthEvent,
        JavaEvent,
        LaunchEvent,
        LoaderEvent,
        CoreEvent,
        InstanceLaunchedEvent,
        InstanceExitedEvent,
        ConsoleOutputEvent,
        InstanceDeletedEvent,
        ConsoleStream,
        EventReceiveError,
        EventTryReceiveError,
        EventSendError,
        EVENT_BUS,
    };
}

// ============================================================================
// Java Module
// ============================================================================

pub mod java {
    //! Java runtime management
    //!
    //! Handles automatic download and installation of Java distributions:
    //! - Temurin (Eclipse Adoptium)
    //! - GraalVM
    //! - Zulu (Azul)
    //! - Liberica (BellSoft)

    pub use lighty_java::{
        JavaDistribution,
        DistributionSelection,
        runtime::JavaRuntime,
        jre_downloader,
        JreError,
        JreResult,
        JavaRuntimeError,
        JavaRuntimeResult,
        DistributionError,
        DistributionResult,
    };
}

// ============================================================================
// Launch Module
// ============================================================================

pub mod launch {
    //! Game launching and installation
    //!
    //! Coordinates the complete launch process:
    //! - File verification and downloading
    //! - Library installation
    //! - Native library extraction
    //! - Asset management
    //! - Argument building
    //! - Process spawning

    pub use lighty_launch::{
        launch::{Launch, LaunchBuilder, LaunchConfig},
        installer::{
            Installer,
            config::{DownloaderConfig, init_downloader_config},
        },
        arguments::Arguments as LaunchArguments,
        errors::{InstallerError, InstallerResult},
        InstanceControl,
        InstanceError,
        InstanceResult,
    };

    /// Launch argument keys for customization
    pub mod keys {
        pub use lighty_launch::arguments::{
            KEY_AUTH_PLAYER_NAME,
            KEY_AUTH_UUID,
            KEY_AUTH_ACCESS_TOKEN,
            KEY_AUTH_XUID,
            KEY_CLIENT_ID,
            KEY_USER_TYPE,
            KEY_USER_PROPERTIES,
            KEY_VERSION_NAME,
            KEY_VERSION_TYPE,
            KEY_GAME_DIRECTORY,
            KEY_ASSETS_ROOT,
            KEY_NATIVES_DIRECTORY,
            KEY_LIBRARY_DIRECTORY,
            KEY_ASSETS_INDEX_NAME,
            KEY_LAUNCHER_NAME,
            KEY_LAUNCHER_VERSION,
            KEY_CLASSPATH,
            KEY_CLASSPATH_SEPARATOR,
        };
    }
}

// ============================================================================
// Loaders Module
// ============================================================================

pub mod loaders {
    //! Minecraft mod loaders and version metadata
    //!
    //! Supports multiple loader types:
    //! - Vanilla
    //! - Fabric
    //! - Quilt
    //! - Forge
    //! - NeoForge
    //! - LightyUpdater
    //! - OptiFine

    pub use lighty_loaders::{
        types::{
            Loader,
            VersionInfo,
            LoaderExtensions,
            InstanceSize,
            version_metadata::{
                Version,
                VersionMetaData,
                Library,
                Asset,
                AssetIndex,
                Arguments,
                MainClass,
                Mods,
                Native,
            },
        },
        utils::{cache, error, manifest, query},
    };

    // Per-loader re-exports (gated on the matching feature)
    #[cfg(feature = "vanilla")]
    pub use lighty_loaders::loaders::vanilla;
    #[cfg(feature = "fabric")]
    pub use lighty_loaders::loaders::fabric;
    #[cfg(feature = "quilt")]
    pub use lighty_loaders::loaders::quilt;
    #[cfg(feature = "forge")]
    pub use lighty_loaders::loaders::forge;
    #[cfg(feature = "neoforge")]
    pub use lighty_loaders::loaders::neoforge;
    #[cfg(feature = "lighty_updater")]
    pub use lighty_loaders::loaders::lighty_updater;
    pub use lighty_loaders::loaders::optifine;

    // Mod-source clients (gated on the matching feature) — exposes
    // `lighty_launcher::loaders::mods::{modrinth,curseforge}::set_api_key`
    // and the `ModRequest` / `ModKey` types.
    #[cfg(any(feature = "modrinth", feature = "curseforge"))]
    pub use lighty_loaders::mods;
}

// ============================================================================
// Version Module
// ============================================================================

pub mod version {
    //! Version builders for game instances
    //!
    //! - `VersionBuilder` - Standard Minecraft versions with loaders
    //! - `LightyVersionBuilder` - LightyUpdater custom versions

    pub use lighty_version::{
        VersionBuilder,
        LightyVersionBuilder,
    };
}

// ============================================================================
// Core Module
// ============================================================================

pub mod core {
    //! Core utilities and system operations
    //!
    //! Provides low-level functionality:
    //! - File system operations
    //! - HTTP client management
    //! - Archive extraction (ZIP, TAR.GZ)
    //! - SHA1 hashing and verification
    //! - Download utilities

    pub use lighty_core::{
        system,
        hosts,
        download,
        extract,
        hash,
        app_state::AppState,
        errors::{AppStateError, AppStateResult},
        SystemError,
        SystemResult,
        ExtractError,
        ExtractResult,
        DownloadError,
        DownloadResult,
        HashError,
        HashResult,
        verify_file_sha1,
        verify_file_sha1_streaming,
        calculate_file_sha1_sync,
        verify_file_sha1_sync,
        calculate_sha1_bytes,
        calculate_sha1_bytes_raw,
    };
}

// ============================================================================
// Macros Module
// ============================================================================

pub mod macros {
    //! Utility macros
    //!
    //! Provides conditional tracing macros that work with or without the `tracing` feature:
    //! - `trace_debug!()` - Debug level logging (no-op without `tracing` feature)
    //! - `trace_info!()` - Info level logging (no-op without `tracing` feature)
    //! - `trace_warn!()` - Warning level logging (no-op without `tracing` feature)
    //! - `trace_error!()` - Error level logging (no-op without `tracing` feature)
    //! - `time_it!()` - Performance timing (no-op without `tracing` feature)
    //!
    //! File system utilities:
    //! - `mkdir!()` - Async directory creation with error logging
    //! - `join_and_mkdir!()` - Join paths and create directory
    //! - `join_and_mkdir_vec!()` - Join multiple paths and create directory
    //! - `mkdir_blocking!()` - Blocking directory creation
    //!
    //! ## Example
    //!
    //! ```no_run
    //! use lighty_launcher::macros::*;
    //!
    //! async fn example() {
    //!     trace_info!("Starting operation");
    //!
    //!     let result = time_it!("my_operation", {
    //!         // Some expensive operation
    //!         42
    //!     });
    //!
    //!     let path = std::path::Path::new("/tmp/my_dir");
    //!     mkdir!(path);
    //! }
    //! ```

    pub use lighty_core::{
        trace_debug,
        trace_info,
        trace_warn,
        trace_error,
        time_it,
        mkdir,
        join_and_mkdir,
        join_and_mkdir_vec,
        mkdir_blocking,
    };
}

// ============================================================================
// Prelude - Commonly used imports
// ============================================================================

pub mod prelude {
    //! Convenient re-exports of most commonly used types
    //!
    //! ```
    //! use lighty_launcher::prelude::*;
    //! ```

    // Authentication
    pub use crate::auth::{
        Authenticator, UserProfile, AuthProvider, AuthError, UserRole,
        OfflineAuth, MicrosoftAuth, AzuriomAuth,
    };

    // Events
    #[cfg(feature = "events")]
    pub use crate::event::{
        EventBus, Event, AuthEvent, JavaEvent, LaunchEvent, LoaderEvent, CoreEvent,
        InstanceLaunchedEvent, InstanceExitedEvent, ConsoleOutputEvent, InstanceDeletedEvent,
        ConsoleStream, EVENT_BUS,
    };

    // Java
    pub use crate::java::JavaDistribution;

    // Launch
    pub use crate::launch::{
        Launch, LaunchBuilder, DownloaderConfig, init_downloader_config,
        InstanceControl, InstanceError, InstanceResult,
    };
    pub use crate::launch::keys::*;

    // Loaders
    pub use crate::loaders::{Loader, VersionInfo, LoaderExtensions, InstanceSize};

    // Version
    pub use crate::version::{VersionBuilder, LightyVersionBuilder};

    // Core utilities
    pub use crate::core::AppState;

    // Macros
    pub use crate::macros::{trace_debug, trace_info, trace_warn, trace_error};
}

// ============================================================================
// Root re-exports for convenience
// ============================================================================

// Most commonly used types at the root for convenience
pub use loaders::Loader;
pub use java::JavaDistribution;
pub use launch::Launch;
pub use auth::{Authenticator, UserProfile};
pub use version::{VersionBuilder, LightyVersionBuilder};

// Re-export the crates themselves for advanced usage
#[doc(hidden)]
pub use lighty_core as _core;
#[doc(hidden)]
pub use lighty_auth as _auth;
#[cfg(feature = "events")]
#[doc(hidden)]
pub use lighty_event as _event;
#[doc(hidden)]
pub use lighty_java as _java;
#[doc(hidden)]
pub use lighty_launch as _launch;
#[doc(hidden)]
pub use lighty_loaders as _loaders;
#[doc(hidden)]
pub use lighty_version as _version;
