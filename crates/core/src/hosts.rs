use std::env;
use std::time::Duration;
use once_cell::sync::Lazy;
use reqwest::{Client, Url};
use tokio::fs;
use thiserror::Error;

/// User-Agent global pour ton launcher
///static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

/// HTTP client unique et optimisé
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

fn env_base(var: &str) -> Option<String> {
    env::var(var)
        .ok()
        .map(|value| value.trim().trim_end_matches('/').to_string())
        .filter(|value| !value.is_empty())
}

fn join_base_and_path(base: &str, path: &str) -> String {
    if path.starts_with('/') {
        format!("{}{}", base, path)
    } else {
        format!("{}/{}", base, path)
    }
}

fn push_fastmcmirror(urls: &mut Vec<String>, base: &str, path: &str) {
    if !path.is_empty() {
        urls.push(join_base_and_path(base, path));
    }
}

pub fn build_fallback_urls(original: &str) -> Vec<String> {
    let mut urls = vec![original.to_string()];

    let Ok(parsed) = Url::parse(original) else {
        return urls;
    };

    let host = parsed.host_str().unwrap_or_default();
    let path = parsed.path();

    if host.eq_ignore_ascii_case("resources.download.minecraft.net") && !path.is_empty() {
        if let Some(base) = env_base("LIGHTY_MIRROR_MOJANG_ASSETS") {
            urls.push(join_base_and_path(&base, path));
        }
        push_fastmcmirror(&mut urls, "https://resources.fastmcmirror.org", path);
    }

    if host.eq_ignore_ascii_case("piston-meta.mojang.com") && !path.is_empty() {
        if let Some(base) = env_base("LIGHTY_MIRROR_PISTON_META") {
            urls.push(join_base_and_path(&base, path));
        }
        push_fastmcmirror(&mut urls, "https://launchermeta.fastmcmirror.org", path);
    }

    if host.eq_ignore_ascii_case("launchermeta.mojang.com") && !path.is_empty() {
        push_fastmcmirror(&mut urls, "https://launchermeta.fastmcmirror.org", path);
    }

    if host.eq_ignore_ascii_case("libraries.minecraft.net") && !path.is_empty() {
        push_fastmcmirror(&mut urls, "https://libraries.fastmcmirror.org", path);
    }

    if host.eq_ignore_ascii_case("meta.fabricmc.net") && !path.is_empty() {
        if let Some(base) = env_base("LIGHTY_MIRROR_FABRIC_META") {
            urls.push(join_base_and_path(&base, path));
        }
        push_fastmcmirror(&mut urls, "https://fabricmeta.fastmcmirror.org", path);
    }

    if host.eq_ignore_ascii_case("maven.fabricmc.net") && !path.is_empty() {
        if let Some(base) = env_base("LIGHTY_MIRROR_FABRIC_MAVEN") {
            urls.push(join_base_and_path(&base, path));
        }
        push_fastmcmirror(&mut urls, "https://fabric.fastmcmirror.org", path);
    }

    if host.eq_ignore_ascii_case("maven.minecraftforge.net") && !path.is_empty() {
        if let Some(base) = env_base("LIGHTY_MIRROR_FORGE_MAVEN") {
            urls.push(join_base_and_path(&base, path));
        }
        push_fastmcmirror(&mut urls, "https://forge.fastmcmirror.org", path);
    }

    if host.eq_ignore_ascii_case("maven.neoforged.net") && !path.is_empty() {
        if let Some(base) = env_base("LIGHTY_MIRROR_NEOFORGE_MAVEN") {
            urls.push(join_base_and_path(&base, path));
        }
    }

    urls
}


/// Chemin du fichier hosts (dépend de l'OS)
#[cfg(target_os = "windows")]
const HOSTS_PATH: &str = "System32\\drivers\\etc\\hosts";

#[cfg(not(target_os = "windows"))]
const HOSTS_PATH: &str = "etc/hosts";

/// Domaines critiques à vérifier
const HOSTS: [&str; 3] = [
    "mojang.com",
    "minecraft.net",
    "lightylauncher.fr",
];

/// Erreurs possibles liées au fichier hosts
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

/// Vérifie si le fichier hosts a été modifié pour bloquer l'auth Mojang
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