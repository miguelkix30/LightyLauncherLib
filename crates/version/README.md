# lighty-version

Version metadata types for [LightyLauncher](https://crates.io/crates/lighty-launcher).

## Note

This is an internal crate for the LightyLauncher ecosystem. Most users should use the main [`lighty-launcher`](https://crates.io/crates/lighty-launcher) crate instead.

## Features

- **Version Metadata**: Comprehensive metadata structures for Minecraft versions
- **Builder Pattern**: Construct version configurations easily
- **Type Safety**: Strongly typed version information
- **Serialization**: Full serde support for JSON import/export

## Structure

```
lighty-version/
└── src/
    ├── lib.rs              # Module declarations and re-exports
    └── version_metadata.rs # VersionMetaData, VersionBuilder, and related types
```

## Usage

```toml
[dependencies]
lighty-version = "0.6.2"
```

```rust
use lighty_version::version_metadata::{VersionMetaData, VersionBuilder};

// Use version metadata
let metadata = VersionMetaData {
    id: "1.21".to_string(),
    main_class: "net.minecraft.client.main.Main".to_string(),
    // ... other fields
};

// Or use the builder
let builder = VersionBuilder::new()
    .id("1.21")
    .main_class("net.minecraft.client.main.Main")
    .build();
```

## Types

### VersionMetaData

Complete metadata for a Minecraft version including:
- Version ID
- Main class
- Libraries
- Assets
- Arguments
- Download URLs

### VersionBuilder

Builder for constructing version metadata:

```rust
use lighty_version::version_metadata::VersionBuilder;

let builder = VersionBuilder::new()
    .id("1.21")
    .main_class("net.minecraft.client.main.Main")
    .asset_index("16")
    .build();
```

## Integration

This crate is typically used with `lighty-loaders` to provide version information:

```rust
use lighty_loaders::version::Version;
use lighty_version::version_metadata::VersionMetaData;

// Version objects contain VersionMetaData internally
let version = Version::new(/* ... */);
let metadata: &VersionMetaData = version.get_metadata();
```

## License

GPL-3.0-or-later

## Links

- **Main Package**: [lighty-launcher](https://crates.io/crates/lighty-launcher)
- **Repository**: [GitHub](https://github.com/Lighty-Launcher/LightyLauncherLib)
- **Documentation**: [docs.rs/lighty-version](https://docs.rs/lighty-version)
