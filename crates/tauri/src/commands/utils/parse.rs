use lighty_java::JavaDistribution;
use lighty_loaders::Loader;

pub(crate) fn parse_loader(loader: &str) -> Result<Loader, String> {
    match loader.to_lowercase().as_str() {
        "vanilla" => Ok(Loader::Vanilla),
        "fabric" => Ok(Loader::Fabric),
        "quilt" => Ok(Loader::Quilt),
        "neoforge" => Ok(Loader::NeoForge),
        "forge" => Ok(Loader::Forge),
        "optifine" => Ok(Loader::Optifine),
        "lighty_updater" => Ok(Loader::LightyUpdater),
        _ => Err(format!("Unknown loader: {}", loader)),
    }
}

pub(crate) fn parse_java_distribution(dist: &str) -> Result<JavaDistribution, String> {
    match dist.to_lowercase().as_str() {
        "temurin" => Ok(JavaDistribution::Temurin),
        "graalvm" => Ok(JavaDistribution::GraalVM),
        _ => Err(format!(
            "Unknown Java distribution: {}. Available: temurin, graalvm",
            dist
        )),
    }
}
