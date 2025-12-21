# Re-exports Reference

Complete reference of all types and functions re-exported by `lighty-launcher`.

## Module Organization

```rust
lighty-launcher
├── auth          // Authentication providers
├── event         // Event system (with "events" feature)
├── java          // Java runtime management
├── launch        // Game launching and installation
├── loaders       // Mod loader implementations
├── version       // Version builders
├── core          // Core utilities
├── macros        // Utility macros
├── tauri         // Tauri integration (with "tauri-commands" feature)
└── prelude       // Common imports
```

## Authentication Module (`lighty_launcher::auth`)

**Source**: [`lighty-auth`](../crates/auth/README.md)

### Types

| Type | Description | Defined In |
|------|-------------|------------|
| `Authenticator` | Trait for authentication providers | `lighty_auth` |
| `UserProfile` | User authentication data | `lighty_auth` |
| `UserRole` | User role (User, Admin) | `lighty_auth` |
| `AuthProvider` | Authentication provider enum | `lighty_auth` |
| `AuthResult<T>` | Result type for authentication operations | `lighty_auth` |
| `AuthError` | Authentication error types | `lighty_auth` |

### Providers

| Provider | Description |
|----------|-------------|
| `OfflineAuth` | Offline authentication with UUID v5 generation |
| `MicrosoftAuth` | Microsoft OAuth 2.0 authentication |
| `AzuriomAuth` | Azuriom CMS integration |

### Functions

| Function | Description |
|----------|-------------|
| `generate_offline_uuid(username: &str)` | Generate deterministic UUID for offline mode |

**Usage**:
```rust
use lighty_launcher::auth::{OfflineAuth, Authenticator};

let mut auth = OfflineAuth::new("Player");
let profile = auth.authenticate().await?;
```

**Detailed docs**: [lighty-auth](../crates/auth/README.md)

---

## Events Module (`lighty_launcher::event`)

**Source**: [`lighty-event`](../crates/event/README.md)
**Requires**: `events` feature

### Core Types

| Type | Description |
|------|-------------|
| `EventBus` | Multi-producer, multi-consumer event bus |
| `EventReceiver` | Receiver for events (from `subscribe()`) |
| `Event` | Main event enum |

### Event Types

| Event | Description |
|-------|-------------|
| `AuthEvent` | Authentication events |
| `JavaEvent` | Java download/installation events |
| `LaunchEvent` | Game installation/launch events |
| `LoaderEvent` | Loader metadata fetching events |
| `CoreEvent` | Core operations (extraction, etc.) |

### Instance Events

| Event | Description |
|-------|-------------|
| `InstanceLaunchedEvent` | Game instance launched |
| `InstanceExitedEvent` | Game instance exited |
| `ConsoleOutputEvent` | Console output line |
| `InstanceDeletedEvent` | Instance deleted |

### Other Types

| Type | Description |
|------|-------------|
| `ConsoleStream` | Stdout or Stderr |
| `EventReceiveError` | Error receiving events |
| `EventTryReceiveError` | Error trying to receive |
| `EventSendError` | Error sending events |
| `EVENT_BUS` | Global event bus singleton |

**Usage**:
```rust
use lighty_launcher::event::{EventBus, Event};

let event_bus = EventBus::new(1000);
let mut receiver = event_bus.subscribe();

tokio::spawn(async move {
    while let Ok(event) = receiver.next().await {
        println!("Event: {:?}", event);
    }
});
```

**Detailed docs**: [lighty-event](../crates/event/README.md)

---

## Java Module (`lighty_launcher::java`)

**Source**: [`lighty-java`](../crates/java/README.md)

### Types

| Type | Description |
|------|-------------|
| `JavaDistribution` | Java distribution enum (Temurin, GraalVM, Zulu, Liberica) |
| `DistributionSelection` | Distribution selection helper |
| `JavaRuntime` | Java process executor |
| `JreError` | JRE download errors |
| `JreResult<T>` | Result type for JRE operations |
| `JavaRuntimeError` | Runtime execution errors |
| `JavaRuntimeResult<T>` | Result type for runtime operations |
| `DistributionError` | Distribution errors |
| `DistributionResult<T>` | Result type for distribution operations |

### Modules

| Module | Description |
|--------|-------------|
| `jre_downloader` | JRE download and installation functions |

**Usage**:
```rust
use lighty_launcher::java::JavaDistribution;

let distribution = JavaDistribution::Temurin;
// Automatically downloaded and managed by launch system
```

