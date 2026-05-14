//! Concrete mod-loader implementations, one submodule per loader.
//!
//! Each submodule exposes a `Lazy<ManifestRepository<_>>` static (e.g.
//! [`vanilla::vanilla::VANILLA`], [`fabric::fabric::FABRIC`]) that caches
//! its manifest fetches and dispatches sub-queries via the [`crate::utils::query::Query`]
//! trait.

pub mod fabric;
pub mod forge;
pub mod lighty_updater;
pub mod neoforge;
pub mod optifine;
pub mod quilt;
pub mod vanilla;





