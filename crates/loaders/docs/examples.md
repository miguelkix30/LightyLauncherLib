# Examples

## Basic Examples

### Vanilla Minecraft

```rust
use lighty_loaders::{Loader, Version};
use lighty_loaders::types::LoaderExtensions;
use lighty_java::JavaDistribution;
use directories::ProjectDirs;

#[tokio::main]
async fn main() {
    let dirs = ProjectDirs::from("com", "MyLauncher", "").unwrap();

    let mut version = Version::new(
        "vanilla-1.21",
        Loader::Vanilla,
        "",
        "1.21",
        &dirs
    );

    // Get metadata
    let metadata = version.get_metadata().await.unwrap();
    println!("Version: {}", metadata.id);
    println!("Main class: {}", metadata.main_class);

    // Launch game
    version.launch(
        "Player",
        "uuid-here",
        JavaDistribution::Temurin
    ).await.unwrap();
}
```

### Fabric Loader

```rust
use lighty_loaders::{Loader, Version};
use lighty_loaders::types::LoaderExtensions;
use lighty_java::JavaDistribution;
use directories::ProjectDirs;

#[tokio::main]
async fn main() {
    let dirs = ProjectDirs::from("com", "MyLauncher", "").unwrap();

    let mut version = Version::new(
        "fabric-1.21",
        Loader::Fabric,
        "0.16.9",
        "1.21",
        &dirs
    );

    // Get metadata (Fabric + Vanilla merged)
    let metadata = version.get_metadata().await.unwrap();

    println!("Minecraft: {}", metadata.id);
    println!("Libraries: {}", metadata.libraries.len());

    // Launch with Fabric
    version.launch(
        "Player",
        "uuid-here",
        JavaDistribution::Temurin
    ).await.unwrap();
}
```

### Quilt Loader

```rust
use lighty_loaders::{Loader, Version};
use lighty_java::JavaDistribution;
use directories::ProjectDirs;

#[tokio::main]
async fn main() {
    let dirs = ProjectDirs::from("com", "MyLauncher", "").unwrap();

    let mut version = Version::new(
        "quilt-1.21",
        Loader::Quilt,
        "0.27.1",
        "1.21",
        &dirs
    );

    version.launch(
        "Player",
        "uuid-here",
        JavaDistribution::Temurin
    ).await.unwrap();

    println!("Quilt launched successfully!");
}
```

### Forge Loader

```rust
use lighty_loaders::{Loader, Version};
use lighty_java::JavaDistribution;
use directories::ProjectDirs;

#[tokio::main]
async fn main() {
    let dirs = ProjectDirs::from("com", "MyLauncher", "").unwrap();

    let mut version = Version::new(
        "forge-1.21",
        Loader::Forge,
        "51.0.38",
        "1.21",
        &dirs
    );

    version.launch(
        "Player",
        "uuid-here",
        JavaDistribution::Temurin
    ).await.unwrap();
}
```

### NeoForge Loader

```rust
use lighty_loaders::{Loader, Version};
use lighty_java::JavaDistribution;
use directories::ProjectDirs;

#[tokio::main]
async fn main() {
    let dirs = ProjectDirs::from("com", "MyLauncher", "").unwrap();

    let mut version = Version::new(
        "neoforge-1.21",
        Loader::NeoForge,
        "21.1.80",
        "1.21",
        &dirs
    );

    version.launch(
        "Player",
        "uuid-here",
        JavaDistribution::Temurin
    ).await.unwrap();
}
```

## Advanced Examples

### Multi-Version Support

```rust
use lighty_loaders::{Loader, Version};
use lighty_loaders::types::LoaderExtensions;
use directories::ProjectDirs;

#[tokio::main]
async fn main() {
    let dirs = ProjectDirs::from("com", "MyLauncher", "").unwrap();

    let versions = vec![
        ("vanilla-1.21", Loader::Vanilla, "", "1.21"),
        ("fabric-1.21", Loader::Fabric, "0.16.9", "1.21"),
        ("quilt-1.20.4", Loader::Quilt, "0.26.4", "1.20.4"),
        ("neoforge-1.21", Loader::NeoForge, "21.1.80", "1.21"),
    ];

    for (name, loader, loader_ver, mc_ver) in versions {
        let version = Version::new(name, loader, loader_ver, mc_ver, &dirs);

        match version.get_metadata().await {
            Ok(metadata) => {
                println!("✓ {} - {} libraries", name, metadata.libraries.len());
            }
            Err(e) => {
                eprintln!("✗ {} - Error: {:?}", name, e);
            }
        }
    }
}
```

