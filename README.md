# LightyLauncher

[![Crates.io](https://img.shields.io/crates/v/lighty-launcher.svg)](https://crates.io/crates/lightylauncher)
[![Documentation](https://docs.rs/lighty-launcher/badge.svg)](https://docs.rs/lightylauncher)
[![License: GPL-3.0](https://img.shields.io/badge/License-GPL%203.0-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Rust Version](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)

> ⚠️ **ACTIVE DEVELOPMENT** - API may change between versions. Use with caution in production.

A modern, async Minecraft launcher library for Rust supporting multiple mod loaders with an optimized architecture based on an intelligent caching system.

## Features

- **Multi-Loader Support**: Vanilla, Fabric, Quilt, NeoForge, Forge, OptiFine
- **Async/Await Architecture**: Built on Tokio for maximum performance
- **Automatic Java Management**: Download and manage JRE distributions (Temurin, GraalVM)
- **Smart Caching System**: Dual cache (raw + query) with configurable TTL and automatic cleanup
- **Tauri Integration**: Pre-configured commands for desktop applications
- **Cross-Platform**: Windows, Linux, and macOS support
- **Type-Safe**: Strongly typed API with comprehensive error handling via `thiserror`
- **Performance Optimized**:
  - Parallel downloads with concurrency limits
  - SHA1 verification for file integrity
  - Async archive extraction
  - Intelligent cache reuse

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
lighty-launcher = { version = "0.1", features = ["all-loaders"] }
tokio = { version = "1.48", features = ["full"] }
directories = "6.0"
once_cell = "1.21"
tracing-subscriber = "0.3"
```

## Quick Start

### Basic Example - Vanilla

```rust
use lighty_launcher::{JavaDistribution, Launch, Loader, Version};
use directories::ProjectDirs;
use once_cell::sync::Lazy;
use tracing::{info, error};

static LAUNCHER_DIRECTORY: Lazy<ProjectDirs> = Lazy::new(|| {
    ProjectDirs::from("com", "MyLauncher", "")
        .expect("Failed to create project directories")
});

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let username = "PlayerName";
    let uuid = "37fefc81-1e26-4d31-a988-74196affc99b";

    let mut version = Version::new(
        "vanilla-1.21",
        Loader::Vanilla,
        "",
        "1.21",
        &LAUNCHER_DIRECTORY
    );

    match version.launch(username, uuid, JavaDistribution::Temurin).await {
        Ok(()) => info!("✅ Launch successful!"),
        Err(e) => error!("❌ Launch failed: {:?}", e),
    }
}
```

### Fabric Example

```rust
let mut fabric = Version::new(
    "fabric-1.21",
    Loader::Fabric,
    "0.16.9",      // Fabric loader version
    "1.21",        // Minecraft version
    &LAUNCHER_DIRECTORY
);

fabric.launch("Player", "uuid", JavaDistribution::Temurin).await?;
```

## Supported Loaders

| Loader | Status | Example Loader Version | Example MC Version |
|--------|--------|------------------------|-------------------|
| **Vanilla** | ✅ Stable | - | `1.21` |
| **Fabric** | ✅ Stable | `0.16.9` | `1.21` |
| **Quilt** | ✅ Stable | `0.27.1` | `1.21` |
| **NeoForge** | ⚠️ Testing | `21.1.80` | `1.21` |
| **Forge** | ⚠️ Testing | `51.0.38` | `1.21` |
| **OptiFine** | 🚧 Experimental | `HD_U_I9` | `1.21` |

### Examples by Loader

<details>
<summary><b>Vanilla</b></summary>

```rust
let mut vanilla = Version::new("vanilla-1.21", Loader::Vanilla, "", "1.21", &LAUNCHER_DIRECTORY);
vanilla.launch(username, uuid, JavaDistribution::Temurin).await?;
```
</details>

<details>
<summary><b>Fabric</b></summary>

```rust
let mut fabric = Version::new("fabric-1.21", Loader::Fabric, "0.16.9", "1.21", &LAUNCHER_DIRECTORY);
fabric.launch(username, uuid, JavaDistribution::Temurin).await?;
```
</details>

<details>
<summary><b>Quilt</b></summary>

```rust
let mut quilt = Version::new("quilt-1.21", Loader::Quilt, "0.27.1", "1.21", &LAUNCHER_DIRECTORY);
quilt.launch(username, uuid, JavaDistribution::Temurin).await?;
```
</details>

<details>
<summary><b>NeoForge</b></summary>

```rust
let mut neoforge = Version::new("neoforge-1.21", Loader::NeoForge, "21.1.80", "1.21", &LAUNCHER_DIRECTORY);
neoforge.launch(username, uuid, JavaDistribution::GraalVM).await?;
```
</details>

<details>
<summary><b>Forge</b></summary>

```rust
let mut forge = Version::new("forge-1.21", Loader::Forge, "51.0.38", "1.21", &LAUNCHER_DIRECTORY);
forge.launch(username, uuid, JavaDistribution::Temurin).await?;
```
</details>

<details>
<summary><b>OptiFine</b></summary>

```rust
let mut optifine = Version::new("optifine-1.21", Loader::Optifine, "HD_U_I9", "1.21", &LAUNCHER_DIRECTORY);
optifine.launch(username, uuid, JavaDistribution::Temurin).await?;
```
</details>

## Java Distributions

LightyLauncher automatically manages Java runtime download and installation:

| Distribution | Support | Recommended For | Java Versions |
|--------------|---------|-----------------|---------------|
| **Temurin** | ✅ | General use | 8, 11, 17, 21 |
| **GraalVM** | ✅ | Maximum performance | 17+ |

```rust
// Temurin (recommended, supports Java 8-21)
JavaDistribution::Temurin

// GraalVM (high performance, Java 17+ only)
JavaDistribution::GraalVM
```

The library automatically:
- Detects the required Java version for each Minecraft version
- Downloads the JRE if not present
- Verifies SHA1 checksums
- Extracts and configures the runtime

## Tauri Integration

LightyLauncher provides ready-to-use Tauri commands for desktop applications.

### Installation

```toml
[dependencies]
lighty-launcher = { version = "0.1", features = ["all-loaders", "tauri-commands"] }
```

### Backend Setup

```rust
// src-tauri/src/lib.rs
use lighty_launcher::tauri::tauri_commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  let _app_state = AppState::new(
    "fr".to_string(),
    "Polar".to_string(),
    "".to_string()
  );

  tauri::Builder::default()
          .invoke_handler(tauri::generate_handler![
            launch,
            get_loaders,
            get_java_distributions,
            get_launcher_path,
            check_version_exists,
        ])
          .run(tauri::generate_context!())
          .expect("error");
}
```

### Frontend Usage (TypeScript)

```typescript
import { invoke } from '@tauri-apps/api/tauri';

await invoke('launch', {
  versionConfig: {
    name: 'fabric-1.21',
    loader: 'fabric',
    loaderVersion: '0.16.9',
    minecraftVersion: '1.21',
  },
  launchConfig: {
    username: 'Hamadi',
    uuid: '37fefc81-1e26-4d31-a988-74196affc99b',
    javaDistribution: 'temurin',
  },
});
```

📖 **Full documentation**: See [TAURI_USAGE.md](TAURI_USAGE.md)

## Cargo Features

Control which loaders are compiled into your binary:

```toml
# All loaders
lighty-launcher = { version = "0.1", features = ["all-loaders"] }

# Specific loaders only
lighty-launcher = { version = "0.1", features = ["vanilla", "fabric"] }

# With Tauri integration
lighty-launcher = { version = "0.1", features = ["all-loaders", "tauri-commands"] }
```

**Available features**:
- `vanilla` - Vanilla Minecraft support (base for all loaders)
- `fabric` - Fabric loader support
- `quilt` - Quilt loader support
- `neoforge` - NeoForge loader support
- `forge` - Forge loader support
- `forge_legacy` - Forge Legacy support (1.7.10 - 1.12.2)
- `all-loaders` - Enable all mod loaders
- `tauri-commands` - Pre-configured Tauri commands

**Default features**: None (choose your loaders)

## Architecture

```
lightylauncher/
├── java/                   # Java runtime management
│   ├── distribution.rs     # JRE distributions (Temurin, GraalVM)
│   ├── jre_downloader.rs   # Download and installation
│   └── runtime.rs          # Java version detection
│
├── minecraft/
│   ├── auth/               # Authentication (WIP)
│   ├── launch/             # Launch logic
│   │   ├── arguments.rs    # JVM and game arguments
│   │   ├── installer.rs    # Assets/libraries installation
│   │   ├── launch.rs       # Launch trait
│   │   └── errors.rs       # Installer errors
│   │
│   ├── loaders/            # Mod loader implementations
│   │   ├── vanilla/
│   │   ├── fabric/
│   │   ├── quilt/
│   │   ├── neoforge/
│   │   ├── forge/
│   │   └── optifine/
│   │
│   ├── utils/              # Minecraft utilities
│   │   ├── cache.rs        # Smart cache with TTL
│   │   ├── manifest.rs     # Repository with dual cache
│   │   ├── query.rs        # Query trait for loaders
│   │   └── error.rs        # Query errors
│   │
│   └── version/            # Version management
│
├── utils/                  # General utilities
│   ├── download.rs         # Async downloads
│   ├── extract.rs          # Archive extraction
│   ├── system.rs           # OS/Architecture detection
│   └── macros.rs           # Utility macros
│
└── tauri/                  # Tauri integration
    └── tauri_commands.rs   # Ready-to-use commands
```

### Caching System

LightyLauncher uses a **dual cache architecture** for optimal performance:

1. **`raw_version_cache`**: Stores complete JSON manifests
2. **`query_cache`**: Stores extracted data by query

Each cache features:
- **Configurable TTL** per data type (via `Query::cache_ttl()`)
- **Automatic cleanup** with adaptive sleep
- **Thread-safe** with `Arc<RwLock<HashMap>>`

## Examples

The [`examples/`](examples/) directory contains complete examples for each loader:

```bash
cargo run --example vanilla --features vanilla
cargo run --example fabric --features fabric
cargo run --example quilt --features quilt
cargo run --example neoforge --features neoforge
cargo run --example forge --features forge
```

## Requirements

- **Rust 1.75+**
- **Tokio** async runtime
- **Internet connection** for downloading game files and JRE

## Platform Support

| Platform | Status | Notes |
|----------|--------|-------|
| Windows | ✅ Tested | x64, ARM64 |
| Linux | ✅ Tested | x64, ARM64 |
| macOS | ✅ Tested | x64 (Intel), ARM64 (Apple Silicon) |

## Performance

LightyLauncher is optimized for performance:

- **Async I/O**: All filesystem and network operations are async
- **Parallel Downloads**: Configurable concurrency limits
- **Smart Caching**: Avoids re-downloads and re-extractions
- **SHA1 Verification**: Guaranteed integrity without overhead
- **Minimal Dependencies**: Only essential crates included
- **Optimized Build Profiles**:
  - `dev`: Optimizes dependencies (opt-level=2)
  - `release`: LTO thin, codegen-units=1
  - `release-small`: Binary size optimized

## License

This project is licensed under the **GNU General Public License v3.0 or later**.
See the [LICENSE](LICENSE) file for details.

## Disclaimer

- **Minecraft** is a trademark of Mojang Studios
- This project is **not affiliated** with Mojang Studios or Microsoft
- For educational and personal use
- Please respect the [Minecraft EULA](https://www.minecraft.net/en-us/eula)

## Links

- **Documentation**: [docs.rs/lightylauncher](https://docs.rs/lightylauncher)
- **Crates.io**: [crates.io/crates/lightylauncher](https://crates.io/crates/lightylauncher)
- **Repository**: [GitHub](https://github.com/Lighty-Launcher/LightyLauncherLib)
- **Issues**: [GitHub Issues](https://github.com/Lighty-Launcher/LightyLauncherLib/issues)
- **Tauri Guide**: [TAURI_USAGE.md](TAURI_USAGE.md)

---

**Made with ❤️ by Hamadi**

*Built with the Rust ecosystem: Tokio, Reqwest, Serde, Thiserror, and more!*
