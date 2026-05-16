//! CurseForge mod fetching example (Fabric on MC 1.21.1 with JEI).
//!
//! Chain `.with_mod().with_curseforge(...)` on the `VersionBuilder` to
//! pull a list of mods from CurseForge by numeric mod ID. Optional pin
//! via `(mod_id, Some(file_id))`; `None` resolves the latest release
//! compatible with the instance's MC + loader. Required dependencies
//! are walked transitively.
//!
//! Unlike Modrinth, the CurseForge API requires an API key. Configure
//! it once before `.run()` via
//! [`lighty_launcher::loaders::mods::curseforge::set_api_key`].
//! Get a key at <https://console.curseforge.com/?#/api-keys>.
//!
//! `VersionBuilder::new(name, Loader::Fabric, loader_version, mc_version)`.
//!
//! - MC versions:             <https://piston-meta.mojang.com/mc/game/version_manifest_v2.json>
//! - Fabric loaders / MC:     <https://meta.fabricmc.net/v2/versions/loader/{mc}>
//! - CurseForge project page: `https://www.curseforge.com/minecraft/mc-mods/<slug>`
//!   (the numeric mod ID is shown on the right column, e.g. JEI = `238222`)
//! - CurseForge API:          <https://docs.curseforge.com/>

use lighty_launcher::loaders::mods::curseforge;
use lighty_launcher::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    #[cfg(feature = "tracing")]
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    AppState::init("LightyLauncher")?;

    // CurseForge API key — read from env to keep the key out of source.
    // Export `CURSEFORGE_API_KEY=...` before running the example.
    curseforge::set_api_key(std::env::var("CURSEFORGE_API_KEY")?);

    // Authenticate
    let mut auth = OfflineAuth::new("Hamadi");
    #[cfg(feature = "events")]
    let profile = auth.authenticate(None).await?;
    #[cfg(not(feature = "events"))]
    let profile = auth.authenticate().await?;

    let instance =
        VersionBuilder::new("curseforge-jei-1.21.1", Loader::Fabric, "0.17.2", "1.21.1");

    instance
        .with_mod()
            .with_curseforge(vec![(238222, None)]) // JEI — Just Enough Items
            .done()
        .launch(&profile, JavaDistribution::Temurin)
        .with_arguments()
            .set(KEY_GAME_DIRECTORY, "runtime") //better folder organization
            .done()
        .run()
        .await?;

    trace_info!("CurseForge launch successful!");

    Ok(())
}