**Detailed docs**: [lighty-java](../crates/java/README.md)

---

## Launch Module (`lighty_launcher::launch`)

**Source**: [`lighty-launch`](../crates/launch/README.md)

### Traits

| Trait | Description |
|-------|-------------|
| `Launch` | Launch builder trait (auto-implemented for VersionInfo types) |
| `Installer` | Installation trait |
| `InstanceControl` | Instance management trait (⚠️ must import!) |

### Types

| Type | Description |
|------|-------------|
| `LaunchBuilder` | Fluent API for configuring launch |
| `LaunchConfig` | Launch configuration |
| `DownloaderConfig` | Download configuration (max retries, concurrency) |
| `LaunchArguments` | Argument building trait |

### Functions

| Function | Description |
|----------|-------------|
| `init_downloader_config(config: DownloaderConfig)` | Configure global downloader settings |

### Errors

| Type | Description |
|------|-------------|
| `InstallerError` | Installation errors |
| `InstallerResult<T>` | Result type for installer operations |
| `InstanceError` | Instance management errors |
| `InstanceResult<T>` | Result type for instance operations |

### Launch Keys (`lighty_launcher::launch::keys`)

All placeholder constants for argument customization:

| Constant | Placeholder | Value |
|----------|-------------|-------|
| `KEY_AUTH_PLAYER_NAME` | `${auth_player_name}` | `"auth_player_name"` |
| `KEY_AUTH_UUID` | `${auth_uuid}` | `"auth_uuid"` |
| `KEY_AUTH_ACCESS_TOKEN` | `${auth_access_token}` | `"auth_access_token"` |
| `KEY_AUTH_XUID` | `${auth_xuid}` | `"auth_xuid"` |
| `KEY_CLIENT_ID` | `${clientid}` | `"clientid"` |
| `KEY_USER_TYPE` | `${user_type}` | `"user_type"` |
| `KEY_USER_PROPERTIES` | `${user_properties}` | `"user_properties"` |
| `KEY_VERSION_NAME` | `${version_name}` | `"version_name"` |
| `KEY_VERSION_TYPE` | `${version_type}` | `"version_type"` |
| `KEY_GAME_DIRECTORY` | `${game_directory}` | `"game_directory"` |
| `KEY_ASSETS_ROOT` | `${assets_root}` | `"assets_root"` |
| `KEY_NATIVES_DIRECTORY` | `${natives_directory}` | `"natives_directory"` |
| `KEY_LIBRARY_DIRECTORY` | `${library_directory}` | `"library_directory"` |
| `KEY_ASSETS_INDEX_NAME` | `${assets_index_name}` | `"assets_index_name"` |
| `KEY_LAUNCHER_NAME` | `${launcher_name}` | `"launcher_name"` |
| `KEY_LAUNCHER_VERSION` | `${launcher_version}` | `"launcher_version"` |
| `KEY_CLASSPATH` | `${classpath}` | `"classpath"` |
| `KEY_CLASSPATH_SEPARATOR` | `${classpath_separator}` | `"classpath_separator"` |

**Usage**:
```rust
use lighty_launcher::launch::{Launch, InstanceControl};
use lighty_launcher::launch::keys::*;

// Launch
instance.launch(&profile, JavaDistribution::Temurin)
    .with_arguments()
        .set(KEY_LAUNCHER_NAME, "MyLauncher")
        .done()
    .run()
    .await?;

// Instance control (must import trait!)
if let Some(pid) = instance.get_pid() {
    instance.close_instance(pid).await?;
}
```

**Detailed docs**: [lighty-launch](../crates/launch/README.md)

---

## Loaders Module (`lighty_launcher::loaders`)

**Source**: [`lighty-loaders`](../crates/loaders/README.md)

### Types

| Type | Description |
|------|-------------|
| `Loader` | Loader enum (Vanilla, Fabric, Quilt, Forge, NeoForge, etc.) |
| `VersionInfo` | Trait for version information |
| `LoaderExtensions` | Trait for loader operations (auto-implemented) |
| `InstanceSize` | Instance size calculation |

### Version Metadata

| Type | Description |
|------|-------------|
| `Version` | Complete version metadata |
| `VersionMetaData` | Version metadata wrapper |
| `Library` | Library dependency |
| `Asset` | Asset file |
| `AssetIndex` | Asset index |
| `Arguments` | Game/JVM arguments |
| `MainClass` | Main class info |
| `Mods` | Mod list |
| `Native` | Native library |

