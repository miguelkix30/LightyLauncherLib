# Installation Process

## Overview

The installation system downloads and verifies all game files required to launch Minecraft. It uses parallel downloads, SHA1 verification, and automatic retry logic.

## Installation Architecture

```
Installer Trait (lighty_launch::installer::Installer)
├─> Libraries Installation
├─> Natives Installation (download + extract)
├─> Client JAR Installation
├─> Assets Installation
└─> Mods Installation (Fabric/Quilt/NeoForge)
```

## Installation Phases

### Phase 1: Verification (Collect Tasks)

**Purpose**: Determine which files need to be downloaded

```rust
let (library_tasks, client_task, asset_tasks, mod_tasks, native_tasks) = tokio::join!(
    libraries::collect_library_tasks(self, &builder.libraries),
    client::collect_client_task(self, builder.client.as_ref()),
    assets::collect_asset_tasks(self, builder.assets.as_ref()),
    mods::collect_mod_tasks(self, builder.mods.as_deref().unwrap_or(&[])),
    natives::collect_native_tasks(self, builder.natives.as_deref().unwrap_or(&[])),
);
```

**What happens**:
- For each file type:
  1. Check if file exists on disk
  2. If exists, verify SHA1 hash
  3. If missing or hash mismatch → add to task list
  4. If valid → skip

**Example task**:
```rust
pub struct DownloadTask {
    pub url: String,
    pub path: PathBuf,
    pub sha1: Option<String>,
    pub size: Option<u64>,
}
```

### Phase 2: Decision

**Skip installation** if all files are valid:
```rust
if total_downloads == 0 {
    // Emit IsInstalled event
    // Extract natives (always required)
    // Return early
}
```

**Proceed with installation** if files need downloading:
```rust
// Emit InstallStarted event
// Execute parallel downloads
// Emit InstallCompleted event
```

### Phase 3: Parallel Download

All downloads happen concurrently:
```rust
tokio::try_join!(
    libraries::download_libraries(library_tasks, event_bus),
    natives::download_and_extract_natives(self, native_tasks, native_paths, event_bus),
    mods::download_mods(mod_tasks, event_bus),
    client::download_client(client_task, event_bus),
    assets::download_assets(asset_tasks, event_bus),
)?;
```

## Installation Components

### 1. Libraries

**Purpose**: Java dependencies (JARs) required by the game

**Location**: `{game_dir}/libraries/`

**Structure**:
```
libraries/
├── com/mojang/logging/1.0.0/logging-1.0.0.jar
├── org/lwjgl/lwjgl/3.3.3/lwjgl-3.3.3.jar
├── org/lwjgl/lwjgl-glfw/3.3.3/lwjgl-glfw-3.3.3.jar
└── net/fabricmc/fabric-loader/0.16.9/fabric-loader-0.16.9.jar
```

**Metadata example**:
```json
{
  "name": "org.lwjgl:lwjgl:3.3.3",
  "url": "https://libraries.minecraft.net/org/lwjgl/lwjgl/3.3.3/lwjgl-3.3.3.jar",
  "sha1": "4158d7bf99b95428c5e8a8eb8d5d31e2f3f5c6a1",
  "size": 746293
}
```

**Download process**:
```rust
async fn download_libraries(
    tasks: Vec<(String, PathBuf)>,
    event_bus: Option<&EventBus>,
) -> InstallerResult<()> {
    if tasks.is_empty() {
        return Ok(());
    }

    // Emit DownloadingLibraries event
    event_bus.emit(LaunchEvent::DownloadingLibraries {
        current: 0,
        total: tasks.len(),
    });

    // Download with progress tracking
    for (index, (url, path)) in tasks.iter().enumerate() {
        download_file(url, path).await?;

        event_bus.emit(LaunchEvent::DownloadingLibraries {
            current: index + 1,
            total: tasks.len(),
        });
    }

    Ok(())
}
```

**Typical count**: 100-300 libraries depending on loader

### 2. Natives

**Purpose**: Platform-specific native binaries (LWJGL, OpenAL, etc.)

**Location**:
- Downloaded to: `{game_dir}/natives/`
- Extracted to: `{temp}/natives-{timestamp}/`

