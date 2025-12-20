# lighty-core

Core utilities and foundational components for the LightyLauncher ecosystem.

## Overview

`lighty-core` provides essential building blocks used across all LightyLauncher crates:
- **Application State Management** - Global app configuration and directory management
- **File Operations** - Async downloads with retry logic and SHA1 verification
- **Archive Extraction** - ZIP, TAR, and TAR.GZ support
- **System Detection** - Cross-platform OS and architecture detection
- **HTTP Client** - Shared async HTTP client with connection pooling

## Quick Start

```toml
[dependencies]
lighty-core = "0.8.6"
```

```rust
use lighty_core::{AppState, download_file, get_os};

#[tokio::main]
async fn main() {
    // Initialize application state
    let _app = AppState::new("com".into(), "MyCompany".into(), "MyApp".into()).unwrap();

    // Detect system
    let os = get_os();
    println!("Running on: {:?}", os);

    // Download a file with SHA1 verification
    download_file(
        "https://example.com/file.zip",
        "/tmp/file.zip",
        Some("expected-sha1-hash")
    ).await.unwrap();
}
```

## Documentation

📚 **[Complete Documentation](./docs)**

| Guide | Description |
|-------|-------------|
| [Overview](./docs/overview.md) | Architecture overview and design philosophy |
| [Application State](./docs/app_state.md) | AppState initialization and usage |
| [Download System](./docs/download.md) | File downloads and verification |
| [Archive Extraction](./docs/extract.md) | Working with archives |
| [System Detection](./docs/system.md) | Platform detection |
| [Logging Macros](./docs/macros.md) | Using trace macros |
| [Examples](./docs/examples.md) | Complete code examples |

## License

MIT

## Links

- **Main Package**: [lighty-launcher](https://crates.io/crates/lighty-launcher)
- **Repository**: [GitHub](https://github.com/Lighty-Launcher/LightyLauncherLib)
- **Documentation**: [docs.rs/lighty-core](https://docs.rs/lighty-core)