### Version Selection Menu

```rust
use lighty_loaders::{Loader, Version};
use lighty_java::JavaDistribution;
use directories::ProjectDirs;
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    let dirs = ProjectDirs::from("com", "MyLauncher", "").unwrap();

    println!("Select Minecraft version:");
    println!("1. Vanilla 1.21");
    println!("2. Fabric 1.21");
    println!("3. Quilt 1.21");
    println!("4. NeoForge 1.21");

    print!("Choice: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut version = match input.trim() {
        "1" => Version::new("vanilla-1.21", Loader::Vanilla, "", "1.21", &dirs),
        "2" => Version::new("fabric-1.21", Loader::Fabric, "0.16.9", "1.21", &dirs),
        "3" => Version::new("quilt-1.21", Loader::Quilt, "0.27.1", "1.21", &dirs),
        "4" => Version::new("neoforge-1.21", Loader::NeoForge, "21.1.80", "1.21", &dirs),
        _ => {
            eprintln!("Invalid choice!");
            return;
        }
    };

    println!("Launching...");
    version.launch("Player", "uuid", JavaDistribution::Temurin).await.unwrap();
}
```

### Using VersionBuilder

```rust
use lighty_version::VersionBuilder;
use lighty_loaders::types::{Loader, LoaderExtensions};
use lighty_java::JavaDistribution;
use directories::ProjectDirs;

#[tokio::main]
async fn main() {
    let dirs = ProjectDirs::from("com", "MyLauncher", "").unwrap();

    let mut version = VersionBuilder::new(
        "my-modpack",
        Loader::Fabric,
        "0.16.9",
        "1.21",
        &dirs,
    );

    // Get complete metadata
    let metadata = version.get_metadata().await.unwrap();

    println!("Instance: {}", version.name());
    println!("Loader: {:?}", version.loader());
    println!("Minecraft: {}", version.minecraft_version());
    println!("Libraries: {}", metadata.libraries.len());

    // Launch the game
    version.launch("Player", "uuid", JavaDistribution::Temurin).await.unwrap();
}
```

### Metadata Inspection

```rust
use lighty_loaders::{Loader, Version};
use lighty_loaders::types::LoaderExtensions;
use directories::ProjectDirs;

#[tokio::main]
async fn main() {
    let dirs = ProjectDirs::from("com", "MyLauncher", "").unwrap();

    let version = Version::new("fabric-1.21", Loader::Fabric, "0.16.9", "1.21", &dirs);
    let metadata = version.get_metadata().await.unwrap();

    // Basic info
    println!("=== Version Info ===");
    println!("ID: {}", metadata.id);
    println!("Type: {}", metadata.type_field);
    println!("Main Class: {}", metadata.main_class);

    // Libraries
    println!("\n=== Libraries ({}) ===", metadata.libraries.len());
    for (i, lib) in metadata.libraries.iter().take(5).enumerate() {
        println!("{}. {}", i + 1, lib.name);
    }

    // Assets
    if let Some(asset_index) = &metadata.asset_index {
        println!("\n=== Assets ===");
        println!("ID: {}", asset_index.id);
        println!("Total Size: {} MB", asset_index.total_size / 1_000_000);
    }

    // Arguments
    println!("\n=== JVM Arguments ===");
    if let Some(jvm_args) = &metadata.arguments.jvm {
        println!("Count: {}", jvm_args.len());
    }

    println!("\n=== Game Arguments ===");
    if let Some(game_args) = &metadata.arguments.game {
        println!("Count: {}", game_args.len());
    }
}
```

### Error Handling

```rust
use lighty_loaders::{Loader, Version};
use lighty_loaders::types::LoaderExtensions;
use lighty_loaders::utils::error::QueryError;
use directories::ProjectDirs;

#[tokio::main]
async fn main() {
    let dirs = ProjectDirs::from("com", "MyLauncher", "").unwrap();

    let version = Version::new("fabric-1.99", Loader::Fabric, "999.0.0", "1.99", &dirs);

    match version.get_metadata().await {
        Ok(metadata) => {
            println!("Success: {}", metadata.id);
        }
        Err(QueryError::NetworkError(e)) => {
            eprintln!("Network error: {}", e);
            eprintln!("Check your internet connection");
        }
        Err(QueryError::NotFound(v)) => {
            eprintln!("Version not found: {}", v);
            eprintln!("This version may not exist");
        }
        Err(QueryError::ParseError(e)) => {
            eprintln!("Parse error: {}", e);
            eprintln!("The API response format may have changed");
        }
        Err(e) => {
            eprintln!("Unknown error: {:?}", e);
        }
    }
}
```

