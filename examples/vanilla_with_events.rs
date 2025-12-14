use lighty_launcher::{
    auth::{OfflineAuth, Authenticator},
    event::{EventBus, Event, LaunchEvent, AuthEvent, JavaEvent, LoaderEvent, CoreEvent},
    java::JavaDistribution,
    launch::Launch,
    loaders::Loader,
    version::VersionBuilder,
};
use directories::ProjectDirs;
use once_cell::sync::Lazy;

static LAUNCHER_DIRECTORY: Lazy<ProjectDirs> =
    Lazy::new(|| ProjectDirs::from("fr", ".LightyLauncher", "")
        .expect("Failed to create project directories"));

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //tracing_subscriber::fmt().with_env_filter("info").init();

    // Create event bus
    let event_bus = EventBus::new(1000);

    // Spawn listener
    let mut receiver = event_bus.subscribe();

    tokio::spawn(async move {
        let mut total = 0u64;
        let mut downloaded = 0u64;

        while let Ok(event) = receiver.next().await {
            match event {
                Event::Auth(AuthEvent::AuthenticationStarted { provider }) => {
                    println!("Authenticating with {}...", provider);
                }
                Event::Auth(AuthEvent::AuthenticationSuccess { username, .. }) => {
                    println!("Authenticated as {}", username);
                }
                Event::Auth(AuthEvent::AuthenticationFailed { error, .. }) => {
                    println!("Authentication failed: {}", error);
                }
                Event::Launch(LaunchEvent::IsInstalled { version }) => {
                    println!("{} is already installed and up-to-date!", version);
                    break;
                }
                Event::Launch(LaunchEvent::InstallStarted { total_bytes, .. }) => {
                    total = total_bytes;
                    println!("Installing: {} MB total", total / 1_000_000);
                }
                Event::Launch(LaunchEvent::InstallProgress { bytes }) => {
                    downloaded += bytes;
                    let percent = (downloaded as f64 / total as f64) * 100.0;
                    print!("\rProgress: {:.1}%", percent);
                    std::io::Write::flush(&mut std::io::stdout()).ok();
                }
                Event::Launch(LaunchEvent::InstallCompleted { .. }) => {
                    println!("\nInstallation completed!");
                    break;
                }
                Event::Java(JavaEvent::JavaNotFound { distribution, version }) => {
                    println!("[Java] {} {} not found, downloading...", distribution, version);
                }
                Event::Java(JavaEvent::JavaAlreadyInstalled { distribution, version, .. }) => {
                    println!("[Java] {} {} already installed", distribution, version);
                }
                Event::Java(JavaEvent::JavaDownloadStarted { distribution, version, total_bytes }) => {
                    println!("[Java] Downloading {} {} ({} MB)", distribution, version, total_bytes / 1_000_000);
                }
                Event::Java(JavaEvent::JavaDownloadProgress { bytes }) => {
                    print!("\r[Java] Download progress: {} MB", bytes / 1_000_000);
                    std::io::Write::flush(&mut std::io::stdout()).ok();
                }
                Event::Java(JavaEvent::JavaDownloadCompleted { distribution, version }) => {
                    println!("\n[Java] {} {} download completed", distribution, version);
                }
                Event::Java(JavaEvent::JavaExtractionStarted { distribution, version }) => {
                    println!("[Java] Extracting {} {}...", distribution, version);
                }
                Event::Java(JavaEvent::JavaExtractionCompleted { distribution, version, .. }) => {
                    println!("[Java] {} {} extraction completed", distribution, version);
                }
                Event::Loader(LoaderEvent::FetchingData { loader, minecraft_version, loader_version }) => {
                    println!("[Loader] Fetching {} data for Minecraft {} (loader version: {})", loader, minecraft_version, loader_version);
                }
                Event::Loader(LoaderEvent::DataFetched { loader, .. }) => {
                    println!("[Loader] {} data fetched successfully", loader);
                }
                Event::Loader(LoaderEvent::ManifestCached { loader }) => {
                    println!("[Loader] Using cached {} manifest", loader);
                }
                Event::Loader(LoaderEvent::MergingLoaderData { base_loader, overlay_loader }) => {
                    println!("[Loader] Merging {} with {}", overlay_loader, base_loader);
                }
                Event::Loader(LoaderEvent::DataMerged { base_loader, overlay_loader }) => {
                    println!("[Loader] {} and {} merged successfully", overlay_loader, base_loader);
                }
                Event::Core(CoreEvent::ExtractionStarted { archive_type, file_count, .. }) => {
                    if file_count > 0 {
                        println!("[Core] Extracting {} archive ({} files)...", archive_type, file_count);
                    } else {
                        println!("[Core] Extracting {} archive...", archive_type);
                    }
                }
                Event::Core(CoreEvent::ExtractionProgress { files_extracted, total_files }) => {
                    if total_files > 0 {
                        let percent = (files_extracted as f64 / total_files as f64) * 100.0;
                        print!("\r[Core] Extraction progress: {}/{} files ({:.1}%)", files_extracted, total_files, percent);
                    } else {
                        print!("\r[Core] Extraction progress: {} files", files_extracted);
                    }
                    std::io::Write::flush(&mut std::io::stdout()).ok();
                }
                Event::Core(CoreEvent::ExtractionCompleted { archive_type, files_extracted }) => {
                    println!("\n[Core] {} extraction completed ({} files)", archive_type, files_extracted);
                }
                _ => {}
            }
        }
    });

    // Authenticate
    let mut auth = OfflineAuth::new("Player");
    let profile = auth.authenticate(Some(&event_bus)).await?;

    // Build and launch
    let mut version = VersionBuilder::new("fishstickmc", Loader::Vanilla, "", "1.7.10", &LAUNCHER_DIRECTORY);

    version.launch(&profile, JavaDistribution::Liberica)
        .with_event_bus(&event_bus)
        .run()
        .await?;

    Ok(())
}
