//! Events and Instance Management Example
//!
//! This example demonstrates:
//! - Event system for progress tracking
//! - Instance management (create, launch, monitor, delete)
//! - Real-time console output streaming
//! - Instance control (PID, close, delete)
//!
//! # Running this example
//!
//! ```bash
//! cargo run --example with_events
//! ```

use lighty_launcher::prelude::*;
use std::time::Duration;

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

    trace_info!("=== Events and Instance Management Example ===\n");

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
                    trace_info!("Authenticating with {}...", provider);
                }
                Event::Auth(AuthEvent::AuthenticationSuccess { username, .. }) => {
                    trace_info!("Authenticated as {}", username);
                }
                Event::Auth(AuthEvent::AuthenticationFailed { error, .. }) => {
                    trace_error!("Authentication failed: {}", error);
                }
                Event::Launch(LaunchEvent::IsInstalled { version }) => {
                    trace_info!("{} is already installed and up-to-date!", version);
                }
                Event::Launch(LaunchEvent::InstallStarted { total_bytes, .. }) => {
                    total = total_bytes;
                    trace_info!("Installing: {} MB total", total / 1_000_000);
                }
                Event::Launch(LaunchEvent::InstallProgress { bytes }) => {
                    downloaded += bytes;
                    let percent = (downloaded as f64 / total as f64) * 100.0;
                    print!("\rProgress: {:.1}%", percent);
                    std::io::Write::flush(&mut std::io::stdout()).ok();
                }
                Event::Launch(LaunchEvent::InstallCompleted { .. }) => {
                    trace_info!("\nInstallation completed!");
                }
                Event::Java(JavaEvent::JavaNotFound { distribution, version }) => {
                    trace_info!("[Java] {} {} not found, downloading...", distribution, version);
                }
                Event::Java(JavaEvent::JavaAlreadyInstalled { distribution, version, .. }) => {
                    trace_info!("[Java] {} {} already installed", distribution, version);
                }
                Event::Java(JavaEvent::JavaDownloadStarted { distribution, version, total_bytes }) => {
                    trace_info!("[Java] Downloading {} {} ({} MB)", distribution, version, total_bytes / 1_000_000);
                }
                Event::Java(JavaEvent::JavaDownloadProgress { bytes }) => {
                    print!("\r[Java] Download progress: {} MB", bytes / 1_000_000);
                    std::io::Write::flush(&mut std::io::stdout()).ok();
                }
                Event::Java(JavaEvent::JavaDownloadCompleted { distribution, version }) => {
                    trace_info!("\n[Java] {} {} download completed", distribution, version);
                }
                Event::Java(JavaEvent::JavaExtractionStarted { distribution, version }) => {
                    trace_info!("[Java] Extracting {} {}...", distribution, version);
                }
                Event::Java(JavaEvent::JavaExtractionCompleted { distribution, version, .. }) => {
                    trace_info!("[Java] {} {} extraction completed", distribution, version);
                }
                Event::Loader(LoaderEvent::FetchingData { loader, minecraft_version, loader_version }) => {
                    trace_info!("[Loader] Fetching {} data for Minecraft {} (loader version: {})", loader, minecraft_version, loader_version);
                }
                Event::Loader(LoaderEvent::DataFetched { loader, .. }) => {
                    trace_info!("[Loader] {} data fetched successfully", loader);
                }
                Event::Loader(LoaderEvent::ManifestCached { loader }) => {
                    trace_info!("[Loader] Using cached {} manifest", loader);
                }
                Event::Loader(LoaderEvent::MergingLoaderData { base_loader, overlay_loader }) => {
                    trace_info!("[Loader] Merging {} with {}", overlay_loader, base_loader);
                }
                Event::Loader(LoaderEvent::DataMerged { base_loader, overlay_loader }) => {
                    trace_info!("[Loader] {} and {} merged successfully", overlay_loader, base_loader);
                }
                Event::Core(CoreEvent::ExtractionStarted { archive_type, file_count, .. }) => {
                    if file_count > 0 {
                        trace_info!("[Core] Extracting {} archive ({} files)...", archive_type, file_count);
                    } else {
                        trace_info!("[Core] Extracting {} archive...", archive_type);
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
                    trace_info!("\n[Core] {} extraction completed ({} files)", archive_type, files_extracted);
                }
                Event::InstanceLaunched(e) => {
                    trace_info!("\n[EVENT] Instance '{}' launched", e.instance_name);
                    trace_info!("PID: {}", e.pid);
                    trace_info!("Version: {}", e.version);
                    trace_info!("Player: {}", e.username);
                }
                Event::ConsoleOutput(e) => {
                    // Stream console output in real-time
                    let prefix = match e.stream {
                        ConsoleStream::Stdout => "[GAME]",
                        ConsoleStream::Stderr => "[ERR ]",
                    };
                    print!("{} {}", prefix, e.line);
                }
                Event::InstanceExited(e) => {
                    trace_info!(
                        "\n[EVENT] Instance '{}' exited with code: {:?}",
                        e.instance_name, e.exit_code
                    );
                }
                Event::InstanceDeleted(e) => {
                    trace_info!("\n[EVENT] Instance '{}' deleted", e.instance_name);
                }
                _ => {}
            }
        }
    });

    // Authenticate
    trace_info!("Step 1: Authenticating...");
    let mut auth = OfflineAuth::new("Player");
    let profile = auth.authenticate(Some(&event_bus)).await?;
    trace_info!("Authenticated as: {}\n", profile.username);

    // Create instance
    trace_info!("Step 2: Creating Vanilla instance...");
    let mut instance = VersionBuilder::new("demo-instance-28", Loader::Vanilla, "", "1.16", launcher_dir);
    trace_info!("Instance created\n");

    // Calculate instance size
    trace_info!("Step 3: Calculating instance size...");
    let metadata = instance.get_metadata().await?;

    // Extract Version from VersionMetaData
    use lighty_launcher::loaders::VersionMetaData;
    let version = match metadata.as_ref() {
        VersionMetaData::Version(v) => v,
        _ => anyhow::bail!("Expected Version metadata"),
    };

    let size = instance.size_of_instance(&version);
    trace_info!("Libraries: {}", InstanceSize::format(size.libraries));
    trace_info!("Client: {}", InstanceSize::format(size.client));
    trace_info!("Assets: {}", InstanceSize::format(size.assets));
    trace_info!("Total: {} ({:.2} GB)\n", InstanceSize::format(size.total), size.total_gb());

    // Launch with event bus
    trace_info!("Step 4: Launching instance...");
    instance.launch(&profile, JavaDistribution::Temurin)
        .with_event_bus(&event_bus)
        .with_jvm_options()
            .set("Xmx", "2G")
            .set("Xms", "1G")
            .done()
        .with_arguments()
            .set("width", "1280")
            .set("height", "720")
            .done()
        .run()
        .await?;

    // Wait for instance to start
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Check running instances
    trace_info!("\nStep 5: Checking running instances...");
    if let Some(pid) = instance.get_pid() {
        trace_info!("Instance is running with PID: {}", pid);
        let all_pids = instance.get_pids();
        trace_info!("All PIDs for this instance: {:?}", all_pids);
    } else {
        trace_warn!("No running instance found");
        return Ok(());
    }

    // Let it run for a while - wait long enough to see the game window
    trace_info!("\nStep 6: Instance running... (waiting 60 seconds)");
    trace_info!("Console output is being streamed above");
    tokio::time::sleep(Duration::from_secs(60)).await;

    // Delete the instance
    // trace_info!("Step 8: Deleting instance...");
    // let running_pids = instance.get_pids();
    // if !running_pids.is_empty() {
    //     trace_warn!("Warning: Instance still running, closing all...");
    //     for pid in running_pids {
    //         instance.close_instance(pid).await?;
    //     }
    //     tokio::time::sleep(Duration::from_secs(2)).await;
    // }

    // instance.delete_instance().await?;
    // trace_info!("Instance deleted from disk");
    trace_info!("Example completed successfully (instance kept for debugging)");

    Ok(())
}
