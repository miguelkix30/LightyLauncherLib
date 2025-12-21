# Loaders Guide

## Overview

This guide covers all supported mod loaders in detail, including their characteristics, usage, and API endpoints.

## Vanilla

Pure Minecraft without any modifications or mod support.

### Characteristics

- **Stability**: Stable
- **API**: Mojang official API
- **Versions**: All Minecraft versions
- **Performance**: Baseline
- **Mod Support**: None

### Usage

```rust
use lighty_loaders::{Loader, Version};
use directories::ProjectDirs;

let dirs = ProjectDirs::from("com", "MyLauncher", "").unwrap();

let version = Version::new(
    "vanilla-1.21",
    Loader::Vanilla,
    "",           // No loader version for vanilla
    "1.21",
    &dirs
);
```

### API Endpoints

**Version Manifest**:
```
https://piston-meta.mojang.com/mc/game/version_manifest_v2.json
```

**Version JSON**:
```
https://piston-meta.mojang.com/v1/packages/{sha1}/{version}.json
```

### Data Flow

1. Fetch version manifest
2. Find matching Minecraft version
3. Download version JSON
4. Parse metadata
5. Return `VersionMetaData`

### Example Metadata

```rust
use lighty_loaders::types::LoaderExtensions;

#[tokio::main]
async fn main() {
    let version = Version::new("vanilla-1.21", Loader::Vanilla, "", "1.21", &dirs);

    let metadata = version.get_metadata().await.unwrap();

    println!("Minecraft: {}", metadata.id);
    println!("Main class: {}", metadata.main_class);
    println!("Libraries: {}", metadata.libraries.len());
}
```

## Fabric

Lightweight, modular mod loader with excellent performance.

### Characteristics

- **Stability**: Stable
- **API**: FabricMC official API
- **Versions**: 1.14+ (official), 1.8+ (community)
- **Performance**: Excellent
- **Mod Support**: Large ecosystem

### Usage

```rust
let version = Version::new(
    "fabric-1.21",
    Loader::Fabric,
    "0.16.9",     // Fabric loader version
    "1.21",       // Minecraft version
    &dirs
);
```

### API Endpoints

**Loader Versions**:
```
https://meta.fabricmc.net/v2/versions/loader
```

**Loader Profile**:
```
https://meta.fabricmc.net/v2/versions/loader/{minecraft}/{loader}/profile/json
```

**Game Versions**:
```
https://meta.fabricmc.net/v2/versions/game
```

### Version Selection

```rust
use lighty_loaders::fabric::FabricQuery;
use lighty_loaders::utils::query::Query;

#[tokio::main]
async fn main() {
    // Get latest loader version for MC 1.21
    let version = Version::new("fabric-1.21", Loader::Fabric, "0.16.9", "1.21", &dirs);

    let metadata = version.get_metadata().await.unwrap();
    println!("Using Fabric {}", version.loader_version());
}
```

### Metadata Merging

Fabric metadata is merged with Vanilla:

1. Fetch Vanilla metadata for MC version
2. Fetch Fabric loader profile
3. Merge libraries (Fabric + Vanilla)
4. Override main class if specified
5. Merge JVM/game arguments
6. Return combined metadata

## Quilt

Fork of Fabric with additional features and improvements.

### Characteristics

- **Stability**: Stable
- **API**: QuiltMC official API
- **Versions**: 1.14+
- **Performance**: Excellent (similar to Fabric)
- **Mod Support**: Fabric-compatible + Quilt-specific

### Usage

```rust
let version = Version::new(
    "quilt-1.21",
    Loader::Quilt,
    "0.27.1",     // Quilt loader version
    "1.21",
    &dirs
);
```

### API Endpoints

**Loader Versions**:
```
https://meta.quiltmc.org/v3/versions/loader
```

**Loader Profile**:
```
https://meta.quiltmc.org/v3/versions/loader/{minecraft}/{loader}/profile/json
```

**Game Versions**:
```
https://meta.quiltmc.org/v3/versions/game
```

### Key Differences from Fabric

