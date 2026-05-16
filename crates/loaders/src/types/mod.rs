//! Shared types used by every loader.
//!
//! - [`version`] holds the pivot metadata structs ([`version_metadata`])
//!   and the `VersionInfo` trait that abstracts over instance builders.
//! - [`loader`] holds the `Loader` enum and the `LoaderExtensions`
//!   blanket trait adding `get_metadata()` and friends.
//! - [`InstanceSize`] reports the disk footprint of an installed instance.

pub mod version;
pub mod loader;
pub mod instance_size;

// Public re-exports — every type lives at `lighty_loaders::types::*`
// regardless of the internal subfolder split.
pub use version::version_info::*;
pub use version::version_metadata::*;
pub use loader::loader::*;
pub use loader::loader_extensions::*;
pub use instance_size::*;

// Path re-exports so downstream code can still write `types::version_metadata`
// and `types::loader` without poking at the subfolder.
pub use version::version_metadata;
pub use loader::loader_extensions;
