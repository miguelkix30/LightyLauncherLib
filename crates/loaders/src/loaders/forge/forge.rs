//! Forge loader (Minecraft 1.13+).
//!
//! Installer model is the same as NeoForge (same `install_profile.json` +
//! `version.json` shape, same processor pipeline). The differences are
//! routed through configuration:
//! - Installer URLs live on `maven.minecraftforge.net`
//! - Processor JARs are fetched from the same Maven base
//! - Cached extracts are scoped to `net/minecraftforge/` (so they don't
//!   collide with NeoForge's `net/neoforged/` extracts)
//!
//! Legacy Forge (≤ 1.12.2) uses a completely different installer format
//! and is handled by the separate `forge_legacy` loader.

use async_trait::async_trait;
use once_cell::sync::Lazy;
use std::{collections::HashMap, fs::File, io::Read, path::PathBuf};
use zip::ZipArchive;

use lighty_core::download::download_file_untracked;
use lighty_core::mkdir;

use crate::loaders::vanilla::vanilla::VanillaQuery;
use crate::types::version_metadata::{Arguments, Library, MainClass, Version, VersionMetaData};
use crate::types::VersionInfo;
use crate::utils::forge_installer::{ForgeInstallProfile, ForgeVersionManifest};
use crate::utils::forge_processor::{extract_maven_bundle_to_libraries, run_processors};
use crate::utils::maven::fetch_maven_sha1;
use crate::utils::{error::QueryError, manifest::ManifestRepository, query::Query};

/// Maven repository for Forge artifacts.
const FORGE_MAVEN: &str = "https://maven.minecraftforge.net";
/// Subdirectory under `libraries/` used to cache files extracted from the
/// Forge installer JAR.
const FORGE_EXTRACT_SUBDIR: &str = "net/minecraftforge";

pub type Result<T> = std::result::Result<T, QueryError>;

/// Shared cached repository for Forge manifests.
pub static FORGE: Lazy<ManifestRepository<ForgeQuery>> =
    Lazy::new(|| ManifestRepository::new());

/// Sub-queries supported by the Forge loader.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ForgeQuery {
    /// Library list from `install_profile.json` (processor-only libraries).
    Libraries,
    /// Full merged [`Version`] for a Forge instance.
    ForgeBuilder,
}

#[async_trait]
impl Query for ForgeQuery {
    type Query = ForgeQuery;
    type Data = VersionMetaData;
    type Raw = ForgeInstallProfile;

    fn name() -> &'static str {
        "forge"
    }

    async fn fetch_full_data<V: VersionInfo>(version: &V) -> Result<ForgeInstallProfile> {
        let installer_url = build_installer_url(version);
        lighty_core::trace_debug!(url = %installer_url, loader = "forge", "Installer URL constructed");

        let profiles_dir = version.game_dirs().join(".forge");
        mkdir!(profiles_dir);

        let installer_path = profiles_dir.join(format!(
            "forge-{}-installer.jar",
            version.loader_version()
        ));

        // Verify cached installer and re-download if needed
        let needs_download = if installer_path.exists() {
            match verify_installer_sha1(&installer_path, &installer_url).await {
                Ok(true) => {
                    lighty_core::trace_info!(loader = "forge", "Installer already exists and SHA1 is valid");
                    false
                }
                Ok(false) => {
                    lighty_core::trace_warn!(loader = "forge", "Installer exists but SHA1 mismatch, re-downloading");
                    true
                }
                Err(_e) => {
                    lighty_core::trace_warn!(error = %_e, loader = "forge", "Could not verify SHA1, using existing file");
                    false
                }
            }
        } else {
            true
        };

        if needs_download {
            lighty_core::trace_info!(path = ?installer_path, loader = "forge", "Downloading installer");
            download_file_untracked(&installer_url, &installer_path)
                .await
                .map_err(|e| QueryError::Conversion {
                    message: format!("Failed to download installer: {}", e),
                })?;

            if let Ok(valid) = verify_installer_sha1(&installer_path, &installer_url).await {
                if !valid {
                    return Err(QueryError::Conversion {
                        message: "Downloaded installer has invalid SHA1".to_string(),
                    });
                }
            }
        }

        // Read the embedded JSONs directly from the installer JAR (no disk extraction)
        let (install_profile, _) = read_jsons_from_jar(&installer_path).await?;

        lighty_core::trace_info!(loader = "forge", "Successfully loaded Forge metadata");

        Ok(install_profile)
    }

    async fn extract<V: VersionInfo>(
        version: &V,
        query: &Self::Query,
        full_data: &ForgeInstallProfile,
    ) -> Result<Self::Data> {
        let result = match query {
            ForgeQuery::Libraries => {
                VersionMetaData::Libraries(extract_install_profile_libraries(full_data))
            }
            ForgeQuery::ForgeBuilder => {
                VersionMetaData::Version(Self::version_builder(version, full_data).await?)
            }
        };
        Ok(result)
    }

    async fn version_builder<V: VersionInfo>(
        version: &V,
        _full_data: &ForgeInstallProfile,
    ) -> Result<Version> {
        // Fetch Vanilla data and read version.json from the installer JAR in parallel
        let (vanilla_builder, version_meta) = tokio::try_join!(
            async {
                let vanilla_data = VanillaQuery::fetch_full_data(version).await?;
                VanillaQuery::version_builder(version, &vanilla_data).await
            },
            async {
                let profiles_dir = version.game_dirs().join(".forge");
                let installer_path = profiles_dir.join(format!(
                    "forge-{}-installer.jar",
                    version.loader_version()
                ));
                let (_, version_meta) = read_jsons_from_jar(&installer_path).await?;
                Ok::<_, QueryError>(version_meta)
            }
        )?;

        // Use ONLY the runtime libraries from version.json (install_profile
        // libraries are processor-only and must not end up on the classpath).
        let version_json_libs = extract_libraries_from_version_meta(&version_meta);

        // Merge: Vanilla base + version.json overrides
        let merged_libs = merge_libraries(vanilla_builder.libraries, version_json_libs);

        Ok(Version {
            main_class: merge_main_class(vanilla_builder.main_class, extract_main_class(&version_meta)),
            java_version: vanilla_builder.java_version,
            arguments: merge_arguments(vanilla_builder.arguments, extract_arguments(&version_meta)),
            libraries: merged_libs,
            mods: None,
            natives: vanilla_builder.natives,
            client: vanilla_builder.client,
            assets_index: vanilla_builder.assets_index,
            assets: vanilla_builder.assets,
        })
    }
}

