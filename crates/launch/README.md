# lighty-launch

Minecraft launch logic for [LightyLauncher](https://crates.io/crates/lighty-launcher).

## Note

This is an internal crate for the LightyLauncher ecosystem. Most users should use the main [`lighty-launcher`](https://crates.io/crates/lighty-launcher) crate instead.

## Features

- **Game Launching**: Launch Minecraft with proper arguments
- **Asset Installation**: Download and install game assets and libraries
- **JVM Arguments**: Generate optimized JVM arguments
- **Process Management**: Manage Minecraft process lifecycle
- **Instance Control**: Track, monitor, and control running instances
- **Console Streaming**: Real-time console output via events
- **Instance Size Calculation**: Calculate disk space usage

## Usage

```toml
[dependencies]
lighty-launch = "0.6.3"
```

```rust
use lighty_launch::launch::Launch;
use lighty_java::JavaDistribution;

#[tokio::main]
async fn main() {
    // Assuming you have a Version object from lighty-loaders
    let mut version = /* ... */;

    // Launch the game
    version.launch(
        "PlayerName",
        "player-uuid",
        JavaDistribution::Temurin
    ).await?;
}
```

## Structure

```
lighty-launch/
└── src/
    ├── lib.rs          # Module declarations
    ├── launch/         # Launch system
    │   ├── mod.rs
    │   ├── builder.rs  # LaunchBuilder
    │   └── runner.rs   # Game execution
    ├── installer/      # Assets and libraries installation
    │   ├── mod.rs      # Installer trait
    │   ├── assets.rs   # Assets installation
    │   └── libraries.rs # Libraries installation
    ├── instance/       # Instance management (NEW)
    │   ├── mod.rs
    │   ├── manager.rs  # InstanceManager
    │   ├── utilities.rs # InstanceControl trait
    │   ├── console.rs  # Console streaming
    │   └── errors.rs   # InstanceError types
    ├── arguments.rs    # JVM and game arguments generation
    └── errors.rs       # Error types (InstallerError, InstallerResult)
```

## Components

### Launch Trait

The `Launch` trait defines the interface for launching Minecraft:

```rust
use async_trait::async_trait;
use lighty_java::JavaDistribution;
use lighty_launch::errors::InstallerResult;

#[async_trait]
pub trait Launch {
    async fn launch(
        &mut self,
        username: &str,
        uuid: &str,
        java_distribution: JavaDistribution
    ) -> InstallerResult<()>;
}
```

### Installer Trait

Handles downloading and installing game files (assets, libraries):

```rust
use lighty_launch::installer::Installer;

#[async_trait]
pub trait Installer {
    async fn install(&self, builder: &VersionBuilder) -> InstallerResult<()>;
}
```

### Arguments

Generates optimized JVM and game arguments:

```rust
use lighty_launch::arguments::Arguments;

let args = Arguments::new(&version_metadata, &java_path);
let jvm_args = args.get_jvm_arguments();
let game_args = args.get_game_arguments("PlayerName", "uuid");
```

**Features**:
- Memory optimization based on system RAM
- Platform-specific arguments
- Library path resolution
- Asset index handling

### Instance Control (NEW)

The `InstanceControl` trait provides instance management capabilities for any type implementing `VersionInfo`:

```rust
use lighty_launch::InstanceControl;  // Must import the trait
use lighty_version::VersionBuilder;
use lighty_loaders::types::Loader;

let minozia = VersionBuilder::new(
    "minozia",
    Loader::Fabric,
    "0.15.0",
    "1.20.1",
    &launcher_dir,
);

// Launch the game
minozia.launch(&profile, JavaDistribution::Temurin)
    .with_jvm_options()
        .set("Xmx", "4G")
        .done()
    .run()
    .await?;

// Get running instance PID
if let Some(pid) = minozia.get_pid() {
    println!("Instance running with PID: {}", pid);

    // Close the instance
    minozia.close_instance(pid).await?;
}

// Get all PIDs (if multiple instances running)
let pids = minozia.get_pids();
for pid in pids {
    println!("Running: {}", pid);
}

// Calculate instance size
let version = minozia.get_metadata().await?;
let size = minozia.size_of_instance(&version);
println!("Total: {}", InstanceSize::format(size.total));
println!("Libraries: {}", InstanceSize::format(size.libraries));
println!("Mods: {}", InstanceSize::format(size.mods));

// Delete instance (only if not running)
minozia.delete_instance().await?;
```

**Available Methods:**
- `get_pid() -> Option<u32>` - Get first PID of running instance
- `get_pids() -> Vec<u32>` - Get all PIDs
- `close_instance(pid) -> Result<()>` - Kill an instance
- `delete_instance() -> Result<()>` - Delete instance from disk
- `size_of_instance(&Version) -> InstanceSize` - Calculate disk usage

### Console Streaming

When the `events` feature is enabled, console output is automatically streamed:

```rust
use lighty_event::{Event, EVENT_BUS};

#[tokio::main]
async fn main() {
    let mut receiver = EVENT_BUS.subscribe();

    tokio::spawn(async move {
        while let Ok(event) = receiver.next().await {
            match event {
                Event::InstanceLaunched(e) => {
                    println!("Instance {} launched with PID {}", e.instance_name, e.pid);
                }
                Event::ConsoleOutput(e) => {
                    print!("[PID {}] {}", e.pid, e.line);
                }
                Event::InstanceExited(e) => {
                    println!("Instance exited with code: {:?}", e.exit_code);
                }
                Event::InstanceDeleted(e) => {
                    println!("Instance {} deleted", e.instance_name);
                }
                _ => {}
            }
        }
    });

    // Launch game...
}
```

**Available Events:**
- `InstanceLaunched` - Emitted when instance starts
- `ConsoleOutput` - Emitted for each stdout/stderr line
- `InstanceExited` - Emitted when instance exits
- `InstanceDeleted` - Emitted when instance is deleted

## Error Handling

All operations return `InstallerResult<T>` with detailed error information:

```rust
use lighty_launch::errors::{InstallerError, InstallerResult};

match version.launch(username, uuid, java_dist).await {
    Ok(()) => println!("Game launched successfully"),
    Err(InstallerError::DownloadFailed(url)) => {
        eprintln!("Failed to download: {}", url);
    }
    Err(e) => eprintln!("Launch error: {:?}", e),
}
```

## License

MIT

## Links

- **Main Package**: [lighty-launcher](https://crates.io/crates/lighty-launcher)
- **Repository**: [GitHub](https://github.com/Lighty-Launcher/LightyLauncherLib)
- **Documentation**: [docs.rs/lighty-launch](https://docs.rs/lighty-launch)
