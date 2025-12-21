# LightyLauncher

[![Crates.io](https://img.shields.io/crates/v/lighty-launcher.svg)](https://crates.io/crates/lighty-launcher)
[![Documentation](https://docs.rs/lighty-launcher/badge.svg)](https://docs.rs/lighty-launcher)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust Version](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)

> **ACTIVE DEVELOPMENT** - API may change between versions. Use with caution in production.

A modern, modular Minecraft launcher library for Rust with full async support, real-time event system, and automatic Java management.

![LightyUpdater Banner](docs/img/banner.png)

## Features

- **Modular Architecture**: Organized into logical namespaces (`auth`, `event`, `java`, `launch`, `loaders`, `version`, `core`)
- **Multi-Loader Support**: Vanilla, Fabric, Quilt, NeoForge, Forge, OptiFine, LightyUpdater
- **Event System**: Real-time progress tracking for all operations (downloads, installations, authentication)
- **Authentication**: Offline, Microsoft OAuth 2.0, Azuriom CMS + trait-based extensibility for custom providers
- **Automatic Java Management**: Download and manage JRE distributions (Temurin, GraalVM, Zulu, Liberica)
- **Async/Await**: Built on Tokio for maximum performance
- **Smart Caching**: Dual cache (raw + query) with configurable TTL
- **Type-Safe**: Strongly typed API with comprehensive error handling
- **Cross-Platform**: Windows, Linux, and macOS support
- **Performance Optimized**: Parallel downloads, async I/O, minimal dependencies

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
lighty-launcher = { version = "0.6", features = ["vanilla", "events"] }
tokio = { version = "1", features = ["full"] }
directories = "6.0"
once_cell = "1.21"
tracing-subscriber = "0.3"
anyhow = "1.0"
```

## Quick Start

### Basic Example - Vanilla Minecraft

```rust
use lighty_launcher::{
    auth::{OfflineAuth, Authenticator},
    java::JavaDistribution,
    launch::Launch,
    loaders::Loader,
    version::VersionBuilder,
};
use directories::ProjectDirs;
use once_cell::sync::Lazy;

static LAUNCHER_DIR: Lazy<ProjectDirs> = Lazy::new(|| {
    ProjectDirs::from("com", "MyLauncher", "")
        .expect("Failed to create project directories")
});

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // Authenticate
    let mut auth = OfflineAuth::new("PlayerName");
    let profile = auth.authenticate().await?;

    // Create version instance
    let mut version = VersionBuilder::new(
        "vanilla-1.21.1",
        Loader::Vanilla,
        "",
        "1.21.1",
        &LAUNCHER_DIR
    );

    // Launch the game
    version.launch(&profile, JavaDistribution::Temurin)
        .run()
        .await?;

    Ok(())
}
```

### Using the Prelude

For convenience, import commonly used types:

```rust
use lighty_launcher::prelude::*;
use directories::ProjectDirs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let launcher_dir = ProjectDirs::from("com", "MyLauncher", "").unwrap();

    let mut auth = OfflineAuth::new("Player");
    let profile = auth.authenticate().await?;

    let mut version = VersionBuilder::new(
        "my-instance",
        Loader::Vanilla,
        "",
        "1.21.1",
        &launcher_dir
    );

    version.launch(&profile, JavaDistribution::Temurin)
        .run()
        .await?;

    Ok(())
}
```

## Modules

LightyLauncher is organized into logical modules, each with its own namespace:

### `lighty_launcher::auth` - Authentication

Multiple authentication methods with a unified, extensible interface:

```rust
use lighty_launcher::auth::{OfflineAuth, MicrosoftAuth, AzuriomAuth, Authenticator};

// Offline (no network required)
let mut auth = OfflineAuth::new("Player");
let profile = auth.authenticate().await?;

// Microsoft OAuth 2.0
let mut auth = MicrosoftAuth::new();
let profile = auth.authenticate().await?;

// Azuriom CMS
let mut auth = AzuriomAuth::new("https://example.com");
let profile = auth.authenticate().await?;
```

**Custom Authentication:**

Implement the `Authenticator` trait to create your own provider:

```rust
use lighty_launcher::auth::{Authenticator, UserProfile, UserRole, AuthResult};

pub struct MyCustomAuth {
    api_url: String,
}

impl Authenticator for MyCustomAuth {
    async fn authenticate(
        &mut self,
        #[cfg(feature = "events")] event_bus: Option<&EventBus>,
    ) -> AuthResult<UserProfile> {
        // Your custom logic here
        Ok(UserProfile {
            username: "Player".to_string(),
            uuid: "uuid-here".to_string(),
            access_token: Some("token".to_string()),
            role: UserRole::User,
        })
    }
}
```

**Key Types:**
- `OfflineAuth` - Offline authentication (UUID v5 generation)
- `MicrosoftAuth` - Microsoft/Xbox Live OAuth 2.0
- `AzuriomAuth` - Azuriom CMS integration
- `Authenticator` - Trait for creating custom authentication providers
- `UserProfile` - User data (username, UUID, access token)
- `generate_offline_uuid()` - Helper to create deterministic UUIDs

### `lighty_launcher::event` - Event System

Real-time progress tracking for all launcher operations:

```rust
use lighty_launcher::event::{EventBus, Event, LaunchEvent, AuthEvent, JavaEvent};

// Create event bus
let event_bus = EventBus::new(1000);
let mut receiver = event_bus.subscribe();

// Listen to events
tokio::spawn(async move {
    while let Ok(event) = receiver.next().await {
        match event {
            Event::Launch(LaunchEvent::InstallProgress { bytes }) => {
                println!("Downloaded {} bytes", bytes);
            }
            Event::Java(JavaEvent::JavaDownloadStarted { distribution, version, total_bytes }) => {
                println!("Downloading {} {} ({} MB)", distribution, version, total_bytes / 1_000_000);
            }
            _ => {}
        }
    }
});

// Use with authentication
let profile = auth.authenticate(Some(&event_bus)).await?;

// Use with launch
version.launch(&profile, JavaDistribution::Temurin)
    .with_event_bus(&event_bus)
    .run()
    .await?;
```

**Event Types:**
- `AuthEvent` - Authentication progress
- `JavaEvent` - JRE download/extraction
- `LaunchEvent` - Installation/launch progress
- `LoaderEvent` - Loader metadata fetching
- `CoreEvent` - Archive extraction

See [crates/event/README.md](crates/event/README.md) for complete documentation.

### `lighty_launcher::java` - Java Management

Automatic Java runtime download and installation:

```rust
use lighty_launcher::java::{JavaDistribution, JavaRuntime};

// Distributions are automatically managed
JavaDistribution::Temurin   // Recommended, supports all Java versions
JavaDistribution::GraalVM    // High performance, Java 17+ only
JavaDistribution::Zulu       // Enterprise support available
JavaDistribution::Liberica   // Lightweight alternative
```

**Supported Distributions:**

| Distribution | Java Versions | Type | Size (Java 21) | Best For |
|--------------|---------------|------|----------------|----------|
| Temurin | 8, 11, 17, 21+ | JRE | ~42 MB | General use, best compatibility |
| GraalVM | 17+ only | JDK | ~303 MB | Maximum performance |
| Zulu | 8, 11, 17, 21+ | JRE | ~82 MB | Enterprise support |
| Liberica | 8, 11, 17, 21+ | JRE | ~50 MB | Lightweight |

### `lighty_launcher::launch` - Game Launching

Complete launch orchestration with customization options. The launch system handles the entire process from metadata fetching to process execution.

#### Launch Flow

```
1. Prepare Metadata → 2. Install Java → 3. Install Dependencies → 4. Build Arguments → 5. Execute Game
```

#### Basic Launch

```rust
use lighty_launcher::launch::Launch;

version.launch(&profile, JavaDistribution::Temurin)
    .run()
    .await?;
```

#### Custom JVM Options

Configure memory, garbage collection, and system properties:

```rust
version.launch(&profile, JavaDistribution::Temurin)
    .with_jvm_options()
        .set("Xmx", "4G")                      // Maximum heap
        .set("Xms", "2G")                      // Initial heap
        .set("XX:+UseG1GC", "")                // G1 garbage collector
        .set("XX:MaxGCPauseMillis", "50")      // Max GC pause
        .set("Dfile.encoding", "UTF-8")        // File encoding
        .done()
    .run()
    .await?;
```

#### Custom Game Arguments

Override window settings, placeholders, and game options:

```rust
version.launch(&profile, JavaDistribution::Temurin)
    .with_arguments()
        .set("width", "1920")                  // Window width
        .set("height", "1080")                 // Window height
        .set("fullscreen", "true")             // Fullscreen mode
        .set("quickPlayMultiplayer", "mc.hypixel.net")  // Auto-connect server
        .done()
    .run()
    .await?;
```

#### Argument Placeholders

The launch system uses variable substitution for dynamic values:

**Authentication Placeholders:**
- `${auth_player_name}` - Player username
- `${auth_uuid}` - Player UUID
- `${auth_access_token}` - Access token or "0" (offline)
- `${auth_xuid}` - Xbox User ID
- `${user_type}` - User type ("legacy" or "msa")

**Directory Placeholders:**
- `${game_directory}` - Game instance directory
- `${assets_root}` - Assets root directory
- `${natives_directory}` - Native libraries directory
- `${classpath}` - Java classpath (all libraries)

**Version Placeholders:**
- `${version_name}` - Minecraft version (e.g., "1.21.1")
- `${launcher_name}` - Launcher brand name
- `${launcher_version}` - Launcher version
- `${assets_index_name}` - Asset index ID

See [crates/launch/docs/arguments.md](crates/launch/docs/arguments.md) for complete placeholder reference.

#### Installation Process

The launch system automatically downloads and verifies all game files:

**Phase 1: Verification** - SHA1 hash checking to skip already-downloaded files
**Phase 2: Parallel Download** - Libraries, natives, assets, client JAR, and mods downloaded concurrently
**Phase 3: Extraction** - Native libraries extracted to temporary directory

**Installation Components:**
- **Libraries** (~100-300 JARs, 50-100 MB) - Java dependencies
- **Natives** (~5-15 files, 5-10 MB) - Platform-specific binaries (LWJGL, OpenAL)
- **Assets** (~3000-10000 files, 200-500 MB) - Textures, sounds, language files
- **Client JAR** (1 file, 20-30 MB) - Main Minecraft executable
- **Mods** (10-200 files, variable size) - Fabric/Quilt/NeoForge modifications

See [crates/launch/docs/installation.md](crates/launch/docs/installation.md) for detailed installation process.

#### Instance Management

Track and control running game processes:

```rust
use lighty_launch::InstanceControl;  // Must import trait!

// Get process ID
if let Some(pid) = instance.get_pid() {
    println!("Running with PID: {}", pid);
}

// Get all PIDs (multiple processes)
let pids = instance.get_pids();

// Close instance
if let Some(pid) = instance.get_pid() {
    instance.close_instance(pid).await?;
}

// Delete instance completely (must not be running)
instance.delete_instance().await?;

// Calculate instance size
let metadata = instance.get_metadata().await?;
let size = instance.size_of_instance(&metadata);
println!("Total size: {:.2} GB", size.total_gb());
```

**Process lifecycle:**
1. Launch → Process spawned, PID registered
2. Running → Console output streamed via events
3. Exit → Process terminates, unregistered automatically
4. Manual close → Send kill signal (SIGTERM/TASKKILL)

See [crates/launch/docs/instance-control.md](crates/launch/docs/instance-control.md) for process management details.

#### Complete Example with Events

```rust
use lighty_launcher::prelude::*;
use lighty_launcher::event::{EventBus, Event, LaunchEvent};
use lighty_launch::InstanceControl;

let event_bus = EventBus::new(1000);
let mut receiver = event_bus.subscribe();

// Event listener
tokio::spawn(async move {
    while let Ok(event) = receiver.recv().await {
        match event {
            Event::Launch(LaunchEvent::DownloadingLibraries { current, total }) => {
                println!("Libraries: {}/{}", current, total);
            }
            Event::Launch(LaunchEvent::DownloadingAssets { current, total }) => {
                println!("Assets: {}/{}", current, total);
            }
            Event::InstanceLaunched(e) => {
                println!("Launched: {} (PID: {})", e.instance_name, e.pid);
            }
            Event::ConsoleOutput(e) => {
                println!("[{}] {}", e.pid, e.line);
            }
            Event::InstanceExited(e) => {
                println!("Exited: PID {} (code: {:?})", e.pid, e.exit_code);
            }
            _ => {}
        }
    }
});

// Launch with events
let mut version = VersionBuilder::new(/*...*/);
let profile = auth.authenticate(Some(&event_bus)).await?;

version.launch(&profile, JavaDistribution::Temurin)
    .with_event_bus(&event_bus)
    .with_jvm_options()
        .set("Xmx", "6G")
        .done()
    .run()
    .await?;

// Get PID
if let Some(pid) = version.get_pid() {
    println!("Game running: {}", pid);
}
```

**Complete Documentation:**
- [Launch Process](crates/launch/docs/launch.md) - Complete launch workflow
- [Arguments System](crates/launch/docs/arguments.md) - Placeholders and argument building
- [Installation](crates/launch/docs/installation.md) - Asset/library installation details
- [Instance Control](crates/launch/docs/instance-control.md) - Process management
- [Events](crates/launch/docs/events.md) - Event types reference
- [How to Use](crates/launch/docs/how-to-use.md) - Practical examples

### `lighty_launcher::loaders` - Mod Loaders

Support for multiple Minecraft mod loaders:

```rust
use lighty_launcher::loaders::{Loader, VersionInfo};

// Available loaders
Loader::Vanilla       // Vanilla Minecraft
Loader::Fabric        // Fabric mod loader
Loader::Quilt         // Quilt mod loader
Loader::NeoForge      // NeoForge (modern Forge fork)
Loader::Forge         // Forge
Loader::LightyUpdater // Custom updater system
Loader::Optifine      // OptiFine (experimental)
```

**Loader Status:**

| Loader | Status | Example Version | Minecraft Version |
|--------|--------|-----------------|-------------------|
| Vanilla | ✅ Stable | - | 1.21.1 |
| Fabric | ✅ Stable | 0.17.2 | 1.21.8 |
| Quilt | ✅ Stable | 0.17.10 | 1.18.2 |
| NeoForge | ⚠️ Testing | 20.2.93 | 1.20.2 |
| Forge | ⚠️ Testing | - | - |
| LightyUpdater | ✅ Stable | - | Custom |
| OptiFine | 🧪 Experimental | - | - |

### `lighty_launcher::version` - Version Builders

Build game instances with different loaders:

```rust
use lighty_launcher::version::{VersionBuilder, LightyVersionBuilder};

// Standard Minecraft with loader
let mut version = VersionBuilder::new(
    "fabric-instance",
    Loader::Fabric,
    "0.17.2",       // Loader version
    "1.21.8",       // Minecraft version
    &launcher_dir
);

// LightyUpdater custom version
let mut version = LightyVersionBuilder::new(
    "custom-instance",
    "https://my-server.com/api",
    &launcher_dir
);
```

### `lighty_launcher::core` - Core Utilities

Low-level utilities for system operations:

```rust
use lighty_launcher::core::{hash, extract, download, system};

// SHA1 verification
core::verify_file_sha1(&path, expected_hash).await?;

// Archive extraction
core::extract::zip_extract(reader, output_dir, None).await?;

// System detection
let (os, arch) = core::system::get_os_arch();
```

## Examples

### Fabric with Events

```rust
use lighty_launcher::{
    auth::{OfflineAuth, Authenticator},
    event::{EventBus, Event, LaunchEvent},
    java::JavaDistribution,
    launch::Launch,
    loaders::Loader,
    version::VersionBuilder,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let launcher_dir = ProjectDirs::from("com", "MyLauncher", "").unwrap();

    // Create event bus
    let event_bus = EventBus::new(1000);
    let mut receiver = event_bus.subscribe();

    // Spawn event listener
    tokio::spawn(async move {
        while let Ok(event) = receiver.next().await {
            match event {
                Event::Launch(LaunchEvent::InstallProgress { bytes }) => {
                    println!("Downloaded {} bytes", bytes);
                }
                Event::Launch(LaunchEvent::InstallCompleted { version, .. }) => {
                    println!("{} installation completed!", version);
                }
                _ => {}
            }
        }
    });

    // Authenticate with events
    let mut auth = OfflineAuth::new("Player");
    let profile = auth.authenticate(Some(&event_bus)).await?;

    // Launch with events
    let mut version = VersionBuilder::new(
        "fabric-1.21.8",
        Loader::Fabric,
        "0.17.2",
        "1.21.8",
        &launcher_dir
    );

    version.launch(&profile, JavaDistribution::Temurin)
        .with_event_bus(&event_bus)
        .run()
        .await?;

    Ok(())
}
```

### Microsoft Authentication

```rust
use lighty_launcher::auth::{MicrosoftAuth, Authenticator};

let mut auth = MicrosoftAuth::new();

// Interactive OAuth flow
let profile = auth.authenticate().await?;

println!("Logged in as: {}", profile.username);
println!("UUID: {}", profile.uuid);
```

### Custom Downloader Configuration

```rust
use lighty_launcher::launch::{init_downloader_config, DownloaderConfig};

// Configure before launching
init_downloader_config(DownloaderConfig {
    max_concurrent_downloads: 150,  // More parallel downloads
    max_retries: 10,                 // More retry attempts
    initial_delay_ms: 100,           // Longer initial delay
});

// Launches will use this configuration
version.launch(&profile, JavaDistribution::Temurin).run().await?;
```

## Cargo Features

Control which functionality is compiled:

```toml
# Minimal - Vanilla only
lighty-launcher = { version = "0.6", features = ["vanilla"] }

# With events
lighty-launcher = { version = "0.6", features = ["vanilla", "events"] }

# Multiple loaders
lighty-launcher = { version = "0.6", features = ["vanilla", "fabric", "quilt", "events"] }

# All loaders
lighty-launcher = { version = "0.6", features = ["all-loaders", "events"] }

# With Tauri integration
lighty-launcher = { version = "0.6", features = ["all-loaders", "events", "tauri-commands"] }
```

**Available Features:**
- `vanilla` - Vanilla Minecraft support (required base)
- `fabric` - Fabric loader
- `quilt` - Quilt loader
- `neoforge` - NeoForge loader
- `forge` - Forge loader
- `forge_legacy` - Legacy Forge (1.7.10 - 1.12.2)
- `lighty_updater` - Custom updater system
- `all-loaders` - All mod loaders
- `events` - Event system
- `tauri-commands` - Tauri desktop integration

## Running Examples

```bash
# Vanilla
cargo run --example vanilla --features vanilla,events

# Vanilla with events (detailed progress)
cargo run --example vanilla_with_events --features vanilla,events

# Fabric
cargo run --example fabric --features fabric

# Quilt
cargo run --example quilt --features quilt

# LightyUpdater
cargo run --example lighty_updater --features lighty_updater
```

## Architecture

```
lighty-launcher/
├── src/
│   └── lib.rs                    # Module organization and re-exports
│
├── crates/
│   ├── auth/                     # Authentication
│   │   ├── offline.rs            # Offline auth
│   │   ├── microsoft.rs          # Microsoft OAuth
│   │   ├── azuriom.rs            # Azuriom CMS
│   │   └── custom.rs             # Custom endpoints
│   │
│   ├── event/                    # Event system
│   │   ├── lib.rs                # EventBus, EventReceiver
│   │   ├── errors.rs             # Custom errors
│   │   └── module/               # Event definitions
│   │       ├── auth.rs           # AuthEvent
│   │       ├── java.rs           # JavaEvent
│   │       ├── launch.rs         # LaunchEvent
│   │       ├── loader.rs         # LoaderEvent
│   │       └── core.rs           # CoreEvent
│   │
│   ├── java/                     # Java runtime management
│   │   ├── distribution.rs       # Distribution providers
│   │   ├── jre_downloader.rs     # Download & install
│   │   └── runtime.rs            # Version detection
│   │
│   ├── launch/                   # Game launching
│   │   ├── arguments/            # Argument building
│   │   │   └── arguments.rs      # Arguments trait
│   │   ├── installer/            # Installation logic
│   │   │   ├── installer.rs      # Installer trait
│   │   │   ├── config.rs         # Downloader config
│   │   │   ├── assets.rs         # Asset management
│   │   │   ├── libraries.rs      # Library management
│   │   │   ├── natives.rs        # Native libraries
│   │   │   └── client.rs         # Client JAR
│   │   └── launch/               # Launch orchestration
│   │       ├── runner.rs         # Launch logic
│   │       ├── builder.rs        # LaunchBuilder
│   │       └── config.rs         # LaunchConfig
│   │
│   ├── loaders/                  # Mod loaders
│   │   ├── vanilla/              # Vanilla Minecraft
│   │   ├── fabric/               # Fabric
│   │   ├── quilt/                # Quilt
│   │   ├── neoforge/             # NeoForge
│   │   ├── forge/                # Forge
│   │   ├── lighty_updater/       # Custom updater
│   │   └── utils/                # Caching & utilities
│   │
│   ├── version/                  # Version builders
│   │   ├── version_builder.rs    # Standard builder
│   │   └── lighty_builder.rs     # LightyUpdater builder
│   │
│   └── core/                     # Core utilities
│       ├── system.rs             # OS/Arch detection
│       ├── hosts.rs              # HTTP client
│       ├── download.rs           # Download utilities
│       ├── extract.rs            # Archive extraction
│       └── hash.rs               # SHA1 verification
│
└── examples/                     # Usage examples
    ├── vanilla.rs
    ├── vanilla_with_events.rs
    ├── fabric.rs
    ├── quilt.rs
    ├── neoforge.rs
    └── lighty_updater.rs
```

## Performance

- **Async I/O**: All filesystem and network operations are async
- **Parallel Downloads**: Configurable concurrency (default: 50 concurrent)
- **Smart Caching**: Dual cache system with TTL
- **Event System**: Zero-cost when disabled via feature flags
- **Minimal Dependencies**: Only essential crates
- **Optimized Profiles**:
  - `dev`: Fast compilation with opt-level=2 for dependencies
  - `release`: LTO thin, optimized for performance
  - `release-small`: Size-optimized binary

## Platform Support

| Platform | Status | Architectures |
|----------|--------|---------------|
| Windows | ✅ Tested | x64, ARM64 |
| Linux | ✅ Tested | x64, ARM64 |
| macOS | ✅ Tested | x64 (Intel), ARM64 (Apple Silicon) |

## Requirements

- **Rust 1.75+**
- **Tokio** async runtime
- **Internet connection** for downloads

## Crate Ecosystem

LightyLauncher is composed of multiple focused crates:

- [`lighty-auth`](crates/auth) - Authentication providers
- [`lighty-event`](crates/event) - Event system
- [`lighty-java`](crates/java) - Java runtime management
- [`lighty-launch`](crates/launch) - Game launching
- [`lighty-loaders`](crates/loaders) - Mod loader implementations
- [`lighty-version`](crates/version) - Version builders
- [`lighty-core`](crates/core) - Core utilities

Each crate can be used independently or together through the main `lighty-launcher` crate.

## License

This project is licensed under the **MIT License** - See [LICENSE](LICENSE) for details.

**Clean Room Implementation**: All components were implemented from scratch using only publicly documented APIs. No GPL-licensed code was used or referenced.

## Disclaimer

- **Minecraft** is a trademark of Mojang Studios
- This project is **not affiliated** with Mojang Studios or Microsoft
- For educational and personal use
- Please respect the [Minecraft EULA](https://www.minecraft.net/en-us/eula)

## Links

- **Documentation**: [docs.rs/lighty-launcher](https://docs.rs/lighty-launcher)
- **Crates.io**: [crates.io/crates/lighty-launcher](https://crates.io/crates/lighty-launcher)
- **Repository**: [GitHub](https://github.com/Lighty-Launcher/LightyLauncherLib)
- **Issues**: [GitHub Issues](https://github.com/Lighty-Launcher/LightyLauncherLib/issues)

---

**Made by Hamadi**

*Built with Rust: Tokio, Reqwest, Serde, Thiserror, and more.*
