use lighty_launcher::prelude::*;
use std::time::Duration;

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

    // Event bus to capture the game's stdout / stderr / exit so we can
    // actually see whether the JVM crashes on boot.
    let event_bus = EventBus::new(2000);
    let (exit_tx, exit_rx) = tokio::sync::oneshot::channel::<Option<i32>>();
    let mut receiver = event_bus.subscribe();

    tokio::spawn(async move {
        let mut exit_tx = Some(exit_tx);
        while let Ok(event) = receiver.next().await {
            match event {
                Event::ConsoleOutput(e) => {
                    let prefix = match e.stream {
                        ConsoleStream::Stdout => "[GAME]",
                        ConsoleStream::Stderr => "[ERR ]",
                    };
                    print!("{} {}", prefix, e.line);
                    if !e.line.ends_with('\n') {
                        println!();
                    }
                }
                Event::InstanceExited(e) => {
                    println!("\n⚠ Instance exited with code: {:?}", e.exit_code);
                    if let Some(tx) = exit_tx.take() {
                        let _ = tx.send(e.exit_code);
                    }
                    break;
                }
                _ => {}
            }
        }
    });

    // Authenticate (offline)
    let mut auth = OfflineAuth::new("Hamadi");
    let profile = auth.authenticate(Some(&event_bus)).await?;

    // Build and launch a legacy Forge instance (1.7.10 + Forge 10.13.4.1614).
    // The launcher auto-detects the legacy installer format from the MC version.
    let mut forge = VersionBuilder::new(
        "forge-legacy-test",
        Loader::Forge,
        "10.13.4.1614",
        "1.7.10",
        launcher_dir,
    );

    forge
        .launch(&profile, JavaDistribution::Temurin)
        .with_event_bus(&event_bus)
        .run()
        .await?;

    trace_info!("Forge legacy launch successful! Waiting for game output...");

    // Wait up to 60s for the JVM to either print something useful or exit.
    let _ = tokio::time::timeout(Duration::from_secs(60), exit_rx).await;

    Ok(())
}
