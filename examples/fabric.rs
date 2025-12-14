use lighty_launcher::{
    auth::{OfflineAuth, Authenticator},
    java::JavaDistribution,
    launch::Launch,
    loaders::Loader,
    version::VersionBuilder,
};
use directories::ProjectDirs;
use once_cell::sync::Lazy;
use tracing::info;

static LAUNCHER_DIRECTORY: Lazy<ProjectDirs> =
    Lazy::new(|| {
        ProjectDirs::from("fr", ".LightyLauncher", "")
            .expect("Failed to create project directories")
    });

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    #[cfg(feature = "tracing")]
    tracing_subscriber::fmt::init();

    // Authentification offline
    let mut auth = OfflineAuth::new("Hamadi");
    let profile = auth.authenticate().await?;

    let mut fabric = VersionBuilder::new("fabric", Loader::Fabric, "0.17.2", "1.21.8", &LAUNCHER_DIRECTORY);

    fabric.launch(&profile, JavaDistribution::Temurin)
        .run()
        .await?;

    info!("Launch successful!");

    Ok(())
}
