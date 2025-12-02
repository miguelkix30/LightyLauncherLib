extern crate core;
pub mod utils;
pub mod java;
pub mod minecraft;

#[cfg(feature = "tauri-commands")]
pub mod tauri;

pub use java::JavaDistribution;
pub use minecraft::launch::launch::Launch;
pub use crate::minecraft::version::version::Loader;
pub use minecraft::version::version::Version;




