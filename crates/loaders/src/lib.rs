//! Per-loader manifest fetching and metadata extraction.
//!
//! Each entry in [`loaders`] (Vanilla, Fabric, Quilt, NeoForge, Forge,
//! OptiFine, LightyUpdater) implements the [`utils::query::Query`] trait
//! to describe how its remote manifest is retrieved and how to extract
//! libraries / arguments / natives / etc. The shared types live in
//! [`types`] and the caching machinery in [`utils`].

pub mod loaders;
pub mod utils;
pub mod types;

// Re-export commonly used items
pub use loaders::{
    fabric, forge, lighty_updater, neoforge, optifine, quilt, vanilla,
};

pub use utils::{
    cache, error, manifest, query,
};

// Re-export types
pub use types::{Loader, LoaderExtensions, VersionInfo, version_metadata};