### Progress Tracking

```rust
use lighty_loaders::{Loader, Version};
use lighty_event::{Event, EVENT_BUS};
use lighty_java::JavaDistribution;
use directories::ProjectDirs;

#[tokio::main]
async fn main() {
    let dirs = ProjectDirs::from("com", "MyLauncher", "").unwrap();

    // Subscribe to events
    let mut receiver = EVENT_BUS.subscribe();

    tokio::spawn(async move {
        while let Ok(event) = receiver.next().await {
            match event {
                Event::DownloadStarted(e) => {
                    println!("⬇️  Downloading: {}", e.file_name);
                }
                Event::DownloadProgress(e) => {
                    let progress = (e.downloaded * 100) / e.total;
                    print!("\r📦 Progress: {}%", progress);
                }
                Event::DownloadCompleted(e) => {
                    println!("\n✅ Downloaded: {}", e.file_name);
                }
                Event::ExtractionStarted(e) => {
                    println!("📂 Extracting: {}", e.file_name);
                }
                Event::InstanceLaunched(e) => {
                    println!("🚀 Launched: {} (PID: {})", e.instance_name, e.pid);
                }
                _ => {}
            }
        }
    });

    let mut version = Version::new("fabric-1.21", Loader::Fabric, "0.16.9", "1.21", &dirs);

    version.launch("Player", "uuid", JavaDistribution::Temurin).await.unwrap();

    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
}
```

### Instance Management

```rust
use lighty_loaders::{Loader, Version};
use lighty_loaders::types::LoaderExtensions;
use lighty_launch::InstanceControl;
use lighty_java::JavaDistribution;
use directories::ProjectDirs;

#[tokio::main]
async fn main() {
    let dirs = ProjectDirs::from("com", "MyLauncher", "").unwrap();

    let mut version = Version::new("fabric-1.21", Loader::Fabric, "0.16.9", "1.21", &dirs);

    // Launch the game
    version.launch("Player", "uuid", JavaDistribution::Temurin).await.unwrap();

    // Wait a bit for the game to start
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Check if running
    if let Some(pid) = version.get_pid() {
        println!("Instance running with PID: {}", pid);

        // Calculate instance size
        let metadata = version.get_metadata().await.unwrap();
        let size = version.size_of_instance(&metadata);

        println!("Total size: {}", lighty_loaders::types::InstanceSize::format(size.total));
        println!("Libraries: {}", lighty_loaders::types::InstanceSize::format(size.libraries));
        println!("Mods: {}", lighty_loaders::types::InstanceSize::format(size.mods));

        // Close the instance
        println!("Closing instance...");
        version.close_instance(pid).await.unwrap();
    }
}
```

### Custom Loader Configuration

```rust
use lighty_loaders::{Loader, Version};
use lighty_loaders::types::LoaderExtensions;
use lighty_java::JavaDistribution;
use directories::ProjectDirs;

#[tokio::main]
async fn main() {
    let dirs = ProjectDirs::from("com", "MyLauncher", "").unwrap();

    let loader_configs = vec![
        ("survival", Loader::Vanilla, "", "1.21"),
        ("modded", Loader::Fabric, "0.16.9", "1.21"),
        ("performance", Loader::Quilt, "0.27.1", "1.21"),
        ("heavy-mods", Loader::NeoForge, "21.1.80", "1.21"),
    ];

    for (name, loader, loader_ver, mc_ver) in loader_configs {
        let version = Version::new(name, loader, loader_ver, mc_ver, &dirs);

        println!("\n=== {} ===", name);
        println!("Loader: {:?}", version.loader());
        println!("Minecraft: {}", version.minecraft_version());

        if !loader_ver.is_empty() {
            println!("Loader version: {}", version.loader_version());
        }

        // Get metadata to verify configuration
        match version.get_metadata().await {
            Ok(metadata) => {
                println!("✓ Valid configuration");
                println!("  Libraries: {}", metadata.libraries.len());
            }
            Err(e) => {
                println!("✗ Invalid configuration: {:?}", e);
            }
        }
    }
}
```

### Concurrent Version Loading