- Enhanced mod metadata system
- Improved dependency resolution
- Better backwards compatibility
- Can run most Fabric mods

### Example

```rust
use lighty_loaders::{Loader, Version};
use lighty_loaders::types::LoaderExtensions;

#[tokio::main]
async fn main() {
    let version = Version::new("quilt-1.21", Loader::Quilt, "0.27.1", "1.21", &dirs);

    let metadata = version.get_metadata().await.unwrap();

    println!("Quilt version: {}", version.loader_version());
    println!("Main class: {}", metadata.main_class);
}
```

## Forge

Traditional mod loader with extensive mod support and long history.

### Characteristics

- **Stability**: Testing (Modern), Stable (Legacy)
- **API**: MinecraftForge official API
- **Versions**: 1.5.2+ (all versions)
- **Performance**: Good
- **Mod Support**: Largest mod ecosystem

### Usage

```rust
let version = Version::new(
    "forge-1.21",
    Loader::Forge,
    "51.0.38",    // Forge version
    "1.21",
    &dirs
);
```

### API Endpoints

**Version Manifest**:
```
https://files.minecraftforge.net/net/minecraftforge/forge/maven-metadata.json
```

**Installer Profile**:
```
https://maven.minecraftforge.net/net/minecraftforge/forge/{version}/forge-{version}-installer.jar
```

### Version Formats

Forge uses different version formats:

**Modern (1.13+)**:
```
{minecraft}-{forge}
Example: 1.21-51.0.38
```

**Legacy (1.7-1.12)**:
```
{minecraft}-{forge}-{minecraft}
Example: 1.12.2-14.23.5.2860-1.12.2
```

### Installation Process

1. Download installer JAR
2. Extract install profile JSON
3. Parse version JSON from installer
4. Download processors (if needed)
5. Run post-processors
6. Merge with Vanilla metadata

### Example

```rust
use lighty_loaders::{Loader, Version};
use lighty_loaders::types::LoaderExtensions;

#[tokio::main]
async fn main() {
    let version = Version::new("forge-1.21", Loader::Forge, "51.0.38", "1.21", &dirs);

    let metadata = version.get_metadata().await.unwrap();

    println!("Forge version: {}", version.loader_version());
    println!("Libraries: {}", metadata.libraries.len());
}
```

## NeoForge

Modern fork of Forge for newer Minecraft versions (1.20.2+).

### Characteristics

- **Stability**: Testing
- **API**: NeoForged official API
- **Versions**: 1.20.2+
- **Performance**: Improved over Forge
- **Mod Support**: Forge-compatible (modern versions)

### Usage

```rust
let version = Version::new(
    "neoforge-1.21",
    Loader::NeoForge,
    "21.1.80",    // NeoForge version
    "1.21",
    &dirs
);
```

### API Endpoints

**Version Manifest**:
```
https://maven.neoforged.net/api/maven/versions/releases/net/neoforged/neoforge
```

**Installer**:
```
https://maven.neoforged.net/releases/net/neoforged/neoforge/{version}/neoforge-{version}-installer.jar
```

### Version Format

```
{major}.{minor}.{patch}
Example: 21.1.80
```

Where:
- `major` = Minecraft version (21 = 1.21)
- `minor` = Feature version
- `patch` = Bug fix version

### Key Improvements

- Cleaner codebase
- Better performance
- Modern tooling
- Active development
- Backwards compatibility with recent Forge mods

### Example

```rust
use lighty_loaders::{Loader, Version};
use lighty_loaders::types::LoaderExtensions;

#[tokio::main]
async fn main() {
    let version = Version::new("neoforge-1.21", Loader::NeoForge, "21.1.80", "1.21", &dirs);

    let metadata = version.get_metadata().await.unwrap();

    println!("NeoForge version: {}", version.loader_version());
    println!("Minecraft: {}", metadata.id);
}
```

## OptiFine

Performance and graphics optimization mod (not a mod loader).

### Characteristics

