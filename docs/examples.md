# Examples Documentation

Complete guide to all examples in the `examples/` directory.

## Overview

| Example | Loader | Features | Complexity |
|---------|--------|----------|------------|
| [vanilla.rs](#vanillars) | Vanilla | Basic launch | ⭐ Beginner |
| [fabric.rs](#fabricrs) | Fabric | Basic launch | ⭐ Beginner |
| [quilt.rs](#quiltrs) | Quilt | Basic launch | ⭐ Beginner |
| [neoforge.rs](#neoforgrs) | NeoForge | Basic launch | ⭐ Beginner |
| [forge.rs](#forgrs) | Forge | Basic launch | ⭐⭐ Intermediate |
| [forge_legacy.rs](#forge_legacyrs) | Forge Legacy | Basic launch | ⭐⭐ Intermediate |
| [optifine.rs](#optifinrs) | OptiFine | Basic launch | ⭐⭐ Intermediate |
| [lighty_updater.rs](#lighty_updaterrs) | LightyUpdater | Custom server | ⭐⭐ Intermediate |
| [with_events.rs](#with_eventsrs) | Vanilla | Events + Instance Control | ⭐⭐⭐ Advanced |

## Running Examples

### Basic Command

```bash
cargo run --example <example_name> --features <required_features>
```

### With Tracing (Debug Output)

```bash
cargo run --example <example_name> --features <required_features>,tracing
```

### Common Feature Combinations

```bash
# Vanilla with events
cargo run --example vanilla --features vanilla,events

# Fabric with events and tracing
cargo run --example fabric --features fabric,events,tracing

# Complete demo
cargo run --example with_events --features vanilla,events,tracing
```

---

## vanilla.rs

**Purpose**: Basic Vanilla Minecraft launcher with custom JVM options and game arguments

**Loader**: Vanilla
**Features Required**: `vanilla`

### What It Demonstrates

- ✅ AppState initialization
- ✅ Offline authentication
- ✅ VersionBuilder creation
- ✅ Custom JVM options (memory, etc.)
- ✅ Custom game arguments (resolution)
- ✅ Downloader configuration
- ✅ Basic launch flow

### Code Walkthrough

```rust
use lighty_launcher::prelude::*;

const QUALIFIER: &str = "com";
const ORGANIZATION: &str = ".LightyLauncher";
const APPLICATION: &str = "";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Initialize AppState
    let _app_state = AppState::new(
        QUALIFIER.to_string(),
        ORGANIZATION.to_string(),
        APPLICATION.to_string(),
    )?;

    let launcher_dir = AppState::get_project_dirs();

    // 2. Configure downloader (optional)
    init_downloader_config(DownloaderConfig {
        max_concurrent_downloads: 100,
        max_retries: 5,
        initial_delay_ms: 50,
    });

    // 3. Authenticate
    let mut auth = OfflineAuth::new("Hamadi");
    let profile = auth.authenticate().await?;

    // 4. Create instance
    let mut version = VersionBuilder::new(
        "vanilla-1.7.10",
        Loader::Vanilla,
        "",
        "1.7.10",
        launcher_dir
    );

    // 5. Launch with custom options
    version.launch(&profile, JavaDistribution::Temurin)
        .with_jvm_options()
            .set("Xmx", "4G")          // Max heap: 4GB
            .set("Xms", "2G")          // Initial heap: 2GB
            .done()
        .with_arguments()
            .set("width", "1920")      // Window width
            .set("height", "1080")     // Window height
            .done()
        .run()
        .await?;

    trace_info!("Launch successful!");
    Ok(())
}
```

### Key Concepts

1. **AppState**: Manages project directories (data, config, cache)
2. **DownloaderConfig**: Configures parallel downloads and retries
3. **JVM Options**: Memory allocation, garbage collection
4. **Game Arguments**: Window resolution, fullscreen, etc.

### Run It

```bash
cargo run --example vanilla --features vanilla
```

---

## fabric.rs

**Purpose**: Fabric mod loader launcher

**Loader**: Fabric
**Features Required**: `fabric`

### What It Demonstrates

- ✅ Fabric loader integration
- ✅ Loader version specification
- ✅ Metadata merging (Vanilla + Fabric)

### Code Highlights

```rust
// Fabric instance with loader version
let mut fabric = VersionBuilder::new(
    "fabric",
    Loader::Fabric,
    "0.17.2",      // Fabric loader version
    "1.21.1",      // Minecraft version
    launcher_dir
);

fabric.launch(&profile, JavaDistribution::Temurin)
    .run()
    .await?;
```

### How It Works

1. Fetches Vanilla 1.21.1 metadata
2. Fetches Fabric 0.17.2 loader data
3. Merges metadata (adds Fabric libraries, updates main class)
4. Launches with merged metadata

### Run It

```bash
cargo run --example fabric --features fabric
```

---

## quilt.rs

**Purpose**: Quilt mod loader launcher

**Loader**: Quilt
**Features Required**: `quilt`

### What It Demonstrates

- ✅ Quilt loader integration
- ✅ Alternative to Fabric

### Code Highlights

```rust
let mut quilt = VersionBuilder::new(
    "quilt",
    Loader::Quilt,
    "0.27.1",      // Quilt loader version
    "1.21.1",      // Minecraft version
    launcher_dir
);
```

### Run It

```bash
cargo run --example quilt --features quilt
```

---

## neoforge.rs

**Purpose**: NeoForge mod loader launcher

**Loader**: NeoForge
**Features Required**: `neoforge`

### What It Demonstrates

- ✅ NeoForge loader (modern Forge fork)
- ✅ Latest Minecraft versions

### Code Highlights

```rust
let mut neoforge = VersionBuilder::new(
    "neoforge",
    Loader::NeoForge,
    "21.1.80",     // NeoForge version
    "1.21.1",      // Minecraft version
    launcher_dir
);
```

### Run It

```bash
cargo run --example neoforge --features neoforge
```

---

## forge.rs

**Purpose**: Forge mod loader launcher

**Loader**: Forge
**Features Required**: `forge`

### What It Demonstrates

- ✅ Forge loader (modern versions)
- ✅ Complex metadata merging

### Code Highlights

```rust
let mut forge = VersionBuilder::new(
    "forge",
    Loader::Forge,
    "47.3.0",      // Forge version
    "1.20.1",      // Minecraft version
    launcher_dir
);
```

### Run It

```bash
cargo run --example forge --features forge
```

---

## forge_legacy.rs

**Purpose**: Legacy Forge launcher (1.7.10 - 1.12.2)

**Loader**: Forge Legacy
**Features Required**: `forge_legacy`

### What It Demonstrates

- ✅ Legacy Forge versions
- ✅ Older Minecraft versions
- ✅ Different metadata format

### Code Highlights

```rust
let mut forge_legacy = VersionBuilder::new(
    "forge-legacy",
    Loader::ForgeLegacy,
    "10.13.4.1614",  // Forge 1.7.10 version
    "1.7.10",        // Minecraft version
    launcher_dir
);
```

### Run It

```bash
cargo run --example forge_legacy --features forge_legacy
```

---

## optifine.rs

**Purpose**: OptiFine launcher

**Loader**: OptiFine
**Features Required**: `optifine`

### What It Demonstrates

- ✅ OptiFine integration
- ✅ Performance optimization mod

### Code Highlights

```rust
let mut optifine = VersionBuilder::new(
    "optifine",
    Loader::Optifine,
    "HD_U_I6",     // OptiFine version
    "1.20.1",      // Minecraft version
    launcher_dir
);
```

### Run It

```bash
cargo run --example optifine --features optifine
```

---

## lighty_updater.rs

**Purpose**: Custom modpack server launcher

**Loader**: LightyUpdater
**Features Required**: `lighty_updater`

### What It Demonstrates

- ✅ Custom server integration
- ✅ LightyVersionBuilder
- ✅ Server-managed modpacks
- ✅ Automatic mod updates

### Code Highlights

```rust
use lighty_launcher::version::LightyVersionBuilder;

// Connect to custom server
let mut modpack = LightyVersionBuilder::new(
    "my-modpack",
    "https://myserver.com/api",  // Server URL
    launcher_dir
);

modpack.launch(&profile, JavaDistribution::Temurin)
    .run()
    .await?;
```

### How It Works

1. **GET {server_url}/version**
   ```json
   {
       "minecraft_version": "1.21.1",
       "loader": "Fabric",
       "loader_version": "0.17.2",
       "mods": [
           {
               "name": "fabric-api",
               "url": "https://server.com/mods/fabric-api.jar",
               "sha1": "abc123...",
               "enabled": true
           }
       ]
   }
   ```

2. Fetches Vanilla + Loader metadata
3. Adds server mods to metadata
4. Downloads and installs mods
5. Launches game

### Server Setup

See [LightyUpdater Repository](https://github.com/Lighty-Launcher/LightyUpdater) for server implementation.

### Run It

```bash
cargo run --example lighty_updater --features lighty_updater
```

---

## with_events.rs

**Purpose**: Complete demonstration of event system and instance management

**Loader**: Vanilla
**Features Required**: `vanilla`, `events`

### What It Demonstrates

- ✅ Event bus creation and subscription
- ✅ All event types (Auth, Java, Launch, Loader, Core, Instance)
- ✅ Real-time progress tracking
- ✅ Console output streaming
- ✅ Instance lifecycle management
- ✅ Instance size calculation
- ✅ PID tracking
- ✅ Instance control (close, delete)

### Code Walkthrough

```rust
use lighty_launcher::prelude::*;
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Initialize AppState
    let _app_state = AppState::new(/*...*/)?;
    let launcher_dir = AppState::get_project_dirs();

    // 2. Create event bus
    let event_bus = EventBus::new(1000);

    // 3. Spawn event listener
    let mut receiver = event_bus.subscribe();
    tokio::spawn(async move {
        while let Ok(event) = receiver.next().await {
            match event {
                Event::Auth(AuthEvent::AuthenticationSuccess { username, .. }) => {
                    trace_info!("Authenticated as {}", username);
                }
                Event::Launch(LaunchEvent::InstallStarted { total_bytes, .. }) => {
                    trace_info!("Installing: {} MB total", total_bytes / 1_000_000);
                }
                Event::Launch(LaunchEvent::InstallProgress { bytes }) => {
                    // Track download progress
                }
                Event::Java(JavaEvent::JavaDownloadStarted { distribution, version, total_bytes }) => {
                    trace_info!("[Java] Downloading {} {} ({} MB)", distribution, version, total_bytes / 1_000_000);
                }
                Event::InstanceLaunched(e) => {
                    trace_info!("\n[EVENT] Instance '{}' launched", e.instance_name);
                    trace_info!("PID: {}", e.pid);
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
                    trace_info!("\n[EVENT] Instance exited with code: {:?}", e.exit_code);
                }
                _ => {}
            }
        }
    });

    // 4. Authenticate with events
    let mut auth = OfflineAuth::new("Player");
    let profile = auth.authenticate(Some(&event_bus)).await?;

    // 5. Create instance
    let mut instance = VersionBuilder::new(
        "demo-instance",
        Loader::Vanilla,
        "",
        "1.21.1",
        launcher_dir
    );

    // 6. Calculate instance size
    let metadata = instance.get_metadata().await?;
    let version = match metadata.as_ref() {
        VersionMetaData::Version(v) => v,
        _ => anyhow::bail!("Expected Version metadata"),
    };

    let size = instance.size_of_instance(&version);
    trace_info!("Libraries: {}", InstanceSize::format(size.libraries));
    trace_info!("Client: {}", InstanceSize::format(size.client));
    trace_info!("Assets: {}", InstanceSize::format(size.assets));
    trace_info!("Total: {:.2} GB", size.total_gb());

    // 7. Launch with event bus
    instance.launch(&profile, JavaDistribution::Liberica)
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

    // 8. Wait for instance to start
    tokio::time::sleep(Duration::from_secs(2)).await;

    // 9. Check running instances
    if let Some(pid) = instance.get_pid() {
        trace_info!("Instance is running with PID: {}", pid);
        let all_pids = instance.get_pids();
        trace_info!("All PIDs for this instance: {:?}", all_pids);
    }

    // 10. Let it run for a while
    tokio::time::sleep(Duration::from_secs(30)).await;

    // 11. Close the instance
    if let Some(pid) = instance.get_pid() {
        instance.close_instance(pid).await?;
        trace_info!("Instance closed successfully");
    }

    // 12. Delete the instance
    tokio::time::sleep(Duration::from_secs(2)).await;
    instance.delete_instance().await?;
    trace_info!("Instance deleted from disk");

    Ok(())
}
```

### Event Types Demonstrated

#### 1. Authentication Events

```rust
Event::Auth(AuthEvent::AuthenticationStarted { provider })
Event::Auth(AuthEvent::AuthenticationSuccess { username, uuid })
Event::Auth(AuthEvent::AuthenticationFailed { error, provider })
```

#### 2. Java Events

```rust
Event::Java(JavaEvent::JavaNotFound { distribution, version })
Event::Java(JavaEvent::JavaAlreadyInstalled { distribution, version, binary_path })
Event::Java(JavaEvent::JavaDownloadStarted { distribution, version, total_bytes })
Event::Java(JavaEvent::JavaDownloadProgress { bytes })
Event::Java(JavaEvent::JavaDownloadCompleted { distribution, version })
Event::Java(JavaEvent::JavaExtractionStarted { distribution, version })
Event::Java(JavaEvent::JavaExtractionCompleted { distribution, version, binary_path })
```

#### 3. Launch Events

```rust
Event::Launch(LaunchEvent::IsInstalled { version })
Event::Launch(LaunchEvent::InstallStarted { version, total_bytes })
Event::Launch(LaunchEvent::InstallProgress { bytes })
Event::Launch(LaunchEvent::DownloadingLibraries { current, total })
Event::Launch(LaunchEvent::DownloadingNatives { current, total })
Event::Launch(LaunchEvent::DownloadingClient { version })
Event::Launch(LaunchEvent::DownloadingAssets { current, total })
Event::Launch(LaunchEvent::DownloadingMods { current, total })
Event::Launch(LaunchEvent::InstallCompleted { version, total_bytes })
```

#### 4. Loader Events

```rust
Event::Loader(LoaderEvent::FetchingData { loader, minecraft_version, loader_version })
Event::Loader(LoaderEvent::DataFetched { loader, minecraft_version, loader_version })
Event::Loader(LoaderEvent::ManifestCached { loader })
Event::Loader(LoaderEvent::MergingLoaderData { base_loader, overlay_loader })
Event::Loader(LoaderEvent::DataMerged { base_loader, overlay_loader })
```

#### 5. Core Events

```rust
Event::Core(CoreEvent::ExtractionStarted { archive_type, source, destination, file_count })
Event::Core(CoreEvent::ExtractionProgress { files_extracted, total_files })
Event::Core(CoreEvent::ExtractionCompleted { archive_type, files_extracted })
```

#### 6. Instance Events

```rust
Event::InstanceLaunched(InstanceLaunchedEvent {
    pid,
    instance_name,
    version,
    username,
    timestamp
})

Event::ConsoleOutput(ConsoleOutputEvent {
    pid,
    instance_name,
    stream,  // Stdout or Stderr
    line,
    timestamp
})

Event::InstanceExited(InstanceExitedEvent {
    pid,
    instance_name,
    exit_code,
    timestamp
})

Event::InstanceDeleted(InstanceDeletedEvent {
    instance_name,
    timestamp
})
```

### Instance Control Operations

```rust
// Get PID
let pid = instance.get_pid();

// Get all PIDs (supports multiple instances)
let pids = instance.get_pids();

// Close instance
instance.close_instance(pid).await?;

// Delete instance (must not be running)
instance.delete_instance().await?;

// Calculate size
let size = instance.size_of_instance(&version);
println!("Total: {:.2} GB", size.total_gb());
```

### Run It

```bash
cargo run --example with_events --features vanilla,events,tracing
```

**Expected Output**:

```
=== Events and Instance Management Example ===

Step 1: Authenticating...
Authenticated as: Player

Step 2: Creating Vanilla instance...
Instance created

Step 3: Calculating instance size...
Libraries: 52.3 MB
Client: 24.2 MB
Assets: 284.7 MB
Total: 361.2 MB (0.35 GB)

Step 4: Launching instance...
[Loader] Fetching Vanilla data for Minecraft 1.21.1
[Loader] Vanilla data fetched successfully
[Java] Temurin 21 already installed
1.21.1 is already installed and up-to-date!

[EVENT] Instance 'demo-instance' launched
PID: 12345
Version: 1.21.1-
Player: Player

[GAME] [14:52:31] [Render thread/INFO]: Setting user: Player
[GAME] [14:52:31] [Render thread/INFO]: Backend library: LWJGL version 3.3.3
[GAME] [14:52:32] [Render thread/INFO]: Reloading ResourceManager...

Step 5: Checking running instances...
Instance is running with PID: 12345
All PIDs for this instance: [12345]

Step 6: Instance running... (waiting 30 seconds)
Console output is being streamed above

Step 7: Closing instance...
Instance closed successfully

Step 8: Deleting instance...
Instance deleted from disk
Example completed successfully
```

---

## Common Patterns

### Pattern 1: Basic Launch

```rust
use lighty_launcher::prelude::*;

let _app = AppState::new(/*...*/)?;
let launcher_dir = AppState::get_project_dirs();

let mut auth = OfflineAuth::new("Player");
let profile = auth.authenticate().await?;

let mut instance = VersionBuilder::new(
    "my-instance",
    Loader::Vanilla,
    "",
    "1.21.1",
    launcher_dir
);

instance.launch(&profile, JavaDistribution::Temurin)
    .run()
    .await?;
```

### Pattern 2: With Custom Options

```rust
instance.launch(&profile, JavaDistribution::Temurin)
    .with_jvm_options()
        .set("Xmx", "4G")
        .set("Xms", "2G")
        .done()
    .with_arguments()
        .set("width", "1920")
        .set("height", "1080")
        .done()
    .run()
    .await?;
```

### Pattern 3: With Events

```rust
let event_bus = EventBus::new(1000);

// Spawn listener
let mut receiver = event_bus.subscribe();
tokio::spawn(async move {
    while let Ok(event) = receiver.next().await {
        // Handle events
    }
});

// Launch with events
instance.launch(&profile, JavaDistribution::Temurin)
    .with_event_bus(&event_bus)
    .run()
    .await?;
```

### Pattern 4: Instance Management

```rust
use lighty_launch::InstanceControl;  // Must import!

// Launch
instance.launch(&profile, JavaDistribution::Temurin).run().await?;

// Get PID
if let Some(pid) = instance.get_pid() {
    println!("Running: {}", pid);

    // Close
    instance.close_instance(pid).await?;
}

// Delete
instance.delete_instance().await?;
```

## Troubleshooting

### Error: "Failed to fetch metadata"

**Solution**: Check internet connection and loader availability

### Error: "Java not found"

**Solution**: Ensure Java distribution is supported for the Minecraft version

### Error: "Instance is running"

**Solution**: Close instance before deleting:

```rust
for pid in instance.get_pids() {
    instance.close_instance(pid).await?;
}
instance.delete_instance().await?;
```

## Related Documentation

- [Sequence Diagrams](./sequence-diagrams.md) - Visual flow diagrams
- [Re-exports](./reexports.md) - API reference
- [Architecture](./architecture.md) - System architecture
- [Launch Process](../crates/launch/docs/launch.md) - Detailed launch flow
- [Events](../crates/launch/docs/events.md) - Event types reference
- [Instance Control](../crates/launch/docs/instance-control.md) - Process management
