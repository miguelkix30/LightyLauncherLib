//! Shared types used by every loader.
//!
//! [`Loader`] is the loader enum, [`VersionInfo`] is the generic
//! "describes an installable instance" trait, [`LoaderExtensions`] is the
//! blanket trait adding `get_metadata()` and friends, [`version_metadata`]
//! holds the pivot metadata structures, and [`InstanceSize`] reports the
//! disk footprint of an installed instance.

pub mod version_info;
pub mod loader;
pub mod loader_extensions;
pub mod version_metadata;
pub mod instance_size;

pub use version_info::*;
pub use loader::*;
pub use loader_extensions::*;
pub use version_metadata::*;
pub use instance_size::*;
