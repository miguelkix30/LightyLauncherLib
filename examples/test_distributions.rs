use lighty_launcher::JavaDistribution;
use directories::ProjectDirs;
use once_cell::sync::Lazy;

static LAUNCHER_DIRECTORY: Lazy<ProjectDirs> =
    Lazy::new(|| {
        ProjectDirs::from("fr", ".LightyLauncher", "")
            .expect("Failed to create project directories")
    });

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let distributions = vec![
        ("Temurin", JavaDistribution::Temurin),
        ("GraalVM", JavaDistribution::GraalVM),
        ("Zulu", JavaDistribution::Zulu),
        ("Liberica", JavaDistribution::Liberica),
    ];

    println!("Testing Java Distributions Download Info (Java 17):\n");

    for (name, dist) in distributions {
        print!("{:<15} ", name);

        match dist.get_download_info(&17).await {
            Ok(info) => {
                let size_str = if let Some(size) = info.file_size {
                    format!("{:.2} MB", size as f64 / 1_048_576.0)
                } else {
                    "Unknown".to_string()
                };
                println!("✅ {} ({})", &info.url[..info.url.len().min(60)], size_str);
            }
            Err(e) => {
                println!("❌ Error: {}", e);
            }
        }
    }
}