```rust
use lighty_loaders::{Loader, Version};
use lighty_loaders::types::LoaderExtensions;
use directories::ProjectDirs;
use tokio::try_join;

#[tokio::main]
async fn main() {
    let dirs = ProjectDirs::from("com", "MyLauncher", "").unwrap();

    let vanilla = Version::new("vanilla-1.21", Loader::Vanilla, "", "1.21", &dirs);
    let fabric = Version::new("fabric-1.21", Loader::Fabric, "0.16.9", "1.21", &dirs);
    let quilt = Version::new("quilt-1.21", Loader::Quilt, "0.27.1", "1.21", &dirs);

    println!("Loading 3 versions concurrently...");

    let (vanilla_meta, fabric_meta, quilt_meta) = try_join!(
        vanilla.get_metadata(),
        fabric.get_metadata(),
        quilt.get_metadata()
    ).unwrap();

    println!("Vanilla: {} libraries", vanilla_meta.libraries.len());
    println!("Fabric: {} libraries", fabric_meta.libraries.len());
    println!("Quilt: {} libraries", quilt_meta.libraries.len());
}
```

### Version Migration

```rust
use lighty_loaders::{Loader, Version};
use lighty_loaders::types::LoaderExtensions;
use directories::ProjectDirs;

#[tokio::main]
async fn main() {
    let dirs = ProjectDirs::from("com", "MyLauncher", "").unwrap();

    // Old version (Fabric)
    let old = Version::new("my-instance", Loader::Fabric, "0.15.11", "1.20.4", &dirs);
    let old_meta = old.get_metadata().await.unwrap();

    println!("=== Old Version (Fabric 1.20.4) ===");
    println!("Libraries: {}", old_meta.libraries.len());

    // New version (Quilt)
    let new = Version::new("my-instance", Loader::Quilt, "0.27.1", "1.21", &dirs);
    let new_meta = new.get_metadata().await.unwrap();

    println!("\n=== New Version (Quilt 1.21) ===");
    println!("Libraries: {}", new_meta.libraries.len());

    println!("\n=== Migration Complete ===");
    println!("Library difference: {}", new_meta.libraries.len() as i32 - old_meta.libraries.len() as i32);
}
```

### LightyUpdater Custom Server

```rust
use lighty_loaders::{Loader, Version};
use lighty_loaders::types::LoaderExtensions;
use lighty_java::JavaDistribution;
use directories::ProjectDirs;

#[tokio::main]
async fn main() {
    let dirs = ProjectDirs::from("com", "MyLauncher", "").unwrap();

    // Custom server URL in loader_version field
    let mut version = Version::new(
        "custom-modpack",
        Loader::LightyUpdater,
        "https://my-server.com/api",
        "1.21",
        &dirs
    );

    // Get metadata from custom server
    match version.get_metadata().await {
        Ok(metadata) => {
            println!("Custom modpack loaded!");
            println!("Version: {}", metadata.id);
            println!("Libraries: {}", metadata.libraries.len());

            // Launch custom modpack
            version.launch("Player", "uuid", JavaDistribution::Temurin).await.unwrap();
        }
        Err(e) => {
            eprintln!("Failed to load custom modpack: {:?}", e);
        }
    }
}
```

### Feature Flag Usage

```toml
# Cargo.toml

[dependencies]
# Only Vanilla and Fabric
lighty-loaders = { version = "0.6.3", features = ["vanilla", "fabric"] }

# All loaders
lighty-loaders = { version = "0.6.3", features = ["all-loaders"] }

# Specific loaders
lighty-loaders = { version = "0.6.3", features = ["vanilla", "fabric", "quilt", "neoforge"] }
```

```rust
use lighty_loaders::{Loader, Version};
use directories::ProjectDirs;

#[tokio::main]
async fn main() {
    let dirs = ProjectDirs::from("com", "MyLauncher", "").unwrap();

    // This works with vanilla feature
    let vanilla = Version::new("vanilla-1.21", Loader::Vanilla, "", "1.21", &dirs);

    // This works with fabric feature
    #[cfg(feature = "fabric")]
    let fabric = Version::new("fabric-1.21", Loader::Fabric, "0.16.9", "1.21", &dirs);

    // This won't compile without forge feature
    #[cfg(feature = "forge")]
    let forge = Version::new("forge-1.21", Loader::Forge, "51.0.38", "1.21", &dirs);

    println!("Loaders available based on features!");
}
```

## Complete Launcher Example

