# Events

## Overview

`lighty-launch` emits `LaunchEvent` types through the event bus system provided by `lighty-event`. These events track the launch process and instance lifecycle.

**Feature**: Requires `events` feature flag

**Export**:
- Event types: `lighty_event::LaunchEvent`
- Re-export: `lighty_launcher::event::LaunchEvent`

## LaunchEvent Types

### DownloadingAssets

Emitted during asset download progress.

**Fields**:
- `current: usize` - Number of assets downloaded
- `total: usize` - Total number of assets to download

**When emitted**: During asset installation

**Example**:
```rust
Event::Launch(LaunchEvent::DownloadingAssets { current, total }) => {
    let progress = (current as f64 / total as f64) * 100.0;
    println!("Assets: {}/{} ({:.1}%)", current, total, progress);
}
```

### DownloadingLibraries

Emitted during library download progress.

**Fields**:
- `current: usize` - Number of libraries downloaded
- `total: usize` - Total number of libraries to download

**When emitted**: During library installation

**Example**:
```rust
Event::Launch(LaunchEvent::DownloadingLibraries { current, total }) => {
    let progress = (current as f64 / total as f64) * 100.0;
    println!("Libraries: {}/{} ({:.1}%)", current, total, progress);
}
```

### DownloadingNatives

Emitted during native library download progress.

**Fields**:
- `current: usize` - Number of natives downloaded
- `total: usize` - Total number of natives to download

**When emitted**: During native library installation

**Example**:
```rust
Event::Launch(LaunchEvent::DownloadingNatives { current, total }) => {
    println!("Natives: {}/{}", current, total);
}
```

### DownloadingMods

Emitted during mod download progress.

**Fields**:
- `current: usize` - Number of mods downloaded
- `total: usize` - Total number of mods to download

**When emitted**: During mod installation (for loaders with mod metadata)

**Example**:
```rust
Event::Launch(LaunchEvent::DownloadingMods { current, total }) => {
    println!("Mods: {}/{}", current, total);
}
```

### InstanceLaunched

Emitted when Minecraft process starts successfully.

**Fields**:
- `instance_name: String` - Name of the launched instance
- `pid: u32` - Process ID

**When emitted**: After game process spawns

**Example**:
```rust
Event::Launch(LaunchEvent::InstanceLaunched { instance_name, pid }) => {
    println!("✓ {} launched with PID {}", instance_name, pid);
}
```

### ConsoleOutput

Emitted for each line of console output (stdout/stderr).

**Fields**:
- `pid: u32` - Process ID
- `line: String` - Console output line

**When emitted**: Real-time as game outputs to console

**Example**:
```rust
Event::Launch(LaunchEvent::ConsoleOutput { pid, line }) => {
    print!("[PID {}] {}", pid, line);
}
```

### InstanceExited

Emitted when game process exits.

**Fields**:
- `pid: u32` - Process ID
- `exit_code: Option<i32>` - Exit code (None if killed)

**When emitted**: After game process terminates

**Example**:
```rust
Event::Launch(LaunchEvent::InstanceExited { pid, exit_code }) => {
    match exit_code {
        Some(0) => println!("Game exited normally"),
        Some(code) => println!("Game crashed with code: {}", code),
        None => println!("Game was killed"),
    }
}
```

### InstanceDeleted

Emitted when instance is deleted from disk.

**Fields**:
- `instance_name: String` - Name of deleted instance

**When emitted**: After `delete_instance()` completes

**Example**:
```rust
Event::Launch(LaunchEvent::InstanceDeleted { instance_name }) => {
    println!("Instance {} deleted", instance_name);
}
```

## Complete Event Flow

### Launch Process

```
DownloadingLibraries (repeated)
    ↓
DownloadingNatives (repeated)
    ↓
DownloadingAssets (repeated)
    ↓
DownloadingMods (if applicable, repeated)
    ↓
InstanceLaunched
    ↓
ConsoleOutput (continuous)
    ↓
InstanceExited
```

### Instance Deletion

```
InstanceDeleted
```

## Complete Example

```rust
use lighty_event::{EventBus, Event, LaunchEvent};
use lighty_launch::InstanceControl;
use lighty_core::AppState;
use lighty_launcher::prelude::*;
use lighty_java::JavaDistribution;

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

    let event_bus = EventBus::new(1000);
    let mut receiver = event_bus.subscribe();

    tokio::spawn(async move {
        while let Ok(event) = receiver.next().await {
            match event {
                Event::Launch(LaunchEvent::DownloadingLibraries { current, total }) => {
                    println!("📦 Libraries: {}/{}", current, total);
                }
                Event::Launch(LaunchEvent::DownloadingNatives { current, total }) => {
                    println!("🔧 Natives: {}/{}", current, total);
                }
                Event::Launch(LaunchEvent::DownloadingAssets { current, total }) => {
                    println!("🎨 Assets: {}/{}", current, total);
                }
                Event::Launch(LaunchEvent::DownloadingMods { current, total }) => {
                    println!("🧩 Mods: {}/{}", current, total);
                }
                Event::Launch(LaunchEvent::InstanceLaunched { instance_name, pid }) => {
                    println!("✓ Launched {} (PID: {})", instance_name, pid);
                }
                Event::Launch(LaunchEvent::ConsoleOutput { pid, line }) => {
                    print!("[{}] {}", pid, line);
                }
                Event::Launch(LaunchEvent::InstanceExited { pid, exit_code }) => {
                    match exit_code {
                        Some(0) => println!("✓ Instance {} exited normally", pid),
                        Some(code) => println!("✗ Instance {} crashed (code: {})", pid, code),
                        None => println!("⚠ Instance {} was killed", pid),
                    }
                }
                Event::Launch(LaunchEvent::InstanceDeleted { instance_name }) => {
                    println!("🗑 Deleted {}", instance_name);
                }
                _ => {}
            }
        }
    });

    let launcher_dir = AppState::get_project_dirs();
    let mut instance = VersionBuilder::new(
        "fabric-1.21",
        Loader::Fabric,
        "0.16.9",
        "1.21.1",
        launcher_dir
    );

    let mut auth = OfflineAuth::new("Player");
    let profile = auth.authenticate(None).await?;

    instance.launch(&profile, JavaDistribution::Temurin)
        .run()
        .await?;

    // Keep alive to see console output
    tokio::time::sleep(tokio::time::Duration::from_secs(120)).await;

    Ok(())
}
```

## Related Documentation

- [How to Use](./how-to-use.md) - Practical examples with events
- [Exports](./exports.md) - Complete export reference
- [lighty-event Events](../../event/docs/events.md) - All event types