/// --------- Merge helpers ----------
fn merge_main_class(vanilla: MainClass, forge: MainClass) -> MainClass {
    if forge.main_class.is_empty() {
        vanilla
    } else {
        forge
    }
}

fn merge_arguments(vanilla: Arguments, forge: Arguments) -> Arguments {
    Arguments {
        game: {
            let mut args = vanilla.game;
            args.extend(forge.game);
            args
        },
        jvm: match (vanilla.jvm, forge.jvm) {
            (Some(mut v), Some(f)) => {
                v.extend(f);
                Some(v)
            }
            (Some(v), None) => Some(v),
            (None, Some(f)) => Some(f),
            (None, None) => None,
        },
    }
}

/// Merges library lists, de-duplicating by `group:artifact` (version-agnostic).
fn merge_libraries(vanilla_libs: Vec<Library>, forge_libs: Vec<Library>) -> Vec<Library> {
    let capacity = vanilla_libs.len() + forge_libs.len();
    let mut lib_map: HashMap<String, Library> = HashMap::with_capacity(capacity);

    // Insert Vanilla first
    for lib in vanilla_libs {
        let key = extract_artifact_key(&lib.name);
        lib_map.insert(key, lib);
    }

    // Forge overrides Vanilla on key collision (typically a newer version)
    for lib in forge_libs {
        let key = extract_artifact_key(&lib.name);
        lib_map.insert(key, lib);
    }

    lib_map.into_values().collect()
}

/// Extracts the `group:artifact` (version-agnostic) key used for dedup.
fn extract_artifact_key(maven_name: &str) -> String {
    let mut parts = maven_name.split(':');
    match (parts.next(), parts.next()) {
        (Some(group), Some(artifact)) => format!("{}:{}", group, artifact),
        _ => maven_name.to_string(),
    }
}

/// --------- Extraction helpers ----------
fn extract_main_class(version_meta: &ForgeVersionManifest) -> MainClass {
    MainClass {
        main_class: version_meta.main_class.clone(),
    }
}

fn extract_arguments(version_meta: &ForgeVersionManifest) -> Arguments {
    Arguments {
        game: version_meta.arguments.game.clone(),
        jvm: Some(version_meta.arguments.jvm.clone()),
    }
}

/// Returns the install_profile.json libraries as the launcher's pivot
/// [`Library`] type so they can be fed through the generic library
/// installer.
///
/// Includes both the processor JARs and the runtime-required artifacts
/// (notably `net.minecraftforge:forge:VERSION:universal`, which is
/// referenced at runtime by FML but absent from `version.json`).
pub fn extract_install_profile_libraries(full_data: &ForgeInstallProfile) -> Vec<Library> {
    full_data
        .libraries
        .iter()
        .map(|lib| Library {
            name: lib.name.clone(),
            url: Some(lib.downloads.artifact.url.clone()),
            path: Some(lib.downloads.artifact.path.clone()),
            sha1: Some(lib.downloads.artifact.sha1.clone()),
            size: Some(lib.downloads.artifact.size),
        })
        .collect()
}

fn extract_libraries_from_version_meta(version_meta: &ForgeVersionManifest) -> Vec<Library> {
    version_meta
        .libraries
        .iter()
        .map(|lib| Library {
            name: lib.name.clone(),
            url: Some(lib.downloads.artifact.url.clone()),
            path: Some(lib.downloads.artifact.path.clone()),
            sha1: Some(lib.downloads.artifact.sha1.clone()),
            size: Some(lib.downloads.artifact.size),
        })
        .collect()
}

