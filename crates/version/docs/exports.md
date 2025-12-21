# Exports

## Overview

Complete reference of all exports from `lighty-version` and their re-exports in `lighty-launcher`.

## In `lighty_version`

### Version Builders

```rust
use lighty_version::{
    VersionBuilder,         // Standard version builder
    LightyVersionBuilder,   // Custom server version builder
};
```

## In `lighty_launcher` (Re-exports)

```rust
use lighty_launcher::version::{
    VersionBuilder,
    LightyVersionBuilder,
};

// Or via prelude
use lighty_launcher::prelude::*;  // Includes VersionBuilder
```

## Usage Patterns

### Pattern 1: Direct Crate Import

```rust
use lighty_version::VersionBuilder;
use lighty_loaders::types::Loader;
use lighty_core::AppState;

const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "MyLauncher";
const APPLICATION: &str = "";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _app = AppState::new(
        QUALIFIER.to_string(),
        ORGANIZATION.to_string(),
        APPLICATION.to_string(),
    )?;

    let launcher_dir = AppState::get_project_dirs();

    let instance = VersionBuilder::new(
        "test",
        Loader::Vanilla,
        "",
        "1.21.1",
        launcher_dir
    );

    Ok(())
}
```

### Pattern 2: Via Main Launcher Crate

```rust
use lighty_launcher::version::VersionBuilder;
use lighty_launcher::loaders::Loader;
use lighty_core::AppState;

const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "MyLauncher";
const APPLICATION: &str = "";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _app = AppState::new(
        QUALIFIER.to_string(),
        ORGANIZATION.to_string(),
        APPLICATION.to_string(),
    )?;

    let launcher_dir = AppState::get_project_dirs();

    let instance = VersionBuilder::new(
        "test",
        Loader::Vanilla,
        "",
        "1.21.1",
        launcher_dir
    );

    Ok(())
}
```

### Pattern 3: Prelude

```rust
use lighty_launcher::prelude::*;
use lighty_core::AppState;

const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "MyLauncher";
const APPLICATION: &str = "";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _app = AppState::new(
        QUALIFIER.to_string(),
        ORGANIZATION.to_string(),
        APPLICATION.to_string(),
    )?;

    let launcher_dir = AppState::get_project_dirs();

    // VersionBuilder is included in prelude
    let instance = VersionBuilder::new(
        "test",
        Loader::Vanilla,
        "",
        "1.21.1",
        launcher_dir
    );

    Ok(())
}
```

## Type Details

### VersionBuilder

```rust
pub struct VersionBuilder<'a, L = ()> {
    pub name: String,
    pub loader: L,
    pub loader_version: String,
    pub minecraft_version: String,
    pub project_dirs: &'a Lazy<ProjectDirs>,
    pub game_dirs: PathBuf,
    pub java_dirs: PathBuf,
}

impl<'a, L> VersionBuilder<'a, L> {
    pub fn new(
        name: &str,
        loader: L,
        loader_version: &str,
        minecraft_version: &str,
        project_dirs: &'a Lazy<ProjectDirs>,
    ) -> Self;

    pub fn with_custom_game_dir(self, game_dir: PathBuf) -> Self;
    pub fn with_custom_java_dir(self, java_dir: PathBuf) -> Self;
}
```

**Implements**:
- `VersionInfo` from `lighty-loaders`
- `Clone`, `Debug`

### LightyVersionBuilder

```rust
pub struct LightyVersionBuilder<'a> {
    pub name: String,
    pub server_url: String,
    pub minecraft_version: Option<String>,
    pub loader: Option<Loader>,
    pub project_dirs: &'a Lazy<ProjectDirs>,
    pub game_dirs: PathBuf,
    pub java_dirs: PathBuf,
}

impl<'a> LightyVersionBuilder<'a> {
    pub fn new(
        name: &str,
        server_url: &str,
        project_dirs: &'a Lazy<ProjectDirs>,
    ) -> Self;
}
```

**Implements**:
- `VersionInfo` from `lighty-loaders`
- `Clone`, `Debug`

## VersionInfo Implementation

Both `VersionBuilder` and `LightyVersionBuilder` implement `VersionInfo`:

```rust
use lighty_loaders::types::VersionInfo;

// All these methods are available:
instance.name()                    // -> &str
instance.loader()                  // -> &Loader
instance.loader_version()          // -> &str
instance.minecraft_version()       // -> &str
instance.game_dirs()               // -> &Path
instance.java_dirs()               // -> &Path
instance.game_dir_exists()         // -> bool
instance.java_dir_exists()         // -> bool
instance.full_identifier()         // -> String
instance.paths()                   // -> (&Path, &Path)
```

## LoaderExtensions Integration

Because `VersionBuilder` and `LightyVersionBuilder` implement `VersionInfo`, they automatically get `LoaderExtensions`:

```rust
use lighty_loaders::types::LoaderExtensions;

// These methods become available:
instance.get_metadata().await?         // Get full metadata
instance.get_libraries().await?        // Get libraries only
instance.get_assets().await?           // Get assets only
instance.get_main_class().await?       // Get main class only
instance.get_natives().await?          // Get natives only
instance.get_java_version().await?     // Get Java version only
```

## InstanceControl Integration

When using with `lighty-launch`, both builders get instance management:

```rust
use lighty_launch::InstanceControl;  // Must import trait

// These methods become available:
instance.get_pid()                     // -> Option<u32>
instance.get_pids()                    // -> Vec<u32>
instance.close_instance(pid).await?    // -> Result<()>
instance.delete_instance().await?      // -> Result<()>
instance.size_of_instance(&version)    // -> InstanceSize
```

## Module Structure

```
lighty_version
├── version_builder
│   └── VersionBuilder
└── lighty_builder
    └── LightyVersionBuilder
```

## Complete Example

```rust
use lighty_core::AppState;
use lighty_version::{VersionBuilder, LightyVersionBuilder};
use lighty_loaders::types::{Loader, VersionInfo, LoaderExtensions};
use lighty_launch::InstanceControl;

const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "MyLauncher";
const APPLICATION: &str = "";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _app = AppState::new(
        QUALIFIER.to_string(),
        ORGANIZATION.to_string(),
        APPLICATION.to_string(),
    )?;

    let launcher_dir = AppState::get_project_dirs();

    // VersionBuilder
    let mut fabric = VersionBuilder::new(
        "fabric",
        Loader::Fabric,
        "0.16.9",
        "1.21.1",
        launcher_dir
    );

    // VersionInfo methods
    println!("Name: {}", fabric.name());
    println!("Loader: {:?}", fabric.loader());

    // LoaderExtensions methods (automatic)
    let metadata = fabric.get_metadata().await?;
    println!("Libraries: {}", metadata.libraries.len());

    // InstanceControl methods (with lighty-launch)
    if let Some(pid) = fabric.get_pid() {
        println!("Running: {}", pid);
    }

    // LightyVersionBuilder
    let mut modpack = LightyVersionBuilder::new(
        "modpack",
        "https://server.com/api",
        launcher_dir
    );

    // Same methods available
    println!("Server: {}", modpack.loader_version());
    let metadata = modpack.get_metadata().await?;

    Ok(())
}
```

## Related Documentation

- [How to Use](./how-to-use.md) - Practical usage examples
- [Overview](./overview.md) - Architecture overview
- [VersionBuilder](./version-builder.md) - Standard builder details
- [LightyVersionBuilder](./lighty-version-builder.md) - Custom server builder details
- [lighty-loaders Traits](../../loaders/docs/traits.md) - VersionInfo and LoaderExtensions details
