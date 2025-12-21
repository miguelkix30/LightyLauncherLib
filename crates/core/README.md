# lighty-core

Core utilities and foundational components for the LightyLauncher ecosystem.

## Overview

**Version**: 0.8.6
**Part of**: [LightyLauncher](https://crates.io/crates/lighty-launcher)

`lighty-core` provides essential building blocks used across all LightyLauncher crates:
- **Application State** - Global app configuration and directory management
- **Download System** - Async file downloads with SHA1 verification
- **Archive Extraction** - ZIP, TAR, and TAR.GZ support with security
- **System Detection** - Cross-platform OS and architecture detection
- **Hash Utilities** - SHA1 verification for files and data
- **Logging Macros** - Unified tracing interface
- **File Macros** - Directory creation utilities

## Quick Start

```toml
[dependencies]
lighty-core = "0.8.6"
```

```rust
use lighty_core::{AppState, system::{OS, ARCHITECTURE}};

const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "MyLauncher";
const APPLICATION: &str = "";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize application state
    let _app = AppState::new(
        QUALIFIER.to_string(),
        ORGANIZATION.to_string(),
        APPLICATION.to_string(),
    )?;

    let launcher_dir = AppState::get_project_dirs();

    println!("Data directory: {}", launcher_dir.data_dir().display());
    println!("Cache directory: {}", launcher_dir.cache_dir().display());
    println!("Running on: {:?} {:?}", OS, ARCHITECTURE);

    Ok(())
}
```

## Features

- **Thread-safe state** - Global AppState with OnceCell
- **Secure extraction** - Path traversal protection, file size limits
- **Smart downloads** - Progress tracking and hash verification
- **Cross-platform** - Windows, macOS, Linux support
- **Zero dependencies** - Minimal external dependencies for core operations

## Documentation

📚 **[Complete Documentation](./docs)**

| Guide | Description |
|-------|-------------|
| [How to Use](./docs/how-to-use.md) | Practical usage guide with examples |
| [Overview](./docs/overview.md) | Architecture and design philosophy |
| [Exports](./docs/exports.md) | Complete export reference |
| [Events](./docs/events.md) | CoreEvent types |
| [AppState](./docs/app_state.md) | Application state management |
| [Download](./docs/download.md) | File download system |
| [Extract](./docs/extract.md) | Archive extraction |
| [Hash](./docs/hash.md) | SHA1 verification utilities |
| [System](./docs/system.md) | Platform detection |
| [Macros](./docs/macros.md) | Logging and file macros |

## Related Crates

- **[lighty-launcher](../../../README.md)** - Main package
- **[lighty-event](../event/README.md)** - Event system (for CoreEvent)
- **[lighty-loaders](../loaders/README.md)** - Uses AppState and system detection
- **[lighty-java](../java/README.md)** - Uses download and extract
- **[lighty-launch](../launch/README.md)** - Uses AppState

## License

MIT
