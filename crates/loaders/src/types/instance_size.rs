use serde::{Deserialize, Serialize};

/// Represents the size breakdown of a Minecraft instance
///
/// This struct provides detailed information about the disk space
/// occupied by different components of a Minecraft instance.
///
/// # Examples
/// ```
/// use lighty_loaders::types::InstanceSize;
///
/// let size = InstanceSize {
///     libraries: 50_000_000,
///     mods: 100_000_000,
///     natives: 5_000_000,
///     client: 20_000_000,
///     assets: 300_000_000,
///     total: 475_000_000,
/// };
///
/// println!("Total: {}", size.total_human());
/// println!("Libraries: {}", size.libraries_human());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceSize {
    /// Size of library files in bytes
    pub libraries: u64,
    /// Size of mod files in bytes
    pub mods: u64,
    /// Size of native library files in bytes
    pub natives: u64,
    /// Size of client JAR in bytes
    pub client: u64,
    /// Size of asset files in bytes
    pub assets: u64,
    /// Total size in bytes
    pub total: u64,
}

impl InstanceSize {
    /// Get total size in megabytes
    ///
    /// # Examples
    /// ```
    /// # use lighty_loaders::types::InstanceSize;
    /// let size = InstanceSize {
    ///     libraries: 0, mods: 0, natives: 0, client: 0, assets: 0,
    ///     total: 52_428_800, // 50 MB
    /// };
    /// assert_eq!(size.total_mb(), 50.0);
    /// ```
    pub fn total_mb(&self) -> f64 {
        self.total as f64 / 1024.0 / 1024.0
    }

    /// Get total size in gigabytes
    ///
    /// # Examples
    /// ```
    /// # use lighty_loaders::types::InstanceSize;
    /// let size = InstanceSize {
    ///     libraries: 0, mods: 0, natives: 0, client: 0, assets: 0,
    ///     total: 1_073_741_824, // 1 GB
    /// };
    /// assert_eq!(size.total_gb(), 1.0);
    /// ```
    pub fn total_gb(&self) -> f64 {
        self.total as f64 / 1024.0 / 1024.0 / 1024.0
    }

    /// Format bytes as a string with appropriate unit (B, KB, MB, GB)
    ///
    /// # Examples
    /// ```
    /// # use lighty_loaders::types::InstanceSize;
    /// assert_eq!(InstanceSize::format(1024), "1.00 KB");
    /// assert_eq!(InstanceSize::format(1048576), "1.00 MB");
    /// assert_eq!(InstanceSize::format(1073741824), "1.00 GB");
    /// assert_eq!(InstanceSize::format(500), "500 B");
    /// ```
    pub fn format(bytes: u64) -> String {
        if bytes < 1024 {
            format!("{} B", bytes)
        } else if bytes < 1024 * 1024 {
            format!("{:.2} KB", bytes as f64 / 1024.0)
        } else if bytes < 1024 * 1024 * 1024 {
            format!("{:.2} MB", bytes as f64 / 1024.0 / 1024.0)
        } else {
            format!("{:.2} GB", bytes as f64 / 1024.0 / 1024.0 / 1024.0)
        }
    }
}