/// --------- Helpers ----------
fn build_installer_url<V: VersionInfo>(version: &V) -> String {
    format!(
        "{}/net/minecraftforge/forge/{}-{}/forge-{}-{}-installer.jar",
        FORGE_MAVEN,
        version.minecraft_version(),
        version.loader_version(),
        version.minecraft_version(),
        version.loader_version(),
    )
}

fn processors_marker_path<V: VersionInfo>(version: &V) -> PathBuf {
    version.game_dirs().join(".forge").join(format!(
        "processors-{}-{}.sha1",
        version.minecraft_version(),
        version.loader_version()
    ))
}

/// Reads `install_profile.json` and `version.json` directly from the
/// installer JAR without extracting anything to disk.
async fn read_jsons_from_jar(
    installer_path: &PathBuf,
) -> Result<(ForgeInstallProfile, ForgeVersionManifest)> {
    let path = installer_path.clone();

    tokio::task::spawn_blocking(move || {
        let file = File::open(&path).map_err(|e| QueryError::Conversion {
            message: format!("Failed to open installer JAR: {}", e),
        })?;

        let mut archive = ZipArchive::new(file).map_err(|e| QueryError::Conversion {
            message: format!("Failed to open ZIP archive: {}", e),
        })?;

        // Read install_profile.json
        let install_profile = {
            let mut file = archive.by_name("install_profile.json").map_err(|_| {
                QueryError::MissingField {
                    field: "install_profile.json in installer JAR".to_string(),
                }
            })?;

            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .map_err(|e| QueryError::Conversion {
                    message: format!("Failed to read install_profile.json: {}", e),
                })?;

            serde_json::from_str::<ForgeInstallProfile>(&contents)?
        };

        // Read version.json
        let version_meta = {
            let mut file = archive
                .by_name("version.json")
                .map_err(|_| QueryError::MissingField {
                    field: "version.json in installer JAR".to_string(),
                })?;

            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .map_err(|e| QueryError::Conversion {
                    message: format!("Failed to read version.json: {}", e),
                })?;

            serde_json::from_str::<ForgeVersionManifest>(&contents)?
        };

        Ok((install_profile, version_meta))
    })
    .await
    .map_err(|e| QueryError::Conversion {
        message: format!("Failed to spawn blocking task: {}", e),
    })?
}

/// Verifies the local installer JAR matches the expected SHA1 from Maven.
async fn verify_installer_sha1(installer_path: &PathBuf, installer_url: &str) -> Result<bool> {
    let expected_sha1 = fetch_maven_sha1(installer_url)
        .await
        .ok_or_else(|| QueryError::Conversion {
            message: "Failed to fetch SHA1 from Maven".to_string(),
        })?;

    lighty_core::verify_file_sha1_sync(installer_path, &expected_sha1).map_err(|e| {
        QueryError::Conversion {
            message: format!("Failed to verify SHA1: {}", e),
        }
    })
}

/// Runs the Forge install processors.
///
/// The caller must have already downloaded the install_profile libraries
/// (via the generic library installer pipeline) so the processor JARs
/// and their classpath dependencies are on disk before this is invoked.
pub async fn run_install_processors<V: VersionInfo>(
    version: &V,
    install_profile: &ForgeInstallProfile,
) -> Result<()> {
    lighty_core::trace_info!(loader = "forge", "Checking if processors need to run");

    let profiles_dir = version.game_dirs().join(".forge");
    mkdir!(profiles_dir);
    let installer_path = profiles_dir.join(format!(
        "forge-{}-installer.jar",
        version.loader_version(),
    ));

    if !installer_path.exists() {
        return Err(QueryError::Conversion {
            message: "Installer JAR not found. Run fetch_full_data first.".to_string(),
        });
    }

    // Extract artifacts bundled inside the installer JAR (Forge ships
    // forge-shim.jar in 1.21+, forge.jar + forge-universal.jar in 1.14,
    // etc. at `/maven/...`). Idempotent.
    let libraries_dir = version.game_dirs().join("libraries");
    extract_maven_bundle_to_libraries(&installer_path, &libraries_dir)?;

    let installer_url = build_installer_url(version);
    let marker_path = processors_marker_path(version);
    if let Some(expected_sha1) = fetch_maven_sha1(&installer_url).await {
        if let Ok(existing) = std::fs::read_to_string(&marker_path) {
            if existing.trim() == expected_sha1 {
                lighty_core::trace_info!(
                    loader = "forge",
                    "Processors already executed for this installer, skipping"
                );
                return Ok(());
            }
        }
    }

    // Run the processors using the shared executor with Forge's
    // maven URL and extract subdirectory.
    run_processors(
        version,
        install_profile,
        installer_path,
        FORGE_MAVEN,
        FORGE_EXTRACT_SUBDIR,
    )
    .await?;

    if let Some(expected_sha1) = fetch_maven_sha1(&installer_url).await {
        if let Err(_err) = std::fs::write(&marker_path, expected_sha1) {
            lighty_core::trace_warn!(
                error = %_err,
                loader = "forge",
                "Failed to write processors marker file"
            );
        }
    }

    lighty_core::trace_info!(loader = "forge", "Processors completed successfully");
    Ok(())
}
