//! Concrete mod-loader implementations, one submodule per loader.
//!
//! Each submodule exposes a `Lazy<ManifestRepository<_>>` static (e.g.
//! [`vanilla::vanilla::VANILLA`], [`fabric::fabric::FABRIC`]) that caches
//! its manifest fetches and dispatches sub-queries via the [`crate::utils::query::Query`]
//! trait.

#[cfg(feature = "fabric")]
pub mod fabric;
#[cfg(feature = "forge")]
pub mod forge;
#[cfg(feature = "lighty_updater")]
pub mod lighty_updater;
#[cfg(feature = "neoforge")]
pub mod neoforge;
pub mod optifine;
#[cfg(feature = "quilt")]
pub mod quilt;
#[cfg(feature = "vanilla")]
pub mod vanilla;





