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
pub mod mods;

// Re-export commonly used items (each gated on its feature)
#[cfg(feature = "fabric")]
pub use loaders::fabric;
#[cfg(feature = "forge")]
pub use loaders::forge;
#[cfg(feature = "lighty_updater")]
pub use loaders::lighty_updater;
#[cfg(feature = "neoforge")]
pub use loaders::neoforge;
pub use loaders::optifine;
#[cfg(feature = "quilt")]
pub use loaders::quilt;
#[cfg(feature = "vanilla")]
pub use loaders::vanilla;

pub use utils::{
    cache, error, manifest, query,
};

// Re-export types
pub use types::{Loader, LoaderExtensions, VersionInfo, version_metadata};
