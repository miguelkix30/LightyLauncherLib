use lighty_launcher::{
    auth::{OfflineAuth, Authenticator},
    java::JavaDistribution,
    launch::{Launch, DownloaderConfig, init_downloader_config, keys::KEY_LAUNCHER_NAME},
    loaders::Loader,
    version::VersionBuilder,
};
use directories::ProjectDirs;
use once_cell::sync::Lazy;
//use tracing::info;

static LAUNCHER_DIRECTORY: Lazy<ProjectDirs> =
    Lazy::new(|| {
        ProjectDirs::from("fr", ".LightyLauncher", "")
            .expect("Failed to create project directories")
    });

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //#[cfg(feature = "tracing")]
    //tracing_subscriber::fmt::init();

    // Configure le downloader (optionnel - valeurs par défaut si non appelé)
    init_downloader_config(
        DownloaderConfig {
        max_concurrent_downloads: 100,   // Plus de downloads concurrents car de base 50
        max_retries: 5,                  // Plus de tentatives car de base 3
        initial_delay_ms: 50,            // Délai initial plus long car de base 20
    }
    );

    // Authentification offline
    let mut auth = OfflineAuth::new("Hamadi");
    let profile = auth.authenticate().await?;

    let mut version = VersionBuilder::new("vanilla-1.21.1", Loader::Vanilla, "", "1.21.1", &LAUNCHER_DIRECTORY);

    version.launch(&profile, JavaDistribution::Termu)
        .with_jvm_options()
            .set("Xmx", "4G")
            .set("Xms", "2G")
            .done()
        .with_arguments()
            .set(KEY_LAUNCHER_NAME, "MyCustomLauncher")
            .set("width", "1920")
            .set("height", "1080")
            .done()
        .run()
        .await?;

    //info!("Launch successful!");

    Ok(())
}
