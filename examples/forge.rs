use lighty_launcher::prelude::*;

const QUALIFIER: &str = "fr";
const ORGANIZATION: &str = ".LightyLauncher";
const APPLICATION: &str = "";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    #[cfg(feature = "tracing")]
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let _app_state = AppState::new(
        QUALIFIER.to_string(),
        ORGANIZATION.to_string(),
        APPLICATION.to_string(),
    )?;

    let launcher_dir = AppState::get_project_dirs();

    // Authenticate (offline)
    let mut auth = OfflineAuth::new("Hamadi");
    #[cfg(feature = "events")]
    let profile = auth.authenticate(None).await?;
    #[cfg(not(feature = "events"))]
    let profile = auth.authenticate().await?;

    // Build and launch Forge instance (1.20.1 + Forge 47.3.0)
    let mut forge = VersionBuilder::new(
        "forge-test",
        Loader::Forge,
        "47.3.0",
        "1.20.1",
        launcher_dir,
    );

    forge
        .launch(&profile, JavaDistribution::Temurin)
        .run()
        .await?;

    trace_info!("Forge launch successful!");

    Ok(())
}
