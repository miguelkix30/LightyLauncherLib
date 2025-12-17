# lighty-tauri

Tauri integration for [LightyLauncher](https://crates.io/crates/lighty-launcher).

## Note

This is an internal crate for the LightyLauncher ecosystem. Most users should use the main [`lighty-launcher`](https://crates.io/crates/lighty-launcher) crate with the `tauri-commands` feature instead.

## Features

- **Pre-configured Tauri Commands**: Ready-to-use commands for desktop applications
- **Type-Safe API**: Strongly typed command interfaces
- **Async Support**: Full async/await support
- **Error Handling**: Proper error propagation to frontend

## Structure

```
lighty-tauri/
└── src/
    ├── lib.rs              # Module declarations and re-exports
    ├── core.rs             # Core types (re-exports AppState from lighty-core)
    ├── tauri_commands.rs   # Main command exports
    └── commands/           # Command implementations
        ├── mod.rs
        ├── auth/           # Authentication commands
        │   └── mod.rs      # Offline, Microsoft, Azuriom
        ├── core/           # Core commands
        │   └── mod.rs      # AppState init, path utilities
        ├── java/           # Java distribution commands
        │   └── mod.rs
        ├── launch/         # Launch command
        │   └── mod.rs
        ├── loaders/        # Loader enumeration
        │   └── mod.rs
        ├── version/        # Version checking
        │   └── mod.rs
        └── utils/          # Utilities
            ├── mod.rs
            └── parse.rs    # Configuration parsing
```

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
lighty-launcher = { version = "0.8.2", features = ["tauri-commands", "all-loaders"] }
tauri = { version = "2", features = [] }
```

### Backend Setup

The easiest way to integrate LightyLauncher with Tauri is to use the **plugin**:

```rust
use lighty_launcher::tauri::lighty_plugin;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(lighty_plugin())
        .run(tauri::generate_context!())
        .expect("error running tauri application");
}
```

That's it! All commands are automatically registered and ready to use from your frontend.

### Why Use the Plugin?

The plugin approach is recommended because:
- ✅ **Simple**: One line of code to add all commands
- ✅ **No macro issues**: Avoids cross-crate macro visibility problems
- ✅ **Future-proof**: New commands are automatically available when you update
- ✅ **Type-safe**: Full TypeScript type support on the frontend

### Frontend Usage (TypeScript/JavaScript)

```typescript
import { invoke } from '@tauri-apps/api/tauri';

// Initialize app state (REQUIRED - call first!)
await invoke('init_app_state', {
  qualifier: 'com',
  organization: 'example',
  application: 'MyLauncher'
});

// Authenticate (Offline)
const profile = await invoke('authenticate_offline', {
  username: 'PlayerName'
});
console.log(profile); // { username: 'PlayerName', uuid: '...', accessToken: null }

// Authenticate (Microsoft)
const msProfile = await invoke('authenticate_microsoft', {
  clientId: 'your-azure-client-id'
});

// Authenticate (Azuriom)
const azProfile = await invoke('authenticate_azuriom', {
  url: 'https://your-server.com',
  username: 'player',
  password: 'password123'
});

// Launch a Minecraft version
await invoke('launch', {
  versionConfig: {
    name: 'fabric-1.21',
    loader: 'fabric',
    loaderVersion: '0.16.9',
    minecraftVersion: '1.21',
  },
  launchConfig: {
    username: profile.username,
    uuid: profile.uuid,
    javaDistribution: 'temurin',
  },
});

// Get available loaders
const loaders = await invoke('get_loaders');
console.log(loaders);
// [
//   { name: 'vanilla', displayName: 'Vanilla' },
//   { name: 'fabric', displayName: 'Fabric' },
//   ...
// ]

// Get Java distributions
const javaDistributions = await invoke('get_java_distributions');
console.log(javaDistributions);
// [
//   { name: 'temurin', displayName: 'Eclipse Temurin' },
//   { name: 'graalvm', displayName: 'GraalVM' }
// ]

// Get launcher directory path
const launcherPath = await invoke('get_launcher_path');
console.log(launcherPath); // "/home/user/.local/share/MyLauncher"

// Check if version exists
const exists = await invoke('check_version_exists', {
  versionName: 'fabric-1.21'
});
console.log(exists); // true or false
```

## Event System

With the `events` feature enabled, you can subscribe to real-time events from the launcher:

```toml
[dependencies]
lighty-launcher = { version = "0.8.2", features = ["tauri-commands", "all-loaders", "events"] }
```

```rust
use lighty_launcher::tauri::lighty_plugin;
use lighty_event::EventBus;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(lighty_plugin())
        .setup(|app| {
            // Create event bus
            let event_bus = EventBus::new(100);

            // Subscribe to events (will emit to frontend)
            subscribe_to_events(app.handle(), event_bus.clone());

            // Store event_bus in state for use in commands
            app.manage(event_bus);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error running tauri application");
}
```

### Frontend - Listening to Events

```typescript
import { listen } from '@tauri-apps/api/event';

// Listen to all launcher events
await listen('lighty-event', (event) => {
  const { eventType, data } = event.payload;

  switch(eventType) {
    case 'auth':
      console.log('Auth event:', data);
      break;
    case 'java':
      console.log('Java event:', data);
      // e.g., { type: 'DownloadProgress', current: 50, total: 100 }
      break;
    case 'launch':
      console.log('Launch event:', data);
      break;
    case 'loader':
      console.log('Loader event:', data);
      break;
    case 'core':
      console.log('Core event:', data);
      // e.g., { type: 'DownloadStarted', url: '...', file: '...' }
      break;
  }
});
```

## Available Commands

### Core Commands

#### init_app_state

Initialize the application state. **Must be called first before any other command!**

**Parameters:**
- `qualifier`: Qualifier (e.g., "com")
- `organization`: Organization name
- `application`: Application name

**Returns:** `Result<(), String>`

#### get_launcher_path

Get the launcher data directory path.

**Returns:** `String`

### Authentication Commands

#### authenticate_offline

Authenticate in offline mode (no network required).

**Parameters:**
- `username`: Player username

**Returns:** `AuthResult { username: String, uuid: String, accessToken: Option<String> }`

#### authenticate_microsoft

Authenticate using Microsoft OAuth 2.0.

**Parameters:**
- `client_id`: Azure AD application client ID

**Returns:** `AuthResult`

#### authenticate_azuriom

Authenticate using Azuriom CMS.

**Parameters:**
- `url`: Azuriom instance base URL
- `username`: Username
- `password`: Password

**Returns:** `AuthResult`

### Launch Commands

#### launch

Launch a Minecraft version.

**Parameters:**
- `version_config`: Version configuration (name, loader, loader_version, minecraft_version)
- `launch_config`: Launch configuration (username, uuid, java_distribution)

**Returns:** `LaunchResult { success: bool, message: String }`

### Java Commands

#### get_java_distributions

Get list of available Java distributions.

**Returns:** `Vec<JavaDistInfo { name: String, display_name: String }>`

### Loader Commands

#### get_loaders

Get list of available mod loaders.

**Returns:** `Vec<LoaderInfo { name: String, display_name: String }>`

### Version Commands

#### check_version_exists

Check if a version directory exists.

**Parameters:**
- `version_name`: Name of the version

**Returns:** `bool`

#### delete_version

Delete a version directory.

**Parameters:**
- `version_name`: Name of the version to delete

**Returns:** `Result<(), String>`

## Error Handling

All commands properly handle errors and return them to the frontend:

```typescript
try {
  await invoke('launch', { ... });
} catch (error) {
  console.error('Launch failed:', error);
}
```

## Full Documentation

For complete integration guide, see [TAURI_USAGE.md](https://github.com/Lighty-Launcher/LightyLauncherLib/blob/main/TAURI_USAGE.md) in the main repository.

## License

MIT

## Links

- **Main Package**: [lighty-launcher](https://crates.io/crates/lighty-launcher)
- **Repository**: [GitHub](https://github.com/Lighty-Launcher/LightyLauncherLib)
- **Documentation**: [docs.rs/lighty-tauri](https://docs.rs/lighty-tauri)
- **Tauri Guide**: [TAURI_USAGE.md](https://github.com/Lighty-Launcher/LightyLauncherLib/blob/main/TAURI_USAGE.md)