**Platform-specific**:
```
Windows:  lwjgl-3.3.3-natives-windows.jar
Linux:    lwjgl-3.3.3-natives-linux.jar
macOS:    lwjgl-3.3.3-natives-macos.jar
```

**Metadata example**:
```json
{
  "name": "org.lwjgl:lwjgl:3.3.3:natives-windows",
  "url": "https://libraries.minecraft.net/org/lwjgl/lwjgl/3.3.3/lwjgl-3.3.3-natives-windows.jar",
  "sha1": "2b6166b5c1bc8b0c5e5c4b8e8f5e6a1c9d8e7f6a",
  "size": 142857,
  "extract": {
    "exclude": ["META-INF/"]
  }
}
```

**Download and extraction**:
```rust
async fn download_and_extract_natives(
    version: &impl VersionInfo,
    download_tasks: Vec<(String, PathBuf)>,
    extract_paths: Vec<PathBuf>,
    event_bus: Option<&EventBus>,
) -> InstallerResult<()> {
    // 1. Download native JARs if needed
    if !download_tasks.is_empty() {
        for (url, path) in download_tasks {
            download_file(&url, &path).await?;
        }
    }

    // 2. Extract natives to temporary directory
    let natives_dir = create_natives_temp_dir();

    for jar_path in extract_paths {
        extract_jar_excluding(&jar_path, &natives_dir, &["META-INF/"]).await?;
    }

    Ok(())
}
```

**Why extract every time?**
- Ensures clean state
- Prevents conflicts between runs
- Handles platform-specific libraries correctly

**Extraction rules**:
- Extract all `.dll` (Windows), `.so` (Linux), `.dylib` (macOS) files
- Exclude `META-INF/` directory (metadata, not needed)
- Flatten directory structure (all files in root)

**Typical count**: 5-15 native libraries

### 3. Client JAR

**Purpose**: Main Minecraft executable

**Location**: `{game_dir}/versions/{version}/{version}.jar`

**Example**:
```
versions/1.21.1/1.21.1.jar
```

**Metadata**:
```json
{
  "url": "https://piston-data.mojang.com/v1/objects/59353fb40c36d304f2035d51e7d6e6baa98dc05c/client.jar",
  "sha1": "59353fb40c36d304f2035d51e7d6e6baa98dc05c",
  "size": 26354187
}
```

**Download process**:
```rust
async fn download_client(
    task: Option<(String, PathBuf)>,
    event_bus: Option<&EventBus>,
) -> InstallerResult<()> {
    if let Some((url, path)) = task {
        event_bus.emit(LaunchEvent::DownloadingClient {
            version: version_name.clone(),
        });

        download_file(&url, &path).await?;

        event_bus.emit(LaunchEvent::ClientDownloaded {
            version: version_name,
        });
    }
    Ok(())
}
```

**Size**: Typically 20-30 MB

### 4. Assets

**Purpose**: Game resources (textures, sounds, language files)

**Location**: `{game_dir}/assets/objects/`

**Structure** (hash-based):
```
assets/
├── indexes/
│   └── 16.json              # Asset index
└── objects/
    ├── 00/
    │   └── 001234abcd...    # Hashed asset file
    ├── 01/
    │   └── 015678efgh...
    └── ff/
        └── ffabcdef01...
```

**Asset index example** (`assets/indexes/16.json`):
```json
{
  "objects": {
    "minecraft/sounds/ambient/cave/cave1.ogg": {
      "hash": "f8c4b5e6a1d2c3b4a5e6f7d8c9b0a1e2f3d4c5b6",
      "size": 18357
    },
    "minecraft/textures/block/stone.png": {
      "hash": "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0",
      "size": 2048
    }
  }
}
```

**Download process**:
```rust
async fn download_assets(
    tasks: Vec<(String, PathBuf)>,
    event_bus: Option<&EventBus>,
) -> InstallerResult<()> {
    if tasks.is_empty() {
        return Ok(());
    }

    event_bus.emit(LaunchEvent::DownloadingAssets {
        current: 0,
        total: tasks.len(),
    });

    // Download in batches for better performance
    let batch_size = 50;
    for (batch_index, batch) in tasks.chunks(batch_size).enumerate() {
        let downloads: Vec<_> = batch
            .iter()
            .map(|(url, path)| download_file(url, path))
            .collect();

        try_join_all(downloads).await?;

        event_bus.emit(LaunchEvent::DownloadingAssets {
            current: (batch_index + 1) * batch_size,
            total: tasks.len(),
        });
    }

    Ok(())
}
```

