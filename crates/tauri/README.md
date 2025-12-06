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
    ├── core.rs             # AppState and core types
    ├── tauri_commands.rs   # Main command exports
    └── commands/           # Command implementations
        ├── mod.rs
        ├── java.rs         # Java distribution commands
        ├── launch.rs       # Launch command
        ├── loaders.rs      # Loader enumeration
        ├── path.rs         # Path utilities
        ├── version.rs      # Version checking
        └── utils/          # Utilities
            ├── mod.rs
            └── parse.rs    # Configuration parsing
```

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
lighty-launcher = { version = "0.6.3", features = ["tauri-commands"] }
# or directly
lighty-tauri = "0.6.3"
```

### Backend Setup

```rust
use lighty_tauri::tauri_commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = AppState::new(
        "com".to_string(),
        "MyLauncher".to_string(),
        "".to_string()
    );

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            launch,
            get_loaders,
            get_java_distributions,
            get_launcher_path,
            check_version_exists,
        ])
        .run(tauri::generate_context!())
        .expect("error running tauri application");
}
```

### Frontend Usage (TypeScript/JavaScript)

```typescript
import { invoke } from '@tauri-apps/api/tauri';

// Launch a Minecraft version
await invoke('launch', {
  versionConfig: {
    name: 'fabric-1.21',
    loader: 'fabric',
    loaderVersion: '0.16.9',
    minecraftVersion: '1.21',
  },
  launchConfig: {
    username: 'PlayerName',
    uuid: '00000000-0000-0000-0000-000000000000',
    javaDistribution: 'temurin',
  },
});

// Get available loaders
const loaders = await invoke('get_loaders');
console.log(loaders); // ['vanilla', 'fabric', 'quilt', 'forge', 'neoforge']

// Get Java distributions
const javaDistributions = await invoke('get_java_distributions');
console.log(javaDistributions); // ['temurin', 'graalvm']

// Get launcher directory path
const launcherPath = await invoke('get_launcher_path');
console.log(launcherPath); // "/home/user/.local/share/MyLauncher"

// Check if version exists
const exists = await invoke('check_version_exists', {
  versionName: 'fabric-1.21'
});
console.log(exists); // true or false
```

## Available Commands

### launch

Launch a Minecraft version.

**Parameters:**
- `version_config`: Version configuration (name, loader, loader version, MC version)
- `launch_config`: Launch configuration (username, UUID, Java distribution)

### get_loaders

Get list of available mod loaders.

**Returns:** `Vec<String>`

### get_java_distributions

Get list of available Java distributions.

**Returns:** `Vec<String>`

### get_launcher_path

Get the launcher directory path.

**Returns:** `String`

### check_version_exists

Check if a version directory exists.

**Parameters:**
- `version_name`: Name of the version

**Returns:** `bool`

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
