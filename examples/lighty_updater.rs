use lighty_launcher::{
    auth::{OfflineAuth, Authenticator},
    java::JavaDistribution,
    launch::Launch,
    version::LightyVersionBuilder,
    loaders::LoaderExtensions
};
use directories::ProjectDirs;
use once_cell::sync::Lazy;

//use tokio::{fs,io::AsyncWriteExt};


static LAUNCHER_DIRECTORY: Lazy<ProjectDirs> =
    Lazy::new(|| {
        ProjectDirs::from("fr", ".LightyLauncher", "")
            .expect("Failed to create project directories")
    });

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //tracing_subscriber::fmt() .with_max_level(tracing::Level::DEBUG) .init();

    let mut auth = OfflineAuth::new("Hamadi");
    let profile = auth.authenticate().await?;

    let url = "http://localhost:8080";

    // Pour LightyUpdater
    let mut version = LightyVersionBuilder::new("minozia", url, &LAUNCHER_DIRECTORY);

    let manifest = version.get_lighty_updater_complete().await?;


    // let content = format!("{:#?}", manifest);
    // let path = "manifest_debug.txt";
    // let mut file = fs::File::create(path).await?;
    // file.write_all(content.as_bytes()).await?;
    // file.flush().await?;


    //dbg!(&version);

    version.launch(&profile, JavaDistribution::Temurin)
        .run()
        .await?;


    Ok(())
}
