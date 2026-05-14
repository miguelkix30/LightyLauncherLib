use std::path::Path;

/// Generic view of an installable instance.
///
/// Used by `ManifestRepository` to support different builder types
/// (`VersionBuilder`, `LightyVersionBuilder`, etc.) under a single interface.
///
/// # Examples
///
/// ```rust
/// use lighty_loaders::types::VersionInfo;
/// use lighty_version::VersionBuilder;
/// use lighty_loaders::types::Loader;
///
/// fn print_version_info<V: VersionInfo>(version: &V) {
///     println!("Name: {}", version.name());
///     println!("Minecraft: {}", version.minecraft_version());
///     println!("Game dir: {}", version.game_dirs().display());
/// }
/// ```
pub trait VersionInfo: Clone + Send + Sync {
    /// Loader type (Vanilla, Fabric, NeoForge, etc.).
    type LoaderType: Clone + Send + Sync + std::fmt::Debug;

    /// Instance name (unique profile identifier).
    fn name(&self) -> &str;

    /// Loader version (or server URL for `LightyVersionBuilder`).
    ///
    /// Examples: `"0.15.0"` for Fabric, `"21.0.0"` for NeoForge.
    fn loader_version(&self) -> &str;

    /// Minecraft version.
    ///
    /// Examples: `"1.20.1"`, `"1.19.4"`.
    fn minecraft_version(&self) -> &str;

    /// Game directory (holds assets, libraries, versions, etc.).
    fn game_dirs(&self) -> &Path;

    /// Java directory (holds JRE installations).
    fn java_dirs(&self) -> &Path;

    /// Returns the loader.
    fn loader(&self) -> &Self::LoaderType;

    // === Utility methods with default implementations ===

    /// Returns whether the game directory exists on disk.
    fn game_dir_exists(&self) -> bool {
        self.game_dirs().exists()
    }

    /// Returns whether the Java directory exists on disk.
    fn java_dir_exists(&self) -> bool {
        self.java_dirs().exists()
    }

    /// Returns a fully qualified version identifier.
    ///
    /// Format: `{name}-{minecraft_version}-{loader_version}`.
    fn full_identifier(&self) -> String {
        format!(
            "{}-{}-{}",
            self.name(),
            self.minecraft_version(),
            self.loader_version()
        )
    }

    /// Returns the (game_dir, java_dir) tuple — useful for logging.
    fn paths(&self) -> (&Path, &Path) {
        (self.game_dirs(), self.java_dirs())
    }

    /// Returns whether the instance is installed.
    ///
    /// An instance is considered installed when its game directory exists.
    fn is_installed(&self) -> bool {
        self.game_dirs().exists()
    }
}
