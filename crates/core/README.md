# lighty-core

Core utilities for [LightyLauncher](https://crates.io/crates/lighty-launcher).

## Note

This is an internal crate for the LightyLauncher ecosystem. Most users should use the main [`lighty-launcher`](https://crates.io/crates/lighty-launcher) crate instead.

## Features

- **Async Downloads**: Concurrent downloads with retry logic and SHA1 verification
- **Archive Extraction**: Support for ZIP, TAR, and TAR.GZ formats
- **System Detection**: OS and architecture detection
- **Hosts Management**: HTTP client with custom user agent
- **Error Handling**: Comprehensive error types with thiserror

## Usage

```toml
[dependencies]
lighty-core = "0.6.2"
```

```rust
use lighty_core::download::download_file;
use lighty_core::system::{get_os, get_architecture};

#[tokio::main]
async fn main() {
    // Detect system
    let os = get_os();
    let arch = get_architecture();
    println!("Running on {:?} {:?}", os, arch);

    // Download file
    let path = download_file(
        "https://example.com/file.zip",
        "/tmp/file.zip",
        Some("expected-sha1-hash")
    ).await?;
}
```

## Structure

```
lighty-core/
└── src/
    ├── lib.rs          # Module declarations and re-exports
    ├── download.rs     # Async file downloads with SHA1 verification
    ├── extract.rs      # Archive extraction (ZIP, TAR, TAR.GZ)
    ├── system.rs       # OS and architecture detection
    ├── hosts.rs        # HTTP client with custom user agent
    ├── errors.rs       # Error types (DownloadError, ExtractError, SystemError)
    └── macros.rs       # Utility macros
```

## Modules

- `download` - Async file downloads with retry logic and SHA1 verification
- `extract` - Archive extraction for ZIP, TAR, and TAR.GZ formats
- `system` - Cross-platform OS and architecture detection
- `hosts` - Shared HTTP client with appropriate user agent
- `errors` - Comprehensive error types with thiserror
- `macros` - Utility macros for common patterns

## License

GPL-3.0-or-later

## Links

- **Main Package**: [lighty-launcher](https://crates.io/crates/lighty-launcher)
- **Repository**: [GitHub](https://github.com/Lighty-Launcher/LightyLauncherLib)
- **Documentation**: [docs.rs/lighty-core](https://docs.rs/lighty-core)