```rust
use lighty_loaders::{Loader, Version};
use lighty_loaders::types::LoaderExtensions;
use lighty_launch::InstanceControl;
use lighty_java::JavaDistribution;
use lighty_event::{Event, EVENT_BUS};
use directories::ProjectDirs;
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    let dirs = ProjectDirs::from("com", "MyLauncher", "MyAwesomeLauncher").unwrap();

    // Setup event listener
    let mut receiver = EVENT_BUS.subscribe();
    tokio::spawn(async move {
        while let Ok(event) = receiver.next().await {
            match event {
                Event::DownloadProgress(e) => {
                    let progress = (e.downloaded * 100) / e.total;
                    print!("\r📦 Downloading: {}%", progress);
                    io::stdout().flush().unwrap();
                }
                Event::DownloadCompleted(_) => println!("\n✅ Download complete"),
                Event::InstanceLaunched(e) => println!("🚀 Game launched (PID: {})", e.pid),
                Event::ConsoleOutput(e) => print!("{}", e.line),
                Event::InstanceExited(e) => {
                    println!("\n👋 Game exited with code: {:?}", e.exit_code);
                }
                _ => {}
            }
        }
    });

    // Main menu
    loop {
        println!("\n=== Minecraft Launcher ===");
        println!("1. Launch Vanilla 1.21");
        println!("2. Launch Fabric 1.21");
        println!("3. Launch Quilt 1.21");
        println!("4. Launch NeoForge 1.21");
        println!("5. View Instance Info");
        println!("6. Exit");

        print!("\nChoice: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut version = match input.trim() {
            "1" => Version::new("vanilla-1.21", Loader::Vanilla, "", "1.21", &dirs),
            "2" => Version::new("fabric-1.21", Loader::Fabric, "0.16.9", "1.21", &dirs),
            "3" => Version::new("quilt-1.21", Loader::Quilt, "0.27.1", "1.21", &dirs),
            "4" => Version::new("neoforge-1.21", Loader::NeoForge, "21.1.80", "1.21", &dirs),
            "5" => {
                show_instance_info(&dirs).await;
                continue;
            }
            "6" => break,
            _ => {
                println!("Invalid choice!");
                continue;
            }
        };

        println!("\n🔄 Preparing to launch...");

        // Get metadata
        match version.get_metadata().await {
            Ok(metadata) => {
                println!("✓ Version: {}", metadata.id);
                println!("✓ Libraries: {}", metadata.libraries.len());
            }
            Err(e) => {
                eprintln!("✗ Failed to get metadata: {:?}", e);
                continue;
            }
        }

        // Launch
        print!("\nUsername: ");
        io::stdout().flush().unwrap();

        let mut username = String::new();
        io::stdin().read_line(&mut username).unwrap();
        let username = username.trim();

        match version.launch(username, "offline-uuid", JavaDistribution::Temurin).await {
            Ok(()) => println!("✓ Launch command sent"),
            Err(e) => eprintln!("✗ Launch failed: {:?}", e),
        }

        // Wait for game to start
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        // Show running info
        if let Some(pid) = version.get_pid() {
            println!("\n📊 Instance running with PID: {}", pid);
        }
    }

    println!("\n👋 Goodbye!");
}

async fn show_instance_info(dirs: &ProjectDirs) {
    let version = Version::new("fabric-1.21", Loader::Fabric, "0.16.9", "1.21", dirs);

    match version.get_metadata().await {
        Ok(metadata) => {
            let size = version.size_of_instance(&metadata);

            println!("\n=== Instance Information ===");
            println!("Name: {}", version.name());
            println!("Minecraft: {}", version.minecraft_version());
            println!("Loader: {:?} {}", version.loader(), version.loader_version());
            println!("\n=== Disk Usage ===");
            println!("Total: {}", lighty_loaders::types::InstanceSize::format(size.total));
            println!("Libraries: {}", lighty_loaders::types::InstanceSize::format(size.libraries));
            println!("Mods: {}", lighty_loaders::types::InstanceSize::format(size.mods));
            println!("Assets: {}", lighty_loaders::types::InstanceSize::format(size.assets));
        }
        Err(e) => {
            eprintln!("Failed to get instance info: {:?}", e);
        }
    }
}
```

## See Also

- [Overview](./overview.md) - Architecture overview
- [Loaders Guide](./loaders.md) - Detailed guide for each loader
- [Caching System](./caching.md) - Cache architecture
