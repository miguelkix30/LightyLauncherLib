use lighty_launcher::{JavaDistribution, Launch, Loader, Version};
use directories::ProjectDirs;
use once_cell::sync::Lazy;
use tracing::{info, error};

static LAUNCHER_DIRECTORY: Lazy<ProjectDirs> =
    Lazy::new(|| {
        ProjectDirs::from("fr", ".LightyLauncher", "")
            .expect("Failed to create project directories")
    });

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let username = "Hamadi";
    let uuid = "37fefc81-1e26-4d31-a988-74196affc99b";
    let url = "http://localhost:8080";

    // Pour Vanilla
    let mut version = Version::new("minozia", Loader::LightyUpdater, url, "1.7.10", &LAUNCHER_DIRECTORY);


    match version.launch(username, uuid, JavaDistribution::Temurin).await {
        Ok(()) => info!("✅ Launch successful!"),
        Err(e) => error!("❌ Launch failed: {:?}", e),
    }
}