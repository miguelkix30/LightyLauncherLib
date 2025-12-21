# LightyUpdater

Custom loader system for modpack management with server-defined metadata.

## Overview

**Status**: Stable
**MC Versions**: Any (server-defined)
**Feature Flag**: `lighty_updater`
**API**: Custom server (user-defined)

LightyUpdater allows you to define custom version metadata on your own server, perfect for modpacks and custom distributions.

## Usage

### With LightyVersionBuilder

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

    // LightyVersionBuilder for custom servers
    let instance = LightyVersionBuilder::new(
        "my-modpack",                    // Instance name
        "https://myserver.com/api",      // Server URL
        "1.21.1",                        // Minecraft version
        launcher_dir
    );

    let metadata = instance.get_metadata().await?;

    println!("Custom modpack loaded: {}", metadata.id);

    Ok(())
}
```

### With VersionBuilder

```rust
let instance = VersionBuilder::new(
    "my-modpack",
    Loader::LightyUpdater,
    "https://myserver.com/api",  // Server URL in loader_version field
    "1.21.1",
    launcher_dir
);
```

## Exports

**In lighty_loaders**: `lighty_loaders::loaders::lighty_updater`
**In lighty_launcher**: `lighty_launcher::loaders::lighty_updater`

**Builder**: `lighty_version::LightyVersionBuilder`
**Re-export**: `lighty_launcher::version::LightyVersionBuilder`

## Server API Requirements

Your server must provide metadata at the specified URL.

### Endpoint

```
GET {server_url}/metadata?mc_version={minecraft_version}
```

Example: `https://myserver.com/api/metadata?mc_version=1.21.1`

### Response Format

Return `VersionMetaData` compatible JSON:

```json
{
  "id": "1.21.1",
  "type": "release",
  "mainClass": "net.minecraft.client.main.Main",
  "libraries": [
    {
      "name": "com.example:my-mod:1.0.0",
      "url": "https://myserver.com/mods/my-mod-1.0.0.jar",
      "sha1": "abc123...",
      "size": 1234567
    }
  ],
  "arguments": {
    "jvm": ["-Xmx4G"],
    "game": ["--username", "${auth_player_name}"]
  },
  "assetIndex": {
    "id": "16",
    "url": "https://piston-data.mojang.com/...",
    "totalSize": 500000000
  }
}
```

## Merging with Vanilla

LightyUpdater can optionally merge with Vanilla:

```rust
// Server returns additional libraries/args
// LightyUpdater merges them with Vanilla base
```

This is configured server-side.

## Use Cases

- **Custom modpacks**: Centralized modpack management
- **Private servers**: Internal launcher with custom mods
- **Testing environments**: Development versions
- **Curated experiences**: Specific mod combinations

## Complete Documentation

For detailed server implementation and advanced features, see:

📚 **[LightyUpdater Repository](https://github.com/Lighty-Launcher/LightyUpdater)**

## Example Server Implementation

Minimal server example (pseudo-code):

```javascript
// Express.js example
app.get('/metadata', async (req, res) => {
  const mcVersion = req.query.mc_version;

  const metadata = {
    id: mcVersion,
    mainClass: "net.minecraft.client.main.Main",
    libraries: [
      // Your custom mods
      {
        name: "com.mymodpack:core:1.0.0",
        url: "https://myserver.com/mods/core-1.0.0.jar",
        sha1: await calculateSha1("core-1.0.0.jar"),
        size: getFileSize("core-1.0.0.jar")
      }
    ],
    arguments: {
      jvm: ["-Xmx4G", "-Xms2G"],
      game: []
    }
  };

  res.json(metadata);
});
```

## Security Considerations

- **HTTPS required**: Use HTTPS for your server
- **SHA1 verification**: Always provide SHA1 hashes
- **Authentication**: Consider adding API keys
- **Rate limiting**: Protect against abuse

## Related Documentation

- [How to Use](../how-to-use.md) - Usage guide
- [Query System](../query.md) - Custom implementation
- [LightyUpdater GitHub](https://github.com/Lighty-Launcher/LightyUpdater) - Full docs
