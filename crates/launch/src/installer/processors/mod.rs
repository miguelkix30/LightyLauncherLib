// Copyright (c) 2025 Hamadi
// Licensed under the MIT License

//! Install-processor pipeline for Forge-family loaders.
//!
//! Lives in `launch` (not `loaders`) because it spawns a JVM — reusing
//! the same JRE the runner resolved for the game launch.

pub(crate) mod processor;
pub(crate) mod forge_install;
