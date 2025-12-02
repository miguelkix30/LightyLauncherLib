
use std::io::Cursor;
use std::path::{Path, PathBuf};
use crate::java::errors::{JreError, JreResult};
use path_absolutize::Absolutize;
use tokio::fs;

use crate::utils::system::{ OperatingSystem, OS};
use crate::utils::download::{download_file};
use crate::utils::extract::{tar_gz_extract, zip_extract} ;

use super::JavaDistribution;

/// Find java binary in JRE folder
pub async fn find_java_binary(
    runtimes_folder: &Path,
    jre_distribution: &JavaDistribution,
    jre_version: &u32,
) -> JreResult<PathBuf> {
    let runtime_path =
        runtimes_folder.join(format!("{}_{}", jre_distribution.get_name(), jre_version));

    // Find JRE in runtime folder
    let mut files = fs::read_dir(&runtime_path).await?;

    if let Some(jre_folder) = files.next_entry().await? {
        let folder_path = jre_folder.path();

        let java_binary = match OS {
            OperatingSystem::WINDOWS => folder_path.join("bin").join("java.exe"),
            OperatingSystem::OSX => folder_path
                .join("Contents")
                .join("Home")
                .join("bin")
                .join("java"),
            _ => folder_path.join("bin").join("java"),
        };

        if java_binary.exists() {
            // Check if the binary has execution permissions on linux and macOS
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;

                let metadata = fs::metadata(&java_binary).await?;

                if metadata.permissions().mode() & 0o111 == 0 {
                    // try to change permissions
                    let mut permissions = metadata.permissions();
                    permissions.set_mode(0o755);
                    fs::set_permissions(&java_binary, permissions).await?;
                }
            }

            return Ok(java_binary.absolutize()?.to_path_buf());
        }
    }

    Err(JreError::NotFound {
        path: runtime_path.display().to_string(),
    })
}

/// Download specific JRE to runtimes
pub async fn jre_download<F>(
    runtimes_folder: &Path,
    jre_distribution: &JavaDistribution,
    jre_version: &u32,
    on_progress: F,
) -> JreResult<PathBuf>
where
    F: Fn(u64, u64),
{
    let runtime_path =
        runtimes_folder.join(format!("{}_{}", jre_distribution.get_name(), jre_version));

    if runtime_path.exists() {
        fs::remove_dir_all(&runtime_path).await?;
    }
    fs::create_dir_all(&runtime_path).await?;

    let url = jre_distribution.get_url(jre_version)
        .map_err(|e| JreError::Download(format!("Failed to get URL: {}", e)))?;

    let retrieved_bytes = download_file(&url, on_progress).await
        .map_err(|e| JreError::Download(format!("Download failed: {}", e)))?;

    let cursor = Cursor::new(&retrieved_bytes[..]);

    match OS {
        OperatingSystem::WINDOWS => {
            zip_extract(cursor, runtime_path.as_path()).await
                .map_err(|e| JreError::Extraction(format!("ZIP extraction failed: {}", e)))?;
        }
        OperatingSystem::LINUX | OperatingSystem::OSX => {
            tar_gz_extract(cursor, runtime_path.as_path()).await
                .map_err(|e| JreError::Extraction(format!("TAR.GZ extraction failed: {}", e)))?;
        }
        OperatingSystem::UNKNOWN => {
            return Err(JreError::UnsupportedOS);
        }
    }

    // Find JRE afterwards
    find_java_binary(runtimes_folder, jre_distribution, jre_version).await
}
