use lighty_launcher::prelude::*;
use lighty_java::JreError;

const QUALIFIER: &str = "fr";
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

    // Authenticate
    let mut auth = OfflineAuth::new("Hamadi");
    #[cfg(feature = "events")]
    let profile = auth.authenticate(None).await?;
    #[cfg(not(feature = "events"))]
    let profile = auth.authenticate().await?;

    // Configure LightyUpdater server URL
    let url = "http://dev.polargames";

    // Build LightyUpdater instance
    let mut version = LightyVersionBuilder::new("wynlers-dev", url, launcher_dir);

    // Fetch metadata to verify connection
    let _metadata = version.get_metadata().await?;
    trace_info!("LightyUpdater metadata fetched successfully");

    // Launch the game
    version.launch(&profile, JavaDistribution::Temurin)
        .run()
        .await?;

    trace_info!("LightyUpdater launch successful!");


    Ok(())
}
