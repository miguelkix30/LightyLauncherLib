
use zip::ZipArchive;
use std::sync::Arc;
use tracing::{error, info, warn};
use std::path::PathBuf;
use tokio::fs;
use tokio::io::{AsyncWriteExt, BufWriter};
use tokio::sync::Semaphore;
use crate::minecraft::version::version::Version;
use crate::minecraft::version::version_metadata::{Library, Native, Client, AssetsFile, VersionBuilder,Mods};
use crate::utils::hosts::HTTP_CLIENT as CLIENT;
use crate::minecraft::utils::sha1::verify_file_sha1;
use crate::{mkdir, time_it};
use futures::future::try_join_all;
use futures::StreamExt;
use crate::minecraft::launch::errors::{InstallerError, InstallerResult};

// Limit concurrent downloads to prevent socket exhaustion
const MAX_CONCURRENT_DOWNLOADS: usize = 50;

impl<'a> Version<'a> {
    /// Installe toutes les dépendances en parallèle
    pub async fn install(&self , builder: &VersionBuilder) -> InstallerResult<()> {
        info!("[Installer] Starting installation for {}", self.name);

        time_it!("Total installation", {
            self.create_directories().await;


            // Vérifier et télécharger en parallèle
            tokio::try_join!(
                self.verify_and_download_libraries(&builder.libraries),
                //TODO revoir les unwraps
                self.verify_and_download_natives(builder.natives.as_deref().unwrap_or(&[])),
                self.verify_and_download_mods(builder.mods.as_deref().unwrap_or(&[])),
                self.verify_and_download_client(builder.client.as_ref()),
                self.verify_and_download_assets(builder.assets.as_ref()),
            )?;
        });

        info!("[Installer] Installation completed successfully!");
        Ok(())
    }

    async fn create_directories(&self) {
        let parent_path = self.game_dirs.to_path_buf();
        mkdir!(parent_path.join("libraries"));
        mkdir!(parent_path.join("natives"));
        mkdir!(parent_path.join("assets").join("objects"));
    }

    /// Vérifie et télécharge les libraries manquantes/corrompues
    async fn verify_and_download_libraries(&self, libraries: &[Library]) -> InstallerResult<()> {
        let parent_path = self.game_dirs.join("libraries");
        let mut tasks = Vec::new();

        for lib in libraries {
            let Some(url) = &lib.url else { continue };
            let Some(path_str) = &lib.path else { continue };

            let path = parent_path.join(path_str);

            let needs_download = if !path.exists() {
                true
            } else if let Some(sha1) = &lib.sha1 {
                match verify_file_sha1(&path, sha1).await {
                    Ok(true) => false,
                    _ => {
                        warn!("[Installer] SHA1 mismatch for {}, re-downloading...", lib.name);
                        let _ = fs::remove_file(&path).await;
                        true
                    }
                }
            } else {
                false
            };

            if needs_download {
                tasks.push((url.clone(), path));
            }
        }

        if tasks.is_empty() {
            info!("[Installer] ✓ All libraries already cached and verified");
            return Ok(());
        }

        info!("[Installer] Downloading {} libraries...", tasks.len());
        time_it!("Libraries download", {
            download_with_concurrency_limit(tasks).await?
        });
        info!("[Installer] ✓ Libraries installed");
        Ok(())
    }

    async fn verify_and_download_mods(&self, mods: &[Mods]) -> InstallerResult<()> {
        let parent_path = self.game_dirs.join("mods");
        let mut tasks = Vec::new();

        for _mod in mods {
            let Some(url) = &_mod.url else { continue };
            let Some(path_str) = &_mod.path else { continue };

            let path = parent_path.join(path_str);

            let needs_download = if !path.exists() {
                true
            } else if let Some(sha1) = &_mod.sha1 {
                match verify_file_sha1(&path, sha1).await {
                    Ok(true) => false,
                    _ => {
                        warn!("[Installer] SHA1 mismatch for {}, re-downloading...", _mod.name);
                        let _ = fs::remove_file(&path).await;
                        true
                    }
                }
            } else {
                false
            };

            if needs_download {
                tasks.push((url.clone(), path));
            }
        }

        if tasks.is_empty() {
            info!("[Installer] ✓ All Mod already cached and verified");
            return Ok(());
        }

        info!("[Installer] Downloading {} libraries...", tasks.len());
        time_it!("Libraries download", {
            download_with_concurrency_limit(tasks).await?
        });
        info!("[Installer] ✓ Libraries installed");
        Ok(())
    }

