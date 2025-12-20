# lighty-event

[![Crates.io](https://img.shields.io/crates/v/lighty-event.svg)](https://crates.io/crates/lighty-event)
[![Documentation](https://docs.rs/lighty-event/badge.svg)](https://docs.rs/lighty-event)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Event system for LightyLauncher - A simple, efficient broadcast-based event system for tracking launcher operations.

## Features

- **Async-first** - Built on tokio's broadcast channels
- **Multiple subscribers** - Broadcast events to multiple listeners simultaneously
- **Typed events** - Strongly typed event system with compile-time safety
- **Comprehensive** - Events for authentication, downloads, installations, and more
- **Progress tracking** - Real-time progress updates for long-running operations
- **Zero-cost** - No overhead when events feature is disabled
- **Error handling** - Custom error types with thiserror
- **Extensible** - Extend with traits for ergonomic APIs

## Event Categories

### Authentication Events (`AuthEvent`)
- `AuthenticationStarted` - Authentication process begins
- `AuthenticationInProgress` - Ongoing authentication with step info
- `AuthenticationSuccess` - Authentication succeeded
- `AuthenticationFailed` - Authentication failed with error
- `AlreadyAuthenticated` - Valid session already exists

### Java Events (`JavaEvent`)
- `JavaNotFound` - JRE not installed
- `JavaAlreadyInstalled` - JRE already present
- `JavaDownloadStarted` - JRE download begins
- `JavaDownloadProgress` - Download progress updates
- `JavaDownloadCompleted` - Download finished
- `JavaExtractionStarted` - Extraction begins
- `JavaExtractionProgress` - Extraction progress
- `JavaExtractionCompleted` - Extraction finished

### Launch Events (`LaunchEvent`)
- `IsInstalled` - Files already up-to-date
- `InstallStarted` - Installation begins
- `InstallProgress` - Installation progress
- `InstallCompleted` - Installation finished
- `Launching` - Game launch starting
- `Launched` - Game process spawned
- `NotLaunched` - Launch failed
- `ProcessOutput` - Game process output
- `ProcessExited` - Game process exited

### Loader Events (`LoaderEvent`)
- `FetchingData` - Fetching loader manifest
- `DataFetched` - Manifest retrieved
- `ManifestNotFound` - Version not found
- `ManifestCached` - Using cached manifest
- `MergingLoaderData` - Merging loader data
- `DataMerged` - Merge completed

### Core Events (`CoreEvent`)
- `ExtractionStarted` - Archive extraction begins
- `ExtractionProgress` - Extraction progress
- `ExtractionCompleted` - Extraction finished

### Instance Events (NEW)
- `InstanceLaunched` - Instance started with PID
- `ConsoleOutput` - Real-time stdout/stderr lines
- `InstanceExited` - Instance exited with exit code
- `InstanceDeleted` - Instance deleted from disk

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
lighty-event = "0.6"
```

## Usage

### Basic Example

```rust
use lighty_event::{EventBus, Event, LaunchEvent, EventReceiveError};

#[tokio::main]
async fn main() {
    // Create event bus with buffer capacity
    let event_bus = EventBus::new(1000);

    // Subscribe to events
    let mut receiver = event_bus.subscribe();

    // Spawn listener task
    tokio::spawn(async move {
        loop {
            match receiver.next().await {
                Ok(event) => {
                    handle_event(event);
                }
                Err(EventReceiveError::BusDropped) => {
                    println!("Event bus closed");
                    break;
                }
                Err(EventReceiveError::Lagged { skipped }) => {
                    eprintln!("Warning: Missed {} events", skipped);
                }
            }
        }
    });

    // Use the event bus with launcher operations...
}

fn handle_event(event: Event) {
    match event {
        Event::Launch(LaunchEvent::InstallStarted { version, total_bytes }) => {
            println!("Installing {} ({} MB)", version, total_bytes / 1_000_000);
        }
        Event::Launch(LaunchEvent::InstallProgress { bytes }) => {
            println!("Downloaded {} bytes", bytes);
        }
        Event::Java(JavaEvent::JavaDownloadStarted { distribution, version, total_bytes }) => {
            println!("Downloading {} {} ({} MB)", distribution, version, total_bytes / 1_000_000);
        }
        _ => {}
    }
}
```

### With LightyLauncher

```rust
use lighty_launcher::{JavaDistribution, Launch, Loader, VersionBuilder};
use lighty_auth::{offline::OfflineAuth, Authenticator};
use lighty_event::{EventBus, Event, LaunchEvent};
use directories::ProjectDirs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let launcher_dir = ProjectDirs::from("fr", ".LightyLauncher", "")
        .expect("Failed to get project directories");

    // Create event bus
    let event_bus = EventBus::new(1000);
    let mut receiver = event_bus.subscribe();

    // Spawn event listener
    tokio::spawn(async move {
        while let Ok(event) = receiver.next().await {
            match event {
                Event::Launch(LaunchEvent::InstallProgress { bytes }) => {
                    println!("Progress: {} bytes", bytes);
                }
                Event::Launch(LaunchEvent::InstallCompleted { version, .. }) => {
                    println!("Installation of {} completed!", version);
                }
                _ => {}
            }
        }
    });

    // Authenticate
    let mut auth = OfflineAuth::new("Player");
    let profile = auth.authenticate(Some(&event_bus)).await?;

    // Build and launch
    let mut version = VersionBuilder::new(
        "my-instance",
        Loader::Vanilla,
        "",
        "1.21.1",
        &launcher_dir
    );

    version.launch(&profile, JavaDistribution::Temurin)
        .with_event_bus(&event_bus)
        .run()
        .await?;

    Ok(())
}
```

### Console Streaming Example (NEW)

The global `EVENT_BUS` automatically streams instance console output:

```rust
use lighty_event::{Event, EVENT_BUS};
use lighty_launch::InstanceControl;
use lighty_version::VersionBuilder;
use lighty_loaders::types::Loader;
use lighty_auth::OfflineAuth;
use lighty_java::JavaDistribution;
use directories::ProjectDirs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let launcher_dir = ProjectDirs::from("fr", ".LightyLauncher", "")
        .expect("Failed to get project directories");

    // Subscribe to global event bus
    let mut receiver = EVENT_BUS.subscribe();

    // Spawn console listener
    tokio::spawn(async move {
        while let Ok(event) = receiver.next().await {
            match event {
                Event::InstanceLaunched(e) => {
                    println!("Instance {} launched (PID: {})", e.instance_name, e.pid);
                    println!("Version: {}", e.version);
                    println!("Player: {}", e.username);
                }
                Event::ConsoleOutput(e) => {
                    // Stream real-time console output
                    match e.stream {
                        ConsoleStream::Stdout => print!("[OUT] {}", e.line),
                        ConsoleStream::Stderr => print!("[ERR] {}", e.line),
                    }
                }
                Event::InstanceExited(e) => {
                    println!("Instance {} exited (code: {:?})", e.instance_name, e.exit_code);
                }
                Event::InstanceDeleted(e) => {
                    println!("Instance {} deleted", e.instance_name);
                }
                _ => {}
            }
        }
    });

    // Authenticate
    let mut auth = OfflineAuth::new("Player");
    let profile = auth.authenticate().await?;

    // Build instance
    let mut minozia = VersionBuilder::new(
        "minozia",
        Loader::Fabric,
        "0.15.0",
        "1.20.1",
        &launcher_dir
    );

    // Launch (events are automatically emitted)
    minozia.launch(&profile, JavaDistribution::Temurin)
        .with_jvm_options()
            .set("Xmx", "4G")
            .done()
        .run()
        .await?;

    // Get PID and manage instance
    if let Some(pid) = minozia.get_pid() {
        println!("Instance running with PID: {}", pid);

        // Wait a bit, then close
        tokio::time::sleep(std::time::Duration::from_secs(30)).await;
        minozia.close_instance(pid).await?;
    }

    Ok(())
}
```

### Progress Tracking Example

```rust
use lighty_event::{EventBus, Event, LaunchEvent, JavaEvent};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

#[tokio::main]
async fn main() {
    let event_bus = EventBus::new(1000);
    let mut receiver = event_bus.subscribe();

    let total_bytes = Arc::new(AtomicU64::new(0));
    let downloaded_bytes = Arc::new(AtomicU64::new(0));

    tokio::spawn({
        let total = Arc::clone(&total_bytes);
        let downloaded = Arc::clone(&downloaded_bytes);

        async move {
            while let Ok(event) = receiver.next().await {
                match event {
                    Event::Launch(LaunchEvent::InstallStarted { total_bytes: t, .. }) => {
                        total.store(t, Ordering::Relaxed);
                        downloaded.store(0, Ordering::Relaxed);
                    }
                    Event::Launch(LaunchEvent::InstallProgress { bytes }) => {
                        downloaded.fetch_add(bytes, Ordering::Relaxed);

                        let d = downloaded.load(Ordering::Relaxed);
                        let t = total.load(Ordering::Relaxed);

                        if t > 0 {
                            let percent = (d as f64 / t as f64) * 100.0;
                            print!("\rProgress: {:.1}%", percent);
                            std::io::Write::flush(&mut std::io::stdout()).ok();
                        }
                    }
                    Event::Launch(LaunchEvent::InstallCompleted { .. }) => {
                        println!("\nInstallation completed!");
                    }
                    _ => {}
                }
            }
        }
    });

    // Use launcher...
}
```

### Multi-Subscriber Example

```rust
use lighty_event::{EventBus, Event, LaunchEvent};

#[tokio::main]
async fn main() {
    let event_bus = EventBus::new(1000);

    // UI subscriber
    let mut ui_receiver = event_bus.subscribe();
    tokio::spawn(async move {
        while let Ok(event) = ui_receiver.next().await {
            // Update UI with event
            update_ui(event);
        }
    });

    // Logger subscriber
    let mut log_receiver = event_bus.subscribe();
    tokio::spawn(async move {
        while let Ok(event) = log_receiver.next().await {
            // Log event to file
            log_event(&event);
        }
    });

    // Analytics subscriber
    let mut analytics_receiver = event_bus.subscribe();
    tokio::spawn(async move {
        while let Ok(event) = analytics_receiver.next().await {
            // Send analytics
            send_analytics(event);
        }
    });

    // All subscribers receive the same events
    // Use launcher with event_bus...
}

fn update_ui(event: Event) { /* ... */ }
fn log_event(event: &Event) { /* ... */ }
fn send_analytics(event: Event) { /* ... */ }
```

## Error Handling

The event system provides custom error types:

### `EventReceiveError`
- `BusDropped` - Event bus has been closed
- `Lagged { skipped }` - Receiver fell behind, some events were missed

### `EventTryReceiveError`
- `Empty` - No events available (non-blocking)
- `BusDropped` - Event bus has been closed
- `Lagged { skipped }` - Receiver fell behind

### `EventSendError`
- `NoReceivers` - No active receivers (event not sent)

Example with error handling:

```rust
use lighty_event::{EventReceiver, EventReceiveError};

async fn listen_events(mut receiver: EventReceiver) {
    loop {
        match receiver.next().await {
            Ok(event) => {
                // Handle event
                println!("Received: {:?}", event);
            }
            Err(EventReceiveError::BusDropped) => {
                eprintln!("Event bus closed, exiting listener");
                break;
            }
            Err(EventReceiveError::Lagged { skipped }) => {
                eprintln!("Warning: Receiver lagged, missed {} events", skipped);
                // Continue listening, but some events were lost
            }
        }
    }
}
```

## Buffer Size Recommendations

The buffer size determines how many events can be buffered before older events are dropped:

- **Small applications**: 100-500 events
- **Medium applications**: 500-2000 events
- **Large applications**: 2000-5000 events
- **Very slow receivers**: 5000+ events

```rust
// Small buffer for simple use cases
let event_bus = EventBus::new(100);

// Larger buffer for complex applications with slow receivers
let event_bus = EventBus::new(5000);
```

If receivers are too slow and the buffer fills up, the oldest events will be dropped and receivers will get a `Lagged` error.

## Non-Blocking Receive

For non-blocking event checking, use `try_next()`:

```rust
use lighty_event::{EventReceiver, EventTryReceiveError};

fn poll_events(receiver: &mut EventReceiver) {
    match receiver.try_next() {
        Ok(event) => {
            println!("Got event: {:?}", event);
        }
        Err(EventTryReceiveError::Empty) => {
            // No events available right now
        }
        Err(EventTryReceiveError::BusDropped) => {
            eprintln!("Event bus closed");
        }
        Err(EventTryReceiveError::Lagged { skipped }) => {
            eprintln!("Missed {} events", skipped);
        }
    }
}
```

## Feature Flags

The event system is optional and can be disabled:

```toml
[dependencies]
lighty-launcher = { version = "0.6", features = ["events"] }
lighty-event = "0.6"
```

When the `events` feature is disabled, event-related code is compiled out with zero overhead.

## Serialization

All events implement `Serialize` and `Deserialize` (via serde), making them easy to:
- Send over the network
- Store in files
- Log to JSON
- Integrate with web APIs (in Tauri)

```rust
use lighty_event::{Event, LaunchEvent};
use serde_json;

let event = Event::Launch(LaunchEvent::InstallStarted {
    version: "1.21.1".to_string(),
    total_bytes: 1000000,
});

// Serialize to JSON
let json = serde_json::to_string(&event).unwrap();
println!("{}", json);

// Deserialize from JSON
let deserialized: Event = serde_json::from_str(&json).unwrap();
```

## Module Structure

```
lighty-event/
├── src/
│   ├── lib.rs          # EventBus, EventReceiver, EVENT_BUS
│   ├── errors.rs       # Error types
│   └── module/         # Event definitions
│       ├── mod.rs      # Module exports
│       ├── auth.rs     # AuthEvent
│       ├── core.rs     # CoreEvent
│       ├── java.rs     # JavaEvent
│       ├── launch.rs   # LaunchEvent
│       ├── loader.rs   # LoaderEvent
│       └── console.rs  # Instance events (NEW)
└── README.md
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

Licensed under the MIT License. See [LICENSE](../../LICENSE) for details.

## Related Crates

- [`lighty-launcher`](../launch) - Main launcher crate
- [`lighty-auth`](../auth) - Authentication system
- [`lighty-java`](../java) - Java runtime management
- [`lighty-core`](../core) - Core utilities
