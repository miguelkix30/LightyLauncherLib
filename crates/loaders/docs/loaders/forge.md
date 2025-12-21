# Forge Loader

Traditional mod loader with extensive mod support and long history.

## Overview

**Status**: In Progress
**MC Versions**: 1.13+ (modern), 1.7-1.12 (legacy)
**Feature Flags**: `forge` (modern), `forge_legacy` (old versions)
**API**: MinecraftForge Maven

Forge is the original and most widely used Minecraft mod loader with the largest mod ecosystem.

## Current Status

⚠️ **Forge support is currently in development**

Both modern Forge (1.13+) and legacy Forge (1.7-1.12) are being implemented.

## Usage

### Modern Forge (1.13+)

```rust
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

    let instance = VersionBuilder::new(
        "forge-1.21",
        Loader::Forge,
        "51.0.38",     // Forge version
        "1.21.1",      // Minecraft version
        launcher_dir
    );

    // When implementation is complete:
    // let metadata = instance.get_metadata().await?;

    Ok(())
}
```

### Legacy Forge (1.7-1.12)

Requires `forge_legacy` feature flag.

```rust
let instance = VersionBuilder::new(
    "forge-1.12",
    Loader::Forge,
    "14.23.5.2860",  // Legacy Forge version
    "1.12.2",
    launcher_dir
);
```

## Exports

**In lighty_loaders**: `lighty_loaders::loaders::forge`
**In lighty_launcher**: `lighty_launcher::loaders::forge`

## API Endpoints

### Version Manifest
```
GET https://files.minecraftforge.net/net/minecraftforge/forge/maven-metadata.json
```

### Installer
```
GET https://maven.minecraftforge.net/net/minecraftforge/forge/{version}/forge-{version}-installer.jar
```

## Version Format

### Modern Forge (1.13+)
```
{minecraft}-{forge}
```
Example: `1.21.1-51.0.38`

### Legacy Forge (1.7-1.12)
```
{minecraft}-{forge}-{minecraft}
```
Example: `1.12.2-14.23.5.2860-1.12.2`

## Installation Process

1. Download Forge installer JAR
2. Extract install profile JSON
3. Parse version JSON
4. Download processors (if needed)
5. Run post-processors
6. Merge with Vanilla metadata

## Mod Ecosystem

Forge has the **largest mod ecosystem** in Minecraft:
- Thousands of mods available
- CurseForge/Modrinth support
- Extensive documentation
- Long history (2011+)

## Comparison with Other Loaders

| Feature | Forge | NeoForge | Fabric |
|---------|-------|----------|--------|
| Mod count | Highest | Growing | High |
| Performance | Good | Better | Best |
| MC support | 1.5+ | 1.20.2+ | 1.14+ |
| Complexity | High | Medium | Low |
| Load time | Slower | Medium | Fast |

## Known Limitations

As the implementation is in progress:
- Installer processing incomplete
- Post-processor execution not implemented
- Some version formats not supported

## Future Plans

- Complete installer parsing
- Implement post-processor execution
- Add legacy Forge support
- Improve version detection

## Related Documentation

- [NeoForge](./neoforge.md) - Modern Forge fork
- [Vanilla](./vanilla.md) - Base Minecraft
- [How to Use](../how-to-use.md) - General usage