    /// Vérifie, télécharge et extrait les natives (clean systématique + extraction parallèle async)
    async fn verify_and_download_natives(&self, natives: &[Native]) -> InstallerResult<()> {
        if natives.is_empty() {
            return Ok(());
        }

        let libraries_path = self.game_dirs.join("libraries");
        let natives_extract_path = self.game_dirs.join("natives");

        // Clean du dossier natives à chaque installation
        if natives_extract_path.exists() {
            let _ = fs::remove_dir_all(&natives_extract_path).await;
        }
        mkdir!(natives_extract_path);

        // ✅ Séparer en deux passes : téléchargement puis extraction
        let mut download_tasks = Vec::new();
        let mut extract_paths = Vec::new();

        for native in natives {
            let Some(url) = &native.url else { continue };
            let Some(path_str) = &native.path else { continue };

            let jar_path = libraries_path.join(path_str);

            // Vérifier si le JAR existe et est valide
            let needs_download = if !jar_path.exists() {
                true
            } else if let Some(sha1) = &native.sha1 {
                match verify_file_sha1(&jar_path, sha1).await {
                    Ok(true) => false,
                    _ => {
                        warn!("[Installer] SHA1 mismatch for {}, re-downloading...", native.name);
                        let _ = fs::remove_file(&jar_path).await;
                        true
                    }
                }
            } else {
                false
            };

            if needs_download {
                download_tasks.push((url.clone(), jar_path.clone()));
            }

            extract_paths.push(jar_path);
        }

        // Télécharger les natives manquantes
        if !download_tasks.is_empty() {
            info!("[Installer] Downloading {} natives...", download_tasks.len());
            time_it!("Natives download", {
                download_with_concurrency_limit(download_tasks).await?
            });
            info!("[Installer] ✓ Natives downloaded");
        } else {
            info!("[Installer] ✓ All natives already cached and verified");
        }

        // Extraire tous les natives en parallèle
        if !extract_paths.is_empty() {
            info!("[Installer] Extracting {} natives...", extract_paths.len());
            let extraction_tasks: Vec<_> = extract_paths
                .into_iter()
                .map(|jar_path| extract_native(jar_path, natives_extract_path.clone()))
                .collect();

            time_it!("Natives extraction", try_join_all(extraction_tasks).await?);
            info!("[Installer] ✓ Natives extracted");
        }

        Ok(())
    }

    /// Vérifie et télécharge le client JAR si nécessaire
    async fn verify_and_download_client(&self, client: Option<&Client>) -> InstallerResult<()> {
        let Some(client) = client else {
            return Ok(());
        };

        let Some(url) = &client.url else {
            return Ok(());
        };

        let client_path = self.game_dirs.join(format!("{}.jar", self.name));

        let needs_download = if !client_path.exists() {
            true
        } else if let Some(sha1) = &client.sha1 {
            match verify_file_sha1(&client_path, sha1).await {
                Ok(true) => false,
                _ => {
                    warn!("[Installer] Client JAR SHA1 mismatch, re-downloading...");
                    let _ = fs::remove_file(&client_path).await;
                    true
                }
            }
        } else {
            false
        };

        if !needs_download {
            info!("[Installer] ✓ Client JAR already cached and verified");
            return Ok(());
        }

        info!("[Installer] Downloading client JAR...");
        time_it!("Client download", download_large_file(url.clone(), client_path).await?);
        info!("[Installer] ✓ Client JAR installed");
        Ok(())
    }

    /// Vérifie et télécharge les assets manquants/corrompus
    async fn verify_and_download_assets(&self, assets: Option<&AssetsFile>) -> InstallerResult<()> {
        let Some(assets) = assets else {
            return Ok(());
        };

        let parent_path = self.game_dirs.join("assets").join("objects");
        let mut tasks = Vec::new();

        for asset in assets.objects.values() {
            let Some(url) = &asset.url else { continue };

            let hash_prefix = &asset.hash[0..2];
            let path = parent_path.join(hash_prefix).join(&asset.hash);

            let needs_download = if !path.exists() {
                true
            } else {
                match verify_file_sha1(&path, &asset.hash).await {
                    Ok(true) => false,
                    _ => {
                        let _ = fs::remove_file(&path).await;
                        true
                    }
                }
            };

            if needs_download {
                tasks.push((url.clone(), path));
            }
        }

        if tasks.is_empty() {
            info!("[Installer] ✓ All assets already cached and verified");
            return Ok(());
        }

        info!("[Installer] Downloading {} new assets...", tasks.len());
        time_it!("Assets download", {
            download_small_with_concurrency_limit(tasks).await?
        });
        info!("[Installer] ✓ Assets installed");
        Ok(())
    }
}