### Loader Implementations

| Module | Loader |
|--------|--------|
| `vanilla` | Vanilla Minecraft |
| `fabric` | Fabric mod loader |
| `quilt` | Quilt mod loader |
| `forge` | Forge mod loader |
| `neoforge` | NeoForge mod loader |
| `lighty_updater` | LightyUpdater custom server |
| `optifine` | OptiFine |

### Utilities

| Module | Description |
|--------|-------------|
| `cache` | Manifest caching utilities |
| `error` | Error types |
| `manifest` | Manifest fetching |
| `query` | Version querying |

**Usage**:
```rust
use lighty_launcher::loaders::{Loader, VersionInfo, LoaderExtensions};

let loader = Loader::Fabric;

// VersionInfo trait provides metadata access
let name = instance.name();
let mc_version = instance.minecraft_version();

// LoaderExtensions provides async operations (auto-implemented)
let metadata = instance.get_metadata().await?;
let libraries = instance.get_libraries().await?;
```

**Detailed docs**: [lighty-loaders](../crates/loaders/README.md)

---

## Version Module (`lighty_launcher::version`)

**Source**: [`lighty-version`](../crates/version/README.md)

### Types

| Type | Description |
|------|-------------|
| `VersionBuilder` | Standard version builder for all loaders |
| `LightyVersionBuilder` | Custom server version builder |

**Usage**:
```rust
use lighty_launcher::version::VersionBuilder;

let instance = VersionBuilder::new(
    "my-instance",
    Loader::Vanilla,
    "",
    "1.21.1",
    launcher_dir
);
```

**Detailed docs**: [lighty-version](../crates/version/README.md)

---

## Core Module (`lighty_launcher::core`)

**Source**: [`lighty-core`](../crates/core/README.md)

### Types

| Type | Description |
|------|-------------|
| `AppState` | Application state and project directories |
| `AppStateError` | AppState errors |
| `AppStateResult<T>` | AppState result type |
| `SystemError` | System operation errors |
| `SystemResult<T>` | System result type |
| `ExtractError` | Archive extraction errors |
| `ExtractResult<T>` | Extract result type |
| `DownloadError` | Download errors |
| `DownloadResult<T>` | Download result type |
| `HashError` | Hashing errors |
| `HashResult<T>` | Hash result type |

### Modules

| Module | Description |
|--------|-------------|
| `system` | System detection and operations |
| `hosts` | Hosts file management |
| `download` | HTTP download utilities |
| `extract` | Archive extraction (ZIP, TAR.GZ) |
| `hash` | SHA1 hashing utilities |

### Functions

| Function | Description |
|----------|-------------|
| `verify_file_sha1(path, hash)` | Verify file SHA1 (async) |
| `verify_file_sha1_streaming(path, hash)` | Verify with streaming |
| `calculate_file_sha1_sync(path)` | Calculate SHA1 (blocking) |
| `verify_file_sha1_sync(path, hash)` | Verify SHA1 (blocking) |
| `calculate_sha1_bytes(bytes)` | Calculate SHA1 from bytes |
| `calculate_sha1_bytes_raw(bytes)` | Raw SHA1 calculation |

**Usage**:
```rust
use lighty_launcher::core::AppState;

const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "MyLauncher";
const APPLICATION: &str = "";

let _app = AppState::new(
    QUALIFIER.to_string(),
    ORGANIZATION.to_string(),
    APPLICATION.to_string(),
)?;

let launcher_dir = AppState::get_project_dirs();
```

**Detailed docs**: [lighty-core](../crates/core/README.md)

---

## Macros Module (`lighty_launcher::macros`)

**Source**: `lighty-core`

### Tracing Macros

| Macro | Description | Requires Feature |
|-------|-------------|------------------|
| `trace_debug!()` | Debug level logging | `tracing` (no-op otherwise) |
| `trace_info!()` | Info level logging | `tracing` (no-op otherwise) |
| `trace_warn!()` | Warning level logging | `tracing` (no-op otherwise) |
| `trace_error!()` | Error level logging | `tracing` (no-op otherwise) |
| `time_it!(name, block)` | Performance timing | `tracing` (no-op otherwise) |

### File System Macros

| Macro | Description |
|-------|-------------|
| `mkdir!(path)` | Async directory creation with error logging |
| `join_and_mkdir!(base, segment)` | Join path and create directory |
| `join_and_mkdir_vec!(base, segments)` | Join multiple segments and create |
| `mkdir_blocking!(path)` | Blocking directory creation |