- **Stability**: Experimental
- **API**: Custom/scraped
- **Versions**: Most Minecraft versions
- **Performance**: Excellent for rendering
- **Mod Support**: Limited (not a true loader)

### Usage

```rust
let version = Version::new(
    "optifine-1.21",
    Loader::Optifine,
    "HD_U_I9",    // OptiFine version code
    "1.21",
    &dirs
);
```

### Version Format

```
HD_{type}_{version}
Examples:
- HD_U_I9  (1.21)
- HD_U_I8  (1.21)
- HD_U_H9  (1.20.4)
```

Where:
- `HD` = High Definition
- `U` = Ultra
- Letter = Sub-version
- Number = Patch

### Installation

OptiFine is typically installed:
1. As a standalone mod
2. Or merged with Forge/Fabric (via OptiFabric)

### Note

OptiFine support is experimental and may have limitations compared to other loaders.

## LightyUpdater

Custom updater system for modpack management.

### Characteristics

- **Stability**: Custom
- **API**: User-defined server
- **Versions**: Any
- **Performance**: Variable
- **Mod Support**: Modpack-specific

### Usage

```rust
let version = Version::new(
    "custom-pack",
    Loader::LightyUpdater,
    "https://myserver.com/api",  // Server URL
    "1.21",
    &dirs
);
```

### Custom Server API

Your server must provide:

**Metadata Endpoint**:
```
GET {server_url}/metadata
```

**Response Format**:
```json
{
  "id": "1.21",
  "mainClass": "net.minecraft.client.main.Main",
  "libraries": [...],
  "arguments": {...},
  ...
}
```

### Use Cases

- Custom modpacks
- Private servers
- Internal launcher systems
- Testing environments

## Loader Comparison

| Feature | Vanilla | Fabric | Quilt | Forge | NeoForge | OptiFine |
|---------|---------|--------|-------|-------|----------|----------|
| **Stability** | Stable | Stable | Stable | Testing | Testing | Experimental |
| **Performance** | Baseline | Excellent | Excellent | Good | Good | Excellent (GFX) |
| **Mod Count** | 0 | High | Medium | Highest | Medium | Limited |
| **MC Versions** | All | 1.14+ | 1.14+ | 1.5.2+ | 1.20.2+ | Most |
| **API Quality** | Official | Official | Official | Official | Official | Custom |
| **Load Time** | Fast | Fast | Fast | Slow | Medium | Fast |

## Version Compatibility

### Minecraft Version Mapping

```
1.21   -> Vanilla, Fabric 0.16+, Quilt 0.27+, Forge 51+, NeoForge 21+
1.20.4 -> Vanilla, Fabric 0.15+, Quilt 0.26+, Forge 49+, NeoForge 20.4+
1.20.1 -> Vanilla, Fabric 0.14+, Quilt 0.25+, Forge 47+, NeoForge 20.1+
1.19.4 -> Vanilla, Fabric 0.14+, Quilt 0.24+, Forge 45+
1.18.2 -> Vanilla, Fabric 0.13+, Quilt 0.17+, Forge 40+
1.16.5 -> Vanilla, Fabric 0.11+, Quilt -, Forge 36+
1.12.2 -> Vanilla, Fabric (community), Quilt -, Forge 14+
```

## Migration Guide

### Fabric to Quilt

```rust
// Before (Fabric)
let version = Version::new("my-instance", Loader::Fabric, "0.16.9", "1.21", &dirs);

// After (Quilt)
let version = Version::new("my-instance", Loader::Quilt, "0.27.1", "1.21", &dirs);
// Most Fabric mods will work without changes
```

### Forge to NeoForge

```rust
// Before (Forge 1.20.2+)
let version = Version::new("my-instance", Loader::Forge, "48.1.0", "1.20.2", &dirs);

// After (NeoForge)
let version = Version::new("my-instance", Loader::NeoForge, "20.2.88", "1.20.2", &dirs);
// Recent Forge mods should be compatible
```

## See Also

- [Overview](./overview.md) - Architecture overview
- [Caching System](./caching.md) - Cache architecture
- [Examples](./examples.md) - Complete usage examples
