use lighty_launcher::{JavaDistribution, Launch, Loader, VersionBuilder};
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
    tracing_subscriber::fmt::init();

    let username = "Hamadi";
    let uuid = "37fefc81-1e26-4d31-a988-74196affc99b";


    let mut quilt = VersionBuilder::new("quilt", Loader::Quilt, "0.17.10", "1.18.2",&LAUNCHER_DIRECTORY);


    match quilt.launch(username, uuid, JavaDistribution::Temurin).await {
        Ok(()) => info!("✅ Launch successful!"),
        Err(e) => error!("❌ Launch failed: {:?}", e),
    }
}