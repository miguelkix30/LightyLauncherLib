//! Fabric launch example.
//!
//! `VersionBuilder::new(name, Loader::Fabric, loader_version, mc_version)`.
//!
//! - Supported MC versions: <https://meta.fabricmc.net/v2/versions/game>
//! - Loader versions:       <https://meta.fabricmc.net/v2/versions/loader>
//! - Compatibility per MC:  <https://meta.fabricmc.net/v2/versions/loader/{mc}>

use lighty_launcher::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    #[cfg(feature = "tracing")]
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

