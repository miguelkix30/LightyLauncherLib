use std::env;
use std::time::Duration;
use once_cell::sync::Lazy;
use reqwest::Client;
use tokio::fs;
use thiserror::Error;

/// Global User-Agent for the launcher (commented placeholder).
///static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

/// Shared HTTP client tuned for parallel asset/library downloads.
///
/// Tuned for the high concurrency of installer workloads: large connection
/// pool, HTTP/2 window scaling, TCP keepalive, and aggressive compression.
pub static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        // Connection pooling - balance between performance and OS limits
        .pool_max_idle_per_host(100)
        .pool_idle_timeout(Some(Duration::from_secs(90)))

        // HTTP/2 optimisations
        .http2_initial_stream_window_size(Some(2 * 1024 * 1024))
        .http2_initial_connection_window_size(Some(4 * 1024 * 1024))
        .http2_adaptive_window(true)
        .http2_max_frame_size(Some(16 * 1024))

        // TCP optimisations
        .tcp_keepalive(Some(Duration::from_secs(60)))
        .tcp_nodelay(true)

        // Timeouts - prevent stuck connections
        .timeout(Duration::from_secs(60))
        .connect_timeout(Duration::from_secs(5))

        // Compression
        .zstd(true)
        .gzip(true)
        .brotli(true)

        .build()
        .expect("Failed to build HTTP client with default configuration - this should never fail")
});


/// Path to the system hosts file (OS-dependent).
#[cfg(target_os = "windows")]
const HOSTS_PATH: &str = "System32\\drivers\\etc\\hosts";

#[cfg(not(target_os = "windows"))]
const HOSTS_PATH: &str = "etc/hosts";

/// Auth-critical domains we refuse to see redirected.
///
/// Malware-modified hosts files commonly redirect these to bypass
/// Mojang authentication or impersonate the launcher's update server.
const HOSTS: [&str; 3] = [
    "mojang.com",
    "minecraft.net",
    "lightylauncher.fr",
];

/// Errors related to the hosts file check.
#[derive(Debug, Error)]
pub enum HostsError {
    #[error("Failed to read hosts file at {0}")]
    HostsReadError(String),

    #[error("Hosts file contains blocked entries: {0}")]
    HostsBlocked(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type HostsResult<T> = std::result::Result<T, HostsError>;

/// Checks whether the system hosts file redirects any auth-critical domains.
///
/// Returns `Err(HostsError::HostsBlocked)` if any line in `/etc/hosts`
/// (or the Windows equivalent) maps one of the [`HOSTS`] entries to a
/// non-standard IP. Comment lines are ignored.
pub async fn check_hosts_file() -> HostsResult<()> {
    let hosts_path = if cfg!(target_os = "windows") {
        let system_drive = env::var("SystemDrive").unwrap_or("C:".to_string());
        format!("{}\\{}", system_drive, HOSTS_PATH)
    } else {
        format!("/{}", HOSTS_PATH)
    };

    if !fs::try_exists(&hosts_path).await? {
        return Ok(());
    }

    let hosts_file = fs::read_to_string(&hosts_path)
        .await
        .map_err(|_| HostsError::HostsReadError(hosts_path.clone()))?;

    let flagged_entries: Vec<_> = hosts_file
        .lines()
        .filter(|line| !line.trim_start().starts_with('#'))
        .flat_map(|line| {
            let mut parts = line.split_whitespace();
            let _ip = parts.next();
            parts.filter(|domain| HOSTS.iter().any(|&entry| domain.contains(entry)))
        })
        .map(|s| s.to_string())
        .collect();

    if !flagged_entries.is_empty() {
        return Err(HostsError::HostsBlocked(flagged_entries.join("\n")));
    }

    Ok(())
}