/// Extrait un native JAR (version memory-mapped avec spawn_blocking)
async fn extract_native(jar_path: PathBuf, natives_dir: PathBuf) -> InstallerResult<()> {
    // Use spawn_blocking for sync ZIP operations
    tokio::task::spawn_blocking(move || {
        // Memory-map the file instead of reading entirely into memory
        let file = std::fs::File::open(&jar_path)?;
        //TODO: REVOIR CE UNSAFE
        let mmap = unsafe { memmap2::Mmap::map(&file)? };

        let cursor = std::io::Cursor::new(&mmap[..]);
        let mut archive = ZipArchive::new(cursor)?;

        // Extract native files
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let file_name = file.name().to_string();

            if is_native_file(&file_name) {
                let dest_path = natives_dir.join(
                    std::path::Path::new(&file_name)
                        .file_name()
                        .unwrap_or_default()
                );

                // Stream directly to disk instead of buffering in memory
                let mut dest_file = std::fs::File::create(&dest_path)?;
                std::io::copy(&mut file, &mut dest_file)?;
            }
        }

        Ok::<_, InstallerError>(())
    })
    .await
    .map_err(|e| InstallerError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?
}

/// Vérifie si un fichier est natif
#[inline]
fn is_native_file(filename: &str) -> bool {
    const NATIVE_EXTENSIONS: &[&str] = &[".dll", ".so", ".dylib", ".jnilib"];

    let filename_lower = filename.to_lowercase();

    NATIVE_EXTENSIONS.iter().any(|ext| filename_lower.ends_with(ext))
        || filename_lower.contains(".so.")
}

/// Téléchargement de petits fichiers
async fn download_small_file(url: String, dest: PathBuf) -> InstallerResult<()> {
    const MAX_RETRIES: u32 = 3;
    const INITIAL_DELAY_MS: u64 = 20;

    let mut last_error = None;

    for attempt in 1..=MAX_RETRIES {
        match download_small_file_once(&url, &dest).await {
            Ok(_) => return Ok(()),
            Err(e) => {
                if attempt < MAX_RETRIES {
                    let delay = INITIAL_DELAY_MS * 2u64.pow(attempt - 1);
                    warn!(
                        "[Retry {}/{}] Failed to download {}: {}. Retrying in {}ms...",
                        attempt, MAX_RETRIES, url, e, delay
                    );
                    tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;
                }
                last_error = Some(e);
            }
        }
    }

    Err(last_error.unwrap())
}

async fn download_small_file_once(url: &str, dest: &PathBuf) -> InstallerResult<()> {
    let bytes = CLIENT.get(url).send().await?.bytes().await?;

    if let Some(parent) = dest.parent() {
        mkdir!(parent);
    }

    fs::write(dest, bytes).await?;
    Ok(())
}

/// Téléchargement de gros fichiers
async fn download_large_file(url: String, dest: PathBuf) -> InstallerResult<()> {
    const MAX_RETRIES: u32 = 3;
    const INITIAL_DELAY_MS: u64 = 20;

    let mut last_error = None;

    for attempt in 1..=MAX_RETRIES {
        match download_large_file_once(&url, &dest).await {
            Ok(_) => return Ok(()),
            Err(e) => {
                if attempt < MAX_RETRIES {
                    let delay = INITIAL_DELAY_MS * 2u64.pow(attempt - 1);
                    warn!(
                        "[Retry {}/{}] Failed to download {}: {}. Retrying in {}ms...",
                        attempt, MAX_RETRIES, url, e, delay
                    );
                    let _ = fs::remove_file(&dest).await;
                    tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;
                }
                last_error = Some(e);
            }
        }
    }

    Err(last_error.unwrap())
}

async fn download_large_file_once(url: &str, dest: &PathBuf) -> InstallerResult<()> {
    let response = CLIENT.get(url).send().await?;

    if !response.status().is_success() {
        return Err(InstallerError::DownloadFailed(format!(
            "HTTP {} for {}",
            response.status(),
            url
        )));
    }

    if let Some(parent) = dest.parent() {
        mkdir!(parent);
    }

    let file = fs::File::create(dest).await?;
    let mut writer = BufWriter::with_capacity(256 * 1024, file);
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        writer.write_all(&chunk).await?;
    }

    writer.flush().await?;
    Ok(())
}

/// Download large files with concurrency limit
async fn download_with_concurrency_limit(tasks: Vec<(String, PathBuf)>) -> InstallerResult<()> {
    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_DOWNLOADS));
    let futures: Vec<_> = tasks
        .into_iter()
        .map(|(url, dest)| {
            let sem = semaphore.clone();
            async move {
                let _permit = sem.acquire().await.unwrap();
                download_large_file(url, dest).await
            }
        })
        .collect();

    try_join_all(futures).await?;
    Ok(())
}

/// Download small files with concurrency limit
async fn download_small_with_concurrency_limit(tasks: Vec<(String, PathBuf)>) -> InstallerResult<()> {
    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_DOWNLOADS));
    let futures: Vec<_> = tasks
        .into_iter()
        .map(|(url, dest)| {
            let sem = semaphore.clone();
            async move {
                let _permit = sem.acquire().await.unwrap();
                download_small_file(url, dest).await
            }
        })
        .collect();

    try_join_all(futures).await?;
    Ok(())
}