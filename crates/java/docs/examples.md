# Examples

## Basic Examples

### Download Java

```rust
use lighty_java::{JavaDistribution, jre_downloader};
use std::path::Path;

#[tokio::main]
async fn main() {
    let runtime_dir = Path::new("./runtimes");

    let java_path = jre_downloader::jre_download(
        runtime_dir,
        &JavaDistribution::Temurin,
        &21,
        |current, total| {
            println!("Progress: {}%", (current * 100) / total);
        }
    ).await.unwrap();

    println!("Java installed: {}", java_path.display());
}
```

### Run Java Program

```rust
use lighty_java::runtime::JavaRuntime;

#[tokio::main]
async fn main() {
    let java_path = "/path/to/java";

    let mut runtime = JavaRuntime::new(java_path);
    runtime
        .add_arg("-jar")
        .add_arg("application.jar");

    runtime.run(
        |line| println!("{}", line),
        |line| eprintln!("{}", line),
    ).await.unwrap();
}
```

### Detect Required Version

```rust
use lighty_java::runtime::get_jre_version;

let minecraft_version = "1.20.4";
let java_version = get_jre_version(minecraft_version);

println!("Minecraft {} requires Java {}", minecraft_version, java_version);
```

## Complete Minecraft Launcher

```rust
use lighty_java::{JavaDistribution, jre_downloader, runtime};
use std::path::Path;

#[tokio::main]
async fn main() {
    let runtime_dir = Path::new("./runtimes");
    let minecraft_version = "1.20.4";

    // 1. Detect required Java version
    let java_version = runtime::get_jre_version(minecraft_version);
    println!("Minecraft {} requires Java {}", minecraft_version, java_version);

    // 2. Check if Java is installed
    let java_path = match jre_downloader::find_java_binary(
        runtime_dir,
        &JavaDistribution::Temurin,
        &java_version
    ).await {
        Ok(path) => {
            println!("Using existing Java: {}", path.display());
            path
        }
        Err(_) => {
            println!("Downloading Java {}...", java_version);
            jre_downloader::jre_download(
                runtime_dir,
                &JavaDistribution::Temurin,
                &java_version,
                |current, total| {
                    print!("\rDownload: {}%", (current * 100) / total);
                }
            ).await.unwrap()
        }
    };

    println!("\nJava ready: {}", java_path.display());

    // 3. Launch Minecraft
    let mut runtime = runtime::JavaRuntime::new(&java_path);

    runtime
        .add_arg("-Xmx4G")
        .add_arg("-Xms1G")
        .add_arg("-Djava.library.path=natives")
        .add_arg("-cp")
        .add_arg("libraries/*:minecraft.jar")
        .add_arg("net.minecraft.client.main.Main")
        .add_arg("--username")
        .add_arg("Player")
        .add_arg("--version")
        .add_arg(minecraft_version);

    println!("Launching Minecraft...");

    runtime.run(
        |line| println!("[GAME] {}", line),
        |line| eprintln!("[ERROR] {}", line),
    ).await.unwrap();
}
```

## Multi-Distribution Support

```rust
use lighty_java::{JavaDistribution, jre_downloader};
use std::path::Path;

#[tokio::main]
async fn main() {
    let runtime_dir = Path::new("./runtimes");

    // User selects distribution
    let distribution = select_distribution();

    let java_path = jre_downloader::jre_download(
        runtime_dir,
        &distribution,
        &21,
        |current, total| {
            println!("{}%", (current * 100) / total);
        }
    ).await.unwrap();

    println!("Installed: {}", java_path.display());
}

fn select_distribution() -> JavaDistribution {
    println!("Select Java distribution:");
    println!("1. Temurin (Recommended)");
    println!("2. GraalVM (Performance)");
    println!("3. Zulu (Enterprise)");
    println!("4. Liberica (Lightweight)");

    // Read user input and return distribution
    JavaDistribution::Temurin  // Default
}
```

## Progress Bar Integration

