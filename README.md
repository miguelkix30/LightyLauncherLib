# LightyLauncher

[![Crates.io](https://img.shields.io/crates/v/lighty-launcher.svg)](https://crates.io/crates/lighty-launcher)
[![Documentation](https://docs.rs/lighty-launcher/badge.svg)](https://docs.rs/lighty-launcher)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust Version](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)

> **ACTIVE DEVELOPMENT** - API may change between versions. Use with caution in production.

A modern, modular Minecraft launcher library for Rust with full async support, real-time event system, and automatic Java management.

![LightyUpdater Banner](docs/img/banner.png)

## Features

- **Modular Architecture**: Organized into logical crates (`auth`, `event`, `java`, `launch`, `loaders`, `version`, `core`)
- **Multi-Loader Support**: Vanilla, Fabric, Quilt, NeoForge, Forge, OptiFine, LightyUpdater
- **Event System**: Real-time progress tracking for all operations
- **Authentication**: Offline, Microsoft OAuth 2.0, Azuriom CMS + extensibility for custom providers
- **Automatic Java Management**: Download and manage JRE distributions (Temurin, GraalVM, Zulu, Liberica)
- **Cross-Platform**: Windows, Linux, and macOS support

## Installation

```toml
[dependencies]
lighty-launcher = "0.8.6"
lighty-core = "0.8.6"
tokio = { version = "1", features = ["full"] }
anyhow = "1.0"
```

## Quick Start

```rust
use lighty_core::AppState;
use lighty_launcher::prelude::*;
use lighty_auth::{offline::OfflineAuth, Authenticator};
use lighty_java::JavaDistribution;

const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "MyLauncher";
const APPLICATION: &str = "";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize AppState
    let _app = AppState::new(
        QUALIFIER.to_string(),
        ORGANIZATION.to_string(),
        APPLICATION.to_string(),
    )?;

    let launcher_dir = AppState::get_project_dirs();

    // Create instance
    let mut instance = VersionBuilder::new(
        "my-instance",
        Loader::Vanilla,
        "",
        "1.21.1",
        launcher_dir
    );

    // Authenticate
    let mut auth = OfflineAuth::new("Player123");
    let profile = auth.authenticate().await?;

    // Launch
    instance.launch(&profile, JavaDistribution::Temurin)
        .run()
        .await?;

    Ok(())
}
```

## Documentation

Complete documentation for each crate:

### Core Crates

| Crate | Description | Documentation |
|-------|-------------|---------------|
| **[lighty-core](crates/core/README.md)** | Core utilities and AppState management | [📚 Docs](crates/core/README.md) |
| **[lighty-launcher](crates/launcher/README.md)** | Main package with re-exports | [📚 Docs](crates/launcher/README.md) |

### Feature Crates

| Crate | Description | Documentation |
|-------|-------------|---------------|
| **[lighty-auth](crates/auth/README.md)** | Authentication (Offline, Microsoft, Azuriom) | [📚 Docs](crates/auth/README.md) |
| **[lighty-java](crates/java/README.md)** | Java runtime management | [📚 Docs](crates/java/README.md) |
| **[lighty-launch](crates/launch/README.md)** | Game launching and process management | [📚 Docs](crates/launch/README.md) |
| **[lighty-loaders](crates/loaders/README.md)** | Mod loader implementations | [📚 Docs](crates/loaders/README.md) |
| **[lighty-version](crates/version/README.md)** | Version builders | [📚 Docs](crates/version/README.md) |
| **[lighty-event](crates/event/README.md)** | Event system for progress tracking | [📚 Docs](crates/event/README.md) |

### Detailed Guides

#### lighty-launch Documentation

| Guide | Description |
|-------|-------------|
| [Launch Process](crates/launch/docs/launch.md) | Complete launch workflow (5 phases) |
| [Arguments System](crates/launch/docs/arguments.md) | Placeholders, JVM options, game arguments |
| [Installation](crates/launch/docs/installation.md) | Asset/library installation details |
| [Instance Control](crates/launch/docs/instance-control.md) | Process management and PID tracking |
| [Events](crates/launch/docs/events.md) | Event types reference |
| [How to Use](crates/launch/docs/how-to-use.md) | Practical examples |
| [Exports](crates/launch/docs/exports.md) | Module exports reference |

#### lighty-version Documentation

| Guide | Description |
|-------|-------------|
| [How to Use](crates/version/docs/how-to-use.md) | Practical usage guide |
| [Overview](crates/version/docs/overview.md) | Architecture and design |
| [Exports](crates/version/docs/exports.md) | Module exports reference |
| [VersionBuilder](crates/version/docs/version-builder.md) | Standard builder details |
| [LightyVersionBuilder](crates/version/docs/lighty-version-builder.md) | Custom server builder |

## Examples

```bash
# Vanilla Minecraft
cargo run --example vanilla --features vanilla,events

# Fabric
cargo run --example fabric --features fabric

# Quilt
cargo run --example quilt --features quilt

# LightyUpdater
cargo run --example lighty_updater --features lighty_updater
```

## Cargo Features

```toml
# Minimal - Vanilla only
lighty-launcher = { version = "0.8.6", features = ["vanilla"] }

# With events
lighty-launcher = { version = "0.8.6", features = ["vanilla", "events"] }

# Multiple loaders
lighty-launcher = { version = "0.8.6", features = ["vanilla", "fabric", "quilt", "events"] }

# All loaders
lighty-launcher = { version = "0.8.6", features = ["all-loaders", "events"] }
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

## Architecture

```
lighty-launcher/
├── crates/
│   ├── core/           # Core utilities and AppState
│   ├── auth/           # Authentication providers
│   ├── event/          # Event system
│   ├── java/           # Java runtime management
│   ├── launch/         # Launch orchestration
│   ├── loaders/        # Mod loader implementations
│   ├── version/        # Version builders
│   └── launcher/       # Main package with re-exports
├── examples/           # Usage examples
└── docs/              # Additional documentation
```

## Contributing

Contributions are welcome! Please read the [Contributing Guide](CONTRIBUTING.md) before submitting a PR.

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Related Projects

- **[LightyUpdater](https://github.com/Lighty-Launcher/LightyUpdater)** - Custom modpack server for LightyLauncher