**Asset URL format**:
```
https://resources.download.minecraft.net/{hash[0:2]}/{hash}
```

**Example**:
```
Hash: f8c4b5e6a1d2c3b4a5e6f7d8c9b0a1e2f3d4c5b6
URL:  https://resources.download.minecraft.net/f8/f8c4b5e6a1d2c3b4a5e6f7d8c9b0a1e2f3d4c5b6
Path: assets/objects/f8/f8c4b5e6a1d2c3b4a5e6f7d8c9b0a1e2f3d4c5b6
```

**Typical count**: 3,000-10,000 assets

### 5. Mods

**Purpose**: Modifications for Fabric/Quilt/NeoForge

**Location**: `{game_dir}/mods/`

**Structure**:
```
mods/
├── fabric-api-0.100.0+1.21.jar
├── sodium-fabric-mc1.21-0.5.8.jar
└── iris-mc1.21-1.7.0.jar
```

**Metadata** (from LightyUpdater server):
```json
{
  "name": "fabric-api",
  "url": "https://server.com/mods/fabric-api-0.100.0.jar",
  "sha1": "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0",
  "size": 2450123,
  "enabled": true
}
```

**Download process**:
```rust
async fn download_mods(
    tasks: Vec<(String, PathBuf)>,
    event_bus: Option<&EventBus>,
) -> InstallerResult<()> {
    if tasks.is_empty() {
        return Ok(());
    }

    event_bus.emit(LaunchEvent::DownloadingMods {
        current: 0,
        total: tasks.len(),
    });

    for (index, (url, path)) in tasks.iter().enumerate() {
        download_file(url, path).await?;

        event_bus.emit(LaunchEvent::DownloadingMods {
            current: index + 1,
            total: tasks.len(),
        });
    }

    Ok(())
}
```

**Mod management**:
- Disabled mods: Add `.disabled` suffix
- Remove old mods: Delete unlisted files
- Update mods: Replace if SHA1 mismatch

**Typical count**: 10-200 mods depending on modpack

## SHA1 Verification

**Purpose**: Ensure file integrity and avoid re-downloading

```rust
async fn verify_sha1(path: &Path, expected: &str) -> bool {
    use sha1::{Sha1, Digest};

    let mut file = match File::open(path).await {
        Ok(f) => f,
        Err(_) => return false,
    };

    let mut hasher = Sha1::new();
    let mut buffer = vec![0u8; 8192];

    loop {
        let n = match file.read(&mut buffer).await {
            Ok(0) => break,
            Ok(n) => n,
            Err(_) => return false,
        };

        hasher.update(&buffer[..n]);
    }

    let hash = format!("{:x}", hasher.finalize());
    hash == expected
}
```

**When used**:
- Before download: Skip if file exists and SHA1 matches
- After download: Verify downloaded file (optional, based on metadata availability)

**Benefits**:
- Saves bandwidth (skip already-downloaded files)
- Ensures file integrity
- Detects corrupted downloads

## Download Implementation

### File Download with Retry

```rust
async fn download_file(url: &str, path: &Path) -> InstallerResult<()> {
    const MAX_RETRIES: u32 = 3;

    for attempt in 1..=MAX_RETRIES {
        match try_download(url, path).await {
            Ok(_) => return Ok(()),
            Err(e) if attempt < MAX_RETRIES => {
                lighty_core::trace_warn!(
                    "Download failed (attempt {}/{}): {}",
                    attempt, MAX_RETRIES, e
                );
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
            Err(e) => return Err(InstallerError::DownloadFailed(e.to_string())),
        }
    }

    unreachable!()
}

async fn try_download(url: &str, path: &Path) -> Result<(), Box<dyn Error>> {
    // Create parent directory
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }

    // Download file
    let response = reqwest::get(url).await?.error_for_status()?;
    let bytes = response.bytes().await?;

    // Write to disk
    tokio::fs::write(path, bytes).await?;

    Ok(())
}
```

### Progress Tracking

