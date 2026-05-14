//! Builders that describe a Minecraft instance to the rest of the library.
//!
//! [`VersionBuilder`] covers standard Vanilla-derived instances (Vanilla,
//! Fabric, Quilt, NeoForge, Forge); [`LightyVersionBuilder`] covers
//! LightyUpdater-managed instances where the loader and Minecraft version
//! are resolved at install time from a remote server.

pub mod version_builder;
pub mod lighty_builder;

// Re-export version_builder
pub use version_builder::*;

// Re-export lighty_builder
pub use lighty_builder::*;
