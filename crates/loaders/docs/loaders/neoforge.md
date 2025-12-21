# NeoForge Loader

Modern fork of Forge for newer Minecraft versions (1.20.2+).

## Overview

**Status**: In Progress
**MC Versions**: 1.20.2+
**Feature Flag**: `neoforge`
**API**: NeoForged Maven

NeoForge is a modern rewrite and fork of MinecraftForge, focusing on newer Minecraft versions with improved performance and cleaner codebase.

## Current Status

⚠️ **NeoForge support is currently in development**

Basic functionality works, but some features may be incomplete or unstable.

## Usage

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
        "neoforge-1.21",
        Loader::NeoForge,
        "21.1.80",     // NeoForge version
        "1.21.1",      // Minecraft version
        launcher_dir
    );

    let metadata = instance.get_metadata().await?;

    println!("NeoForge {} loaded", metadata.id);

    Ok(())
}
```

## Exports

**In lighty_loaders**: `lighty_loaders::loaders::neoforge`
**In lighty_launcher**: `lighty_launcher::loaders::neoforge`

## API Endpoints

### Version Manifest
```
GET https://maven.neoforged.net/api/maven/versions/releases/net/neoforged/neoforge
```

### Installer
```
GET https://maven.neoforged.net/releases/net/neoforged/neoforge/{version}/neoforge-{version}-installer.jar
```

## Version Format

NeoForge uses semantic versioning:
```
{major}.{minor}.{patch}
```

Where `major` corresponds to Minecraft version:
- `21.x.x` = Minecraft 1.21.x
- `20.4.x` = Minecraft 1.20.4
- `20.2.x` = Minecraft 1.20.2

Examples:
- `21.1.80` for MC 1.21.1
- `20.4.109` for MC 1.20.4

## Query Types

- `NeoForgeBuilder` - Full metadata

## Installation Process

1. Download NeoForge installer JAR
2. Extract install profile
3. Parse version JSON
4. Process libraries
5. Return metadata

## Mod Support

NeoForge is designed for:
- Recent Forge mods (1.20.2+)
- NeoForge-specific mods
- Improved mod compatibility over Forge

## Comparison with Forge

| Feature | Forge | NeoForge |
|---------|-------|----------|
| MC Versions | 1.5+ | 1.20.2+ |
| Codebase | Legacy | Modern rewrite |
| Performance | Good | Improved |
| Mod compatibility | Extensive (old) | Modern mods |
| Development | Slow | Active |

## Known Limitations

As the implementation is in progress:
- Some metadata fields may be incomplete
- Error handling needs improvement
- Performance not fully optimized

## Future Plans

- Complete installer processing
- Add Forge compatibility layer
- Improve error messages
- Optimize caching

## Related Documentation

- [Forge](./forge.md) - Traditional Forge loader (also in progress)
- [Vanilla](./vanilla.md) - Base Minecraft
- [How to Use](../how-to-use.md) - General usage
