use lighty_launcher::prelude::*;

const QUALIFIER: &str = "com";
const ORGANIZATION: &str = ".LightyLauncher";
const APPLICATION: &str = "";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    #[cfg(feature = "tracing")]
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let _app_state = AppState::new(
        QUALIFIER.to_string(),
        ORGANIZATION.to_string(),
        APPLICATION.to_string(),
    )?;

    let launcher_dir = AppState::get_project_dirs();

    trace_info!("Starting Forge example...");
    trace_info!("Minecraft 1.20.1 with Forge 47.3.0");

    // Authenticate (offline mode for demo)
    let mut auth = OfflineAuth::new("ForgePlayer");
    #[cfg(feature = "events")]
    let profile = auth.authenticate(None).await?;
    #[cfg(not(feature = "events"))]
    let profile = auth.authenticate().await?;

    // Create Forge instance
    // Example: Minecraft 1.20.1 with Forge 47.3.0
    let mut forge_instance = VersionBuilder::new(
        "forge-1.20.1",
        Loader::Forge,
        "47.3.0",          // Forge version
        "1.20.1",          // Minecraft version
        launcher_dir
    );

    trace_info!("Launching Forge...");
    trace_info!("This will:");
    trace_info!("  1. Download and verify Forge installer JAR");
    trace_info!("  2. Extract metadata from installer (install_profile.json and version.json)");
    trace_info!("  3. Merge Forge libraries with Vanilla libraries");
    trace_info!("  4. Download all required libraries and assets");
    trace_info!("  5. Launch the game with proper Forge arguments");

    // Launch the game
    forge_instance
        .launch(&profile, JavaDistribution::Temurin)
        .run()
        .await?;

    trace_info!("Forge instance launched successfully!");

    // Keep the program running to see console output
    tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;

    Ok(())
}