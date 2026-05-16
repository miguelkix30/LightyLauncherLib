// Copyright (c) 2025 Hamadi
// Licensed under the MIT License

//! Resource-installer steps: libraries, natives, client JAR, assets, mods.

// pub(crate) so the launch pipeline can feed Forge-family
// install_profile libraries through the same parallel-download/retry/SHA1
// logic used for the vanilla library set.
pub(crate) mod libraries;
pub(crate) mod mods;
pub(crate) mod natives;
pub(crate) mod client;
pub(crate) mod assets;
// User-attached mod resolver (Modrinth + CurseForge). Compiled only
// when at least one source feature is enabled — gated at the module
// boundary so disabling both lops it out of the binary cleanly.
#[cfg(any(feature = "modrinth", feature = "curseforge"))]
pub(crate) mod mod_resolver;