**Usage**:
```rust
use lighty_launcher::macros::*;

trace_info!("Starting operation");

let result = time_it!("my_operation", {
    // Expensive operation
    42
});

let path = std::path::Path::new("/tmp/my_dir");
mkdir!(path);
```

---

## Tauri Module (`lighty_launcher::tauri`)

**Source**: `lighty-tauri`
**Requires**: `tauri-commands` feature

### Plugin

| Function | Description |
|----------|-------------|
| `lighty_plugin()` | Main Tauri plugin with all commands |

### Commands

All Tauri commands are re-exported:
- Authentication commands
- Launch commands
- Java distribution commands
- Loader commands
- Version management commands
- Path utilities

### Types

| Type | Description |
|------|-------------|
| `AppState` | Tauri app state |
| `VersionConfig` | Version configuration |
| `LaunchConfig` | Launch configuration |
| `LaunchResult` | Launch result |
| `LoaderInfo` | Loader information |
| `JavaDistInfo` | Java distribution info |

**Usage**:
```rust
use lighty_launcher::tauri::lighty_plugin;

tauri::Builder::default()
    .plugin(lighty_plugin())
    .run(tauri::generate_context!())
    .expect("error running tauri application");
```

---

## Prelude (`lighty_launcher::prelude`)

Convenient re-exports of most commonly used types.

### Included Types

```rust
use lighty_launcher::prelude::*;

// Authentication
Authenticator, UserProfile, OfflineAuth, MicrosoftAuth, AzuriomAuth

// Events (with "events" feature)
EventBus, Event, AuthEvent, JavaEvent, LaunchEvent, LoaderEvent, CoreEvent,
InstanceLaunchedEvent, InstanceExitedEvent, ConsoleOutputEvent,
InstanceDeletedEvent, ConsoleStream, EVENT_BUS

// Java
JavaDistribution

// Launch
Launch, LaunchBuilder, DownloaderConfig, init_downloader_config,
InstanceControl, InstanceError, InstanceResult

// Launch keys (all KEY_* constants)

// Loaders
Loader, VersionInfo, LoaderExtensions, InstanceSize

// Version
VersionBuilder, LightyVersionBuilder

// Core
AppState

// Macros
trace_debug, trace_info, trace_warn, trace_error
```

**Usage**:
```rust
use lighty_launcher::prelude::*;

// All common types are now available
let mut auth = OfflineAuth::new("Player");
let instance = VersionBuilder::new(...);
```

---

## Root Re-exports

Most commonly used types are also available at the root:

```rust
use lighty_launcher::{
    Loader,               // loaders::Loader
    JavaDistribution,     // java::JavaDistribution
    Launch,               // launch::Launch
    Authenticator,        // auth::Authenticator
    UserProfile,          // auth::UserProfile
    VersionBuilder,       // version::VersionBuilder
    LightyVersionBuilder, // version::LightyVersionBuilder
};
```

---

## Import Patterns

### Pattern 1: Prelude (Recommended)

```rust
use lighty_launcher::prelude::*;

// Everything you need is available
```

### Pattern 2: Specific Modules

```rust
use lighty_launcher::{
    auth::{OfflineAuth, Authenticator},
    java::JavaDistribution,
    launch::Launch,
    loaders::Loader,
    version::VersionBuilder,
    core::AppState,
};
```

### Pattern 3: Root + Modules

```rust
use lighty_launcher::{VersionBuilder, Loader, JavaDistribution};
use lighty_launcher::auth::OfflineAuth;
use lighty_launcher::launch::Launch;
```

### Pattern 4: Direct Crate Access

```rust
// Use underlying crates directly (advanced)
use lighty_auth::OfflineAuth;
use lighty_launch::Launch;
use lighty_version::VersionBuilder;
```

---

## Related Documentation

- [Sequence Diagrams](./sequence-diagrams.md) - Visual flow diagrams
- [Architecture](./architecture.md) - System architecture
- [Examples](./examples.md) - Code examples

### Crate Documentation

- [lighty-core](../crates/core/README.md)
- [lighty-auth](../crates/auth/README.md)
- [lighty-event](../crates/event/README.md)
- [lighty-java](../crates/java/README.md)
- [lighty-launch](../crates/launch/README.md)
- [lighty-loaders](../crates/loaders/README.md)
- [lighty-version](../crates/version/README.md)