```rust
use lighty_java::{JavaDistribution, jre_downloader};
use indicatif::{ProgressBar, ProgressStyle};

#[tokio::main]
async fn main() {
    let runtime_dir = std::path::Path::new("./runtimes");

    // Create progress bar
    let pb = ProgressBar::new(100);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .unwrap()
    );

    let java_path = jre_downloader::jre_download(
        runtime_dir,
        &JavaDistribution::Temurin,
        &21,
        |current, total| {
            pb.set_length(total);
            pb.set_position(current);
            pb.set_message(format!("{} MB", current / 1_000_000));
        }
    ).await.unwrap();

    pb.finish_with_message("Download complete!");
    println!("Java installed: {}", java_path.display());
}
```

## Event-Driven UI

```rust
use lighty_java::{JavaDistribution, jre_downloader};
use lighty_event::{EventBus, Event, JavaEvent};

#[tokio::main]
async fn main() {
    let runtime_dir = std::path::Path::new("./runtimes");

    let event_bus = EventBus::new(1000);
    let mut receiver = event_bus.subscribe();

    // Spawn UI updater
    tokio::spawn(async move {
        while let Ok(event) = receiver.next().await {
            if let Event::Java(java_event) = event {
                match java_event {
                    JavaEvent::JavaDownloadStarted { distribution, version, total_bytes } => {
                        println!("⬇️  Downloading {} {} ({} MB)",
                            distribution, version, total_bytes / 1_000_000);
                    }
                    JavaEvent::JavaDownloadProgress { bytes } => {
                        print!("\r📦 {} MB", bytes / 1_000_000);
                    }
                    JavaEvent::JavaDownloadCompleted { .. } => {
                        println!("\n✅ Download complete");
                    }
                    JavaEvent::JavaExtractionStarted { .. } => {
                        println!("📂 Extracting...");
                    }
                    JavaEvent::JavaExtractionCompleted { binary_path, .. } => {
                        println!("✅ Ready: {}", binary_path);
                    }
                    _ => {}
                }
            }
        }
    });

    let java_path = jre_downloader::jre_download(
        runtime_dir,
        &JavaDistribution::Temurin,
        &21,
        |_, _| {},
        Some(&event_bus)
    ).await.unwrap();

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    println!("Java path: {}", java_path.display());
}
```

## Concurrent Downloads

```rust
use lighty_java::{JavaDistribution, jre_downloader};
use tokio::try_join;

#[tokio::main]
async fn main() {
    let runtime_dir = std::path::Path::new("./runtimes");

    println!("Downloading multiple Java versions...");

    let (java8, java17, java21) = try_join!(
        jre_downloader::jre_download(
            runtime_dir,
            &JavaDistribution::Temurin,
            &8,
            |_, _| {}
        ),
        jre_downloader::jre_download(
            runtime_dir,
            &JavaDistribution::Temurin,
            &17,
            |_, _| {}
        ),
        jre_downloader::jre_download(
            runtime_dir,
            &JavaDistribution::Temurin,
            &21,
            |_, _| {}
        )
    ).unwrap();

    println!("Java 8: {}", java8.display());
    println!("Java 17: {}", java17.display());
    println!("Java 21: {}", java21.display());
}
```

## Version-Specific Logic

```rust
use lighty_java::{JavaDistribution, jre_downloader, runtime};

#[tokio::main]
async fn main() {
    let minecraft_version = "1.20.4";
    let runtime_dir = std::path::Path::new("./runtimes");

    // Detect Java version
    let java_version = runtime::get_jre_version(minecraft_version);

    // Select distribution based on version
    let distribution = if java_version >= 17 {
        println!("Using GraalVM for better performance");
        JavaDistribution::GraalVM
    } else {
        println!("Using Temurin for compatibility");
        JavaDistribution::Temurin
    };

    let java_path = jre_downloader::jre_download(
        runtime_dir,
        &distribution,
        &java_version,
        |_, _| {}
    ).await.unwrap();

    println!("Ready to launch Minecraft {}", minecraft_version);
    println!("Using {} Java {}", distribution.get_name(), java_version);
    println!("Java path: {}", java_path.display());
}
```

## See Also

- [Overview](./overview.md) - Architecture overview
- [Distributions](./distributions.md) - Distribution comparison
- [Installation](./installation.md) - Download process
- [Runtime](./runtime.md) - Execution details