With events feature:
```rust
#[cfg(feature = "events")]
pub async fn download_with_progress(
    url: &str,
    path: &Path,
    event_bus: &EventBus,
) -> InstallerResult<()> {
    let response = reqwest::get(url).await?.error_for_status()?;
    let total_size = response.content_length().unwrap_or(0);

    let mut file = File::create(path).await?;
    let mut downloaded = 0u64;
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk).await?;

        downloaded += chunk.len() as u64;

        event_bus.emit(Event::Launch(LaunchEvent::DownloadProgress {
            url: url.to_string(),
            current: downloaded,
            total: total_size,
        }));
    }

    Ok(())
}
```

## Directory Creation

Directories are created on-demand:
```rust
async fn create_directories(version: &impl VersionInfo) {
    let parent_path = version.game_dirs().to_path_buf();

    mkdir!(parent_path.join("libraries"));
    mkdir!(parent_path.join("natives"));
    mkdir!(parent_path.join("assets").join("objects"));
}
```

**Created directories**:
```
{game_dir}/
├── libraries/          # Java JAR libraries
├── natives/            # Native library downloads
├── assets/
│   ├── indexes/        # Asset index files
│   └── objects/        # Hashed asset files
├── mods/              # Mod files (if applicable)
├── versions/          # Client JAR
├── saves/             # World saves (created by game)
├── resourcepacks/     # Resource packs (created by game)
└── screenshots/       # Screenshots (created by game)
```

## Events Emitted

### Installation Events

```rust
LaunchEvent::InstallStarted {
    version: String,
    total_bytes: u64,
}

LaunchEvent::IsInstalled {
    version: String,
}

LaunchEvent::DownloadingLibraries {
    current: usize,
    total: usize,
}

LaunchEvent::DownloadingNatives {
    current: usize,
    total: usize,
}

LaunchEvent::DownloadingClient {
    version: String,
}

LaunchEvent::ClientDownloaded {
    version: String,
}

LaunchEvent::DownloadingAssets {
    current: usize,
    total: usize,
}

LaunchEvent::DownloadingMods {
    current: usize,
    total: usize,
}

LaunchEvent::InstallCompleted {
    version: String,
    total_bytes: u64,
}
```

## Performance Characteristics

### Parallel Downloads
- **Libraries**: Downloaded sequentially (100-300 files, ~50-100 MB total)
- **Natives**: Downloaded sequentially (5-15 files, ~5-10 MB total)
- **Client**: Single file (~20-30 MB)
- **Assets**: Downloaded in batches of 50 (3000-10000 files, ~200-500 MB total)
- **Mods**: Downloaded sequentially (10-200 files, variable size)

All categories run in parallel using `tokio::try_join!`.

### Optimization Strategies

1. **Skip verified files**: SHA1 check before download
2. **Batch asset downloads**: 50 assets per batch
3. **Concurrent categories**: All types download simultaneously
4. **Automatic retry**: 3 attempts per file
5. **Temp directory for natives**: Clean state per launch

## Error Handling

```rust
pub enum InstallerError {
    DownloadFailed(String),
    VerificationFailed(String),
    ExtractionFailed(String),
    IOError(std::io::Error),
}
```

**Example**:
```rust
match version.install(metadata, event_bus).await {
    Ok(_) => println!("Installation complete"),
    Err(InstallerError::DownloadFailed(url)) => {
        eprintln!("Failed to download: {}", url);
    }
    Err(InstallerError::VerificationFailed(file)) => {
        eprintln!("Verification failed: {}", file);
    }
    Err(e) => eprintln!("Installation error: {}", e),
}
```

## Complete Example

```rust
use lighty_core::AppState;
use lighty_launcher::prelude::*;
use lighty_launch::Installer;

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
        "fabric-1.21",
        Loader::Fabric,
        "0.16.9",
        "1.21.1",
        launcher_dir
    );

    // Get metadata
    let metadata = instance.get_metadata().await?;
    let version = match metadata.as_ref() {
        VersionMetaData::Version(v) => v,
        _ => return Err(anyhow::anyhow!("Invalid metadata")),
    };

    // Install all dependencies
    instance.install(version, None).await?;

    println!("Installation complete!");

    Ok(())
}
```

## Related Documentation

- [Launch Process](./launch.md) - Complete launch flow
- [Events](./events.md) - Event types
- [How to Use](./how-to-use.md) - Practical examples
