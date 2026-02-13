use lighty_launcher::prelude::*;
use lighty_launcher::event::{EventBus, Event};

const QUALIFIER: &str = "com";
const ORGANIZATION: &str = ".LightyLauncher";
const APPLICATION: &str = "";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .compact()
        .init();

    let _app_state = AppState::new(
        QUALIFIER.to_string(),
        ORGANIZATION.to_string(),
        APPLICATION.to_string(),
    )?;

    let launcher_dir = AppState::get_project_dirs();

    println!("\n=== Forge Debug Example ===");
    println!("Minecraft 1.20.1 with Forge 47.2.0");
    println!("This will show all arguments and libraries being loaded\n");

    // Authenticate (offline mode for demo)
    let mut auth = OfflineAuth::new("DebugPlayer");
    let profile = auth.authenticate(None).await?;

    // Create Forge instance
    let mut forge_instance = VersionBuilder::new(
        "forge-debug",
        Loader::Forge,
        "47.2.0",          // Forge version
        "1.20.1",          // Minecraft version
        launcher_dir
    );

    // Create event bus to capture all events
    let event_bus = EventBus::new(1000);
    
    // Spawn event listener
    let bus_clone = event_bus.clone();
    tokio::spawn(async move {
        let mut receiver = bus_clone.subscribe();
        while let Some(event) = receiver.recv().await {
            match event {
                Event::ConsoleOutput(e) => {
                    println!("[{}] {}: {}", e.instance_name, e.stream, e.line);
                }
                Event::InstanceExited(e) => {
                    println!("\n=== GAME EXITED ===");
                    println!("Exit code: {:?}", e.exit_code);
                    println!("Instance: {}", e.instance_name);
                    
                    // Read logs if available
                    let game_dir = std::path::PathBuf::from(format!(
                        "{}/AppData/Roaming/.LightyLauncher/forge-debug",
                        std::env::var("USERPROFILE").unwrap_or_default()
                    ));
                    let log_file = game_dir.join("logs/latest.log");
                    if log_file.exists() {
                        println!("\n=== MINECRAFT LOG ===");
                        if let Ok(content) = std::fs::read_to_string(&log_file) {
                            println!("{}", content);
                        }
                    } else {
                        println!("No log file found at: {}", log_file.display());
                    }
                }
                _ => {}
            }
        }
    });

    // Launch with event bus
    let result = forge_instance
        .launch(&profile, JavaDistribution::Temurin)
        .with_event_bus(&event_bus)
        .run()
        .await;

    match result {
        Ok(_) => println!("\n✓ Launch completed successfully"),
        Err(e) => {
            println!("\n✗ Launch failed: {}", e);
            println!("\nError details: {:?}", e);
        }
    }

    // Wait a bit for events to process
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    Ok(())
}
