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
    tracing_subscriber::fmt::init();

    let username = "Hamadi";
    let uuid = "37fefc81-1e26-4d31-a988-74196affc99b";


    let mut fabric = Version::new("fabric", Loader::Fabric, "0.17.2", "1.21.8",&LAUNCHER_DIRECTORY);


    match fabric.launch(username, uuid, JavaDistribution::Temurin).await {
        Ok(()) => info!("✅ Launch successful!"),
        Err(e) => error!("❌ Launch failed: {:?}", e),
    }

}
