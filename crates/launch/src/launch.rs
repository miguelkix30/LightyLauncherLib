use lighty_core::time_it;
use lighty_java::jre_downloader::jre_download;
use lighty_java::JavaDistribution;
use lighty_loaders::types::version_metadata::Version;
use crate::errors::{InstallerError, InstallerResult};
use crate::installer::Installer;
use lighty_loaders::types::{Loader, LoaderExtensions, VersionInfo};
use std::sync::Arc;
use std::path::PathBuf;
use lighty_loaders::types::version_metadata::VersionMetaData;
use tokio::sync::oneshot;
use lighty_java::jre_downloader::find_java_binary;
use lighty_java::runtime::JavaRuntime;
use crate::arguments::Arguments;
use tracing::{info, error, debug};

pub trait Launch {
    async fn launch(&mut self, username: &str, uuid: &str, java_distribution: JavaDistribution) -> InstallerResult<()>;
}

// Implémentation générique pour tout type implémentant VersionInfo + les traits nécessaires
impl<T> Launch for T
where
    T: VersionInfo<LoaderType = Loader> + LoaderExtensions + Arguments + Installer,
{
    async fn launch(&mut self, username: &str, uuid: &str, java_distribution: JavaDistribution) -> InstallerResult<()> {
        // 1. Préparer les métadonnées du loader
        let metadata = prepare_metadata(self).await?;

        let version = extract_version(&metadata)?;

        // 2. S'assurer que Java est installé
        let java_path = ensure_java_installed(self, version, &java_distribution).await?;

        // 3. Installer les dépendances Minecraft
        time_it!("Install delay", self.install(version).await?);

        // 4. Lancer le jeu
        execute_game(self, version, username, uuid, java_path).await
    }
}

/// Récupère les métadonnées complètes du loader
async fn prepare_metadata<T>(builder: &mut T) -> InstallerResult<Arc<VersionMetaData>>
where
    T: VersionInfo<LoaderType = Loader> + LoaderExtensions,
{
    debug!("[Launch] Fetching metadata for loader: {:?}", builder.loader());

    let metadata = match builder.loader() {
        Loader::Vanilla => builder.get_complete().await?,
        Loader::Fabric => builder.get_fabric_complete().await?,
        Loader::Quilt => builder.get_quilt_complete().await?,
        Loader::NeoForge => builder.get_neoforge_complete().await?,
        Loader::LightyUpdater => builder.get_lighty_updater_complete().await?,
        _ => return Err(InstallerError::UnsupportedLoader(format!("{:?}", builder.loader()))),
    };

    info!("[Launch] Metadata fetched successfully for {:?}", builder.loader());
    Ok(metadata)
}

/// S'assure que Java est installé et retourne le chemin vers l'exécutable
async fn ensure_java_installed<T>(
    builder: &T,
    version: &Version,
    java_distribution: &JavaDistribution,
) -> InstallerResult<PathBuf>
where
    T: VersionInfo,
{
    let java_version = version.java_version.major_version;

    // Vérifier si Java est déjà installé
    match find_java_binary(builder.java_dirs(), java_distribution, &java_version).await {
        Ok(path) => {
            info!("[Java] Java {} already installed at: {:?}", java_version, path);
            Ok(path)
        }
        Err(_) => {
            info!("[Java] Java {} not found, downloading...", java_version);

            let path = jre_download(
                builder.java_dirs(),
                java_distribution,
                &java_version,
                |current, total| {
                    debug!("[Java] Download progress: {}/{}", current, total);
                }
            ).await.map_err(|e| InstallerError::DownloadFailed(format!("JRE download failed: {}", e)))?;

            info!("[Java] Java {} installed successfully", java_version);
            Ok(path)
        }
    }
}

/// Lance le jeu avec les arguments appropriés
async fn execute_game<T>(
    builder: &T,
    version: &Version,
    username: &str,
    uuid: &str,
    java_path: PathBuf,
) -> InstallerResult<()>
where
    T: VersionInfo + Arguments,
{
    // Construire les arguments
    let arguments = builder.build_arguments(version, username, uuid);
    println!("{:?}", arguments);
    debug!("[Launch] Launch arguments: {:?}", arguments);

    // Créer JavaRuntime avec le chemin vers java.exe
    let java_runtime = JavaRuntime::new(java_path);
    info!("[Launch] Executing game...");

    match java_runtime.execute(arguments, builder.game_dirs()).await {
        Ok(mut child) => {
            let (_tx, rx) = oneshot::channel::<()>();

            // Affiche les logs Java en temps réel dans le terminal
            let print_output = |_: &(), buf: &[u8]| -> lighty_java::JavaRuntimeResult<()> {
                print!("{}", String::from_utf8_lossy(buf));
                Ok(())
            };

            if let Err(e) = java_runtime
                .handle_io(&mut child, print_output, print_output, rx, &())
                .await
            {
                error!("[Launch] IO error: {}", e);
            }

            if let Some(pid) = child.id() {
                info!("[Launch] Game launched successfully, PID: {}", pid);
            } else {
                info!("[Launch] Game launched successfully, PID unavailable");
            }

            // tx.send(()); // <- à utiliser si tu veux forcer l'arrêt du process plus tard
            Ok(())
        }
        Err(e) => {
            error!("[Launch] Failed to launch game: {}", e);
            Err(InstallerError::DownloadFailed(format!("Launch failed: {}", e)))
        }
    }
}

/// Extrait l'objet Version depuis VersionMetaData
fn extract_version(metadata: &VersionMetaData) -> InstallerResult<&Version> {
    match metadata {
        VersionMetaData::Version(v) => Ok(v),
        _ => Err(InstallerError::InvalidMetadata),
    }
}