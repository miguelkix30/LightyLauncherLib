use crate::time_it;
use crate::java::jre_downloader::jre_download;
use crate::java::JavaDistribution;
use crate::minecraft::version::version_metadata::VersionBuilder;
use crate::minecraft::launch::errors::{InstallerError, InstallerResult};
use crate::minecraft::version::version::Loader;
use std::sync::Arc;
use crate::minecraft::version::version_metadata::VersionMetaData;
use tokio::sync::oneshot;
use crate::java::jre_downloader::find_java_binary;
use crate::java::runtime::JavaRuntime;
use crate::minecraft::version::version::Version;
use crate::minecraft::launch::arguments::Arguments;
use tracing::{info, error, debug};

pub trait Launch<'a> {
    async fn launch(&mut self, username: &str, uuid: &str, java_distribution: JavaDistribution) -> InstallerResult<()>;
}

impl<'a> Launch<'a> for Version<'a> {
    async fn launch(&mut self, username: &str, uuid: &str, java_distribution: JavaDistribution) -> InstallerResult<()> {

        let metadata: Arc<VersionMetaData> = match self.loader {
            Loader::Vanilla => self.get_complete().await.map_err(|_| InstallerError::UnsupportedLoader("Vanilla".into()))?,
            Loader::Fabric => self.get_fabric_complete().await.map_err(|_| InstallerError::UnsupportedLoader("Fabric".into()))?,
            Loader::Quilt => self.get_quilt_complete().await.map_err(|_| InstallerError::UnsupportedLoader("Quilt".into()))?,
            Loader::NeoForge => self.get_neoforge_complete().await.map_err(|_| InstallerError::UnsupportedLoader("NeoForge".into()))?,
            Loader::LightyUpdater => self.get_lighty_updater_complete().await.map_err(|_| InstallerError::UnsupportedLoader("LightyUpdater".into()))?,
            _ => return Err(InstallerError::UnsupportedLoader("Unknown".into())),
        };

        let builder: &VersionBuilder = match metadata.as_ref() {
            VersionMetaData::VersionBuilder(b) => b,
            _ => return Err(InstallerError::InvalidMetadata),
        };

        // Vérifier/installer Java AVANT l'installation de Minecraft
        let java_version = &builder.java_version.major_version;
        let java_path = match find_java_binary(&self.java_dirs, &java_distribution, java_version).await {
            Ok(path) => {
                info!("[Java] Java {} already installed at: {:?}", java_version, path);
                path
            }
            Err(_) => {
                // Java non trouvé, on l'installe
                info!("[Java] Java {} not found, downloading...", java_version);

                jre_download(
                    &self.java_dirs,
                    &java_distribution,
                    java_version,
                    |current, total| {
                        debug!("Téléchargement JRE : {}/{}", current, total);
                    }
                ).await.map_err(|e| InstallerError::DownloadFailed(format!("JRE download failed: {}", e)))?
            }
        };

        info!("[Java] Using Java binary: {:?}", java_path);

        // Installer les dépendances Minecraft APRES le JRE
        time_it!("Install delay",self.install(builder).await?);

        // Construire les arguments
        let arguments = self.build_arguments(builder, username, uuid);

        println!("LES ARGUMENTS COMPLET{:?}", arguments);

        // Créer JavaRuntime avec le chemin vers java.exe
        let java_runtime = JavaRuntime::new(java_path);

        match java_runtime.execute(arguments, &self.game_dirs).await {
            Ok(mut child) => {
                let (_tx, rx) = oneshot::channel::<()>();

                // Affiche les logs Java en temps réel dans le terminal
                fn print_output(_: &(), buf: &[u8]) -> crate::java::JavaRuntimeResult<()> {
                    print!("{}", String::from_utf8_lossy(buf));
                    Ok(())
                }

                if let Err(e) = java_runtime
                    .handle_io(&mut child, print_output, print_output, rx, &())
                    .await
                {
                    error!("Erreur IO: {}", e);
                }

                if let Some(pid) = child.id() {
                    info!("Processus lancé avec succès, PID: {}", pid);
                } else {
                    info!("Processus lancé avec succès, PID non disponible");
                }

                // tx.send(()); // <- à utiliser si tu veux forcer l'arrêt du process plus tard
            }
            Err(e) => {
                error!("Erreur lors du lancement: {}", e);
                return Err(InstallerError::DownloadFailed(format!("Launch failed: {}", e)));
            }
        }

        Ok(())
    }
}