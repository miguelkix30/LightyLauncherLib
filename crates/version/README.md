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
    ├── lib.rs                  # Module declarations and re-exports
    ├── version_builder.rs      # Version builder pattern
    ├── lighty_builder.rs       # LightyUpdater builder
    └── loader_extensions_impl.rs # Loader-specific extensions
```

## Usage

```toml
[dependencies]
lighty-version = "0.6.3"
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

## Documentation

📚 **[Complete Documentation](./docs)**

| Guide | Description |
|-------|-------------|
| [Overview](./docs/overview.md) | Architecture and design philosophy |
| [Version Metadata](./docs/metadata.md) | Metadata structures |
| [Builder Pattern](./docs/builder.md) | Using the builder API |
| [Examples](./docs/examples.md) | Complete usage examples |

## License

MIT

## Links

- **Main Package**: [lighty-launcher](https://crates.io/crates/lighty-launcher)
- **Repository**: [GitHub](https://github.com/Lighty-Launcher/LightyLauncherLib)
- **Documentation**: [docs.rs/lighty-version](https://docs.rs/lighty-version)
