# Exports

## Overview

Complete reference of all exports from `lighty-launch` and their re-exports in `lighty-launcher`.

## In `lighty_launch`

### Launch System

```rust
use lighty_launch::{
    LaunchBuilder,    // Builder for launching instances
    LaunchConfig,     // Launch configuration
};
```

### Traits

```rust
use lighty_launch::{
    InstanceControl,  // MUST import to use instance management methods
    Installer,        // Asset/library installation
};
```

### Instance Management

```rust
use lighty_launch::instance::{
    InstanceControl,
    InstanceError,
    InstanceResult,
};
```

### Installer

```rust
use lighty_launch::installer::{
    Installer,
    // Internal modules (not typically used directly):
    // assets, libraries, natives, mods, client, config
};
```

### Arguments

```rust
use lighty_launch::arguments::Arguments;
```

### Errors

```rust
use lighty_launch::errors::{
    InstallerError,
    InstallerResult,
    InstanceError,
    InstanceResult,
};
```

## In `lighty_launcher` (Re-exports)

```rust
use lighty_launcher::launch::{
    // Launch
    LaunchBuilder,
    LaunchConfig,

    // Traits
    InstanceControl,
    Installer,

    // Errors
    errors::{
        InstallerError,
        InstallerResult,
        InstanceError,
        InstanceResult,
    },

    // Arguments
    arguments::Arguments,
};
```

## Usage Patterns

### Pattern 1: Direct Crate Import

```rust
use lighty_launch::{LaunchBuilder, InstanceControl};
use lighty_core::AppState;
use lighty_launcher::prelude::*;

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
    let mut instance = VersionBuilder::new(
        "test",
        Loader::Vanilla,
        "",
        "1.21.1",
        launcher_dir
    );

    let mut auth = OfflineAuth::new("Player");

    #[cfg(not(feature = "events"))]
    let profile = auth.authenticate().await?;

    instance.launch(&profile, JavaDistribution::Temurin)
        .run()
        .await?;

    Ok(())
}
```

### Pattern 2: Via Main Launcher Crate

```rust
use lighty_launcher::{
    prelude::*,
    launch::InstanceControl,
};

// Use methods from InstanceControl trait
if let Some(pid) = instance.get_pid() {
    println!("Running: {}", pid);
}
```

### Pattern 3: Prelude

```rust
use lighty_launcher::prelude::*;
use lighty_launch::InstanceControl; // Still need to import trait explicitly

// VersionBuilder is in prelude, but InstanceControl must be imported
let mut instance = VersionBuilder::new(/*...*/);
instance.launch(&profile, JavaDistribution::Temurin).run().await?;
```

## Type Details

### LaunchBuilder

```rust
pub struct LaunchBuilder<'a, V: VersionInfo> {
    // ...
}

impl<'a, V: VersionInfo> LaunchBuilder<'a, V> {
    pub fn with_jvm_options(self) -> JvmOptionsBuilder<'a, V>;
    pub fn with_game_options(self) -> GameOptionsBuilder<'a, V>;
    pub async fn run(self) -> InstallerResult<()>;
}
```

### InstanceControl Trait

```rust
pub trait InstanceControl: VersionInfo {
    fn get_pid(&self) -> Option<u32>;
    fn get_pids(&self) -> Vec<u32>;
    async fn close_instance(&self, pid: u32) -> InstanceResult<()>;
    async fn delete_instance(&self) -> InstanceResult<()>;
    fn size_of_instance(&self, version: &Version) -> InstanceSize;
}
```

**IMPORTANT**: This trait is automatically implemented for any type implementing `VersionInfo`, but you **MUST import the trait** to use its methods:

```rust
use lighty_launch::InstanceControl; // Required!
```

### InstallerError

```rust
pub enum InstallerError {
    DownloadFailed(String),
    VerificationFailed(String),
    ExtractionFailed(String),
    IOError(std::io::Error),
    // ...
}
```

### InstanceError

```rust
pub enum InstanceError {
    InstanceRunning,
    InstanceNotFound,
    ProcessKillFailed,
    // ...
}
```

## Module Structure

```
lighty_launch
├── launch
│   ├── LaunchBuilder
│   ├── LaunchConfig
│   ├── builder (internal)
│   ├── runner (internal)
│   └── config (internal)
├── installer
│   ├── Installer (trait)
│   ├── assets (internal)
│   ├── libraries (internal)
│   ├── natives (internal)
│   ├── mods (internal)
│   ├── client (internal)
│   └── config (internal)
├── instance
│   ├── InstanceControl (trait)
│   ├── InstanceError
│   ├── InstanceResult
│   ├── manager (internal)
│   ├── utilities (internal)
│   └── console (internal)
├── arguments
│   └── Arguments
└── errors
    ├── InstallerError
    ├── InstallerResult
    ├── InstanceError
    └── InstanceResult
```

## Error Handling

### Installer Errors

```rust
use lighty_launch::errors::{InstallerError, InstallerResult};

match instance.launch(&profile, JavaDistribution::Temurin).run().await {
    Ok(_) => println!("Launched"),
    Err(InstallerError::DownloadFailed(url)) => {
        eprintln!("Download failed: {}", url);
    }
    Err(InstallerError::VerificationFailed(file)) => {
        eprintln!("Verification failed: {}", file);
    }
    Err(e) => {
        eprintln!("Launch error: {}", e);
    }
}
```

### Instance Errors

```rust
use lighty_launch::errors::{InstanceError, InstanceResult};
use lighty_launch::InstanceControl;

match instance.delete_instance().await {
    Ok(_) => println!("Deleted"),
    Err(InstanceError::InstanceRunning) => {
        eprintln!("Cannot delete: instance is running");
    }
    Err(InstanceError::InstanceNotFound) => {
        eprintln!("Instance not found");
    }
    Err(e) => {
        eprintln!("Error: {}", e);
    }
}
```

## Feature-Gated Exports

### Events Feature

```toml
[dependencies]
lighty-launch = { version = "0.8.6", features = ["events"] }
```

When `events` feature is enabled:
- `LaunchEvent` types are emitted (from `lighty-event`)
- Console output is streamed via events
- Instance lifecycle events are emitted

```rust
#[cfg(feature = "events")]
use lighty_event::{EventBus, Event, LaunchEvent};

#[cfg(feature = "events")]
{
    let event_bus = EventBus::new(1000);
    // Events are automatically emitted during launch
    instance.launch(&profile, JavaDistribution::Temurin).run().await?;
}
```

## Related Documentation

- [How to Use](./how-to-use.md) - Practical usage examples
- [Events](./events.md) - LaunchEvent types
- [Overview](./overview.md) - Architecture overview
- [Instance Control](./instance-control.md) - Detailed instance management
