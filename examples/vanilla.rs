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

    // Pour Vanilla
    let mut version = VersionBuilder::new("vanilla-1.7.10", Loader::Vanilla, "", "1.7.10", &LAUNCHER_DIRECTORY);


    match version.launch(username, uuid, JavaDistribution::Zulu).await {
        Ok(()) => info!("✅ Launch successful!"),
        Err(e) => error!("❌ Launch failed: {:?}", e),
    }
}