use crate::types::version_metadata::{Library, MainClass, Arguments, Version, VersionMetaData};
use crate::types::VersionInfo;
use crate::utils::{error::QueryError, query::Query, manifest::ManifestRepository};
use crate::loaders::vanilla::vanilla::VanillaQuery;
use once_cell::sync::Lazy;
use super::forge_metadata::{ForgeInstallProfile, ForgeVersionManifest, ForgeArgument, ForgeArgumentValue};
use async_trait::async_trait;
use lighty_core::hosts::HTTP_CLIENT as CLIENT;
use lighty_core::{mkdir, download::download_file_untracked};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use zip::ZipArchive;
use sha1::{Sha1, Digest};

pub type Result<T> = std::result::Result<T, QueryError>;

pub static FORGE: Lazy<ManifestRepository<ForgeQuery>> = Lazy::new(|| ManifestRepository::new());

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ForgeQuery {
    Libraries,
    Arguments,
    MainClass,
    ForgeBuilder,
}

/// Combined metadata from both install_profile.json and version.json
#[derive(Debug, Clone)]
pub struct ForgeMetaData {
    pub install_profile: ForgeInstallProfile,
    pub version_manifest: ForgeVersionManifest,
}

#[async_trait]
impl Query for ForgeQuery {
    type Query = ForgeQuery;
    type Data = VersionMetaData;
    type Raw = ForgeMetaData;

    fn name() -> &'static str {
        "forge"
    }

    async fn fetch_full_data<V: VersionInfo>(version: &V) -> Result<ForgeMetaData> {
        // Build installer URL
        let installer_url = build_forge_installer_url(version);
        lighty_core::trace_debug!(url = %installer_url, "Fetching Forge installer");

        // Create .forge profile directory
        let profiles_dir = version.game_dirs().join(".forge");
        mkdir!(profiles_dir);

        // Local installer path
        let installer_path = profiles_dir.join(format!(
            "forge-{}-{}-installer.jar",
            version.minecraft_version(),
            version.loader_version()
        ));

        // Download installer if needed
        if should_download_installer(&installer_path, &installer_url).await {
            lighty_core::trace_debug!(path = ?installer_path, "Downloading Forge installer");
            download_file_untracked(&installer_url, &installer_path)
                .await
                .map_err(|e| QueryError::Conversion {
                    message: format!("Failed to download Forge installer: {}", e),
                })?;

            // Verify SHA1 after download
            if let Ok(valid) = verify_installer_sha1(&installer_path, &installer_url).await {
                if !valid {
                    return Err(QueryError::Conversion {
                        message: "Downloaded Forge installer has invalid SHA1".to_string(),
                    });
                }
            }
        }

        // Read JSONs from installer JAR
        let metadata = read_forge_metadata_from_jar(&installer_path).await?;
        lighty_core::trace_debug!("Successfully loaded Forge metadata");

        Ok(metadata)
    }

    async fn extract<V: VersionInfo>(
        version: &V,
        query: &Self::Query,
        full_data: &ForgeMetaData,
    ) -> Result<Self::Data> {
        let result = match query {
            ForgeQuery::Libraries => {
                VersionMetaData::Libraries(extract_libraries(full_data).await?)
            }
            ForgeQuery::Arguments => VersionMetaData::Arguments(extract_arguments(full_data)),
            ForgeQuery::MainClass => VersionMetaData::MainClass(extract_main_class(full_data)),
            ForgeQuery::ForgeBuilder => {
                VersionMetaData::Version(Self::version_builder(version, full_data).await?)
            }
        };
        Ok(result)
    }

    async fn version_builder<V: VersionInfo>(
        version: &V,
        full_data: &ForgeMetaData,
    ) -> Result<Version> {
        // Fetch Vanilla data in parallel with extracting Forge libraries
        let (vanilla_builder, forge_libraries) = tokio::try_join!(
            async {
                let vanilla_data = VanillaQuery::fetch_full_data(version).await?;
                VanillaQuery::version_builder(version, &vanilla_data).await
            },
            extract_libraries(full_data)
        )?;

        // Merge Vanilla with Forge
        Ok(Version {
            main_class: merge_main_class(
                vanilla_builder.main_class,
                extract_main_class(full_data),
            ),
            java_version: vanilla_builder.java_version,
            arguments: merge_arguments(vanilla_builder.arguments, extract_arguments(full_data)),
            libraries: merge_libraries(vanilla_builder.libraries, forge_libraries),
            mods: None,
            natives: vanilla_builder.natives,
            client: vanilla_builder.client,
            assets_index: vanilla_builder.assets_index,
            assets: vanilla_builder.assets,
        })
    }
}

// ========== Merge Functions ==========

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

/// Merge libraries avoiding duplicates by comparing group:artifact (without version)
fn merge_libraries(vanilla_libs: Vec<Library>, forge_libs: Vec<Library>) -> Vec<Library> {
    let capacity = vanilla_libs.len() + forge_libs.len();
    let mut lib_map: HashMap<String, Library> = HashMap::with_capacity(capacity);

    // Add Vanilla first
    for lib in vanilla_libs {
        let key = extract_artifact_key(&lib.name);
        lib_map.insert(key, lib);
    }

    // Forge overwrites Vanilla if same artifact (newer version)
    for lib in forge_libs {
        let key = extract_artifact_key(&lib.name);
        lib_map.insert(key, lib);
    }

    lib_map.into_values().collect()
}

/// Extract "group:artifact" (without version) to identify duplicates
fn extract_artifact_key(maven_name: &str) -> String {
    let mut parts = maven_name.split(':');
    match (parts.next(), parts.next()) {
        (Some(group), Some(artifact)) => format!("{}:{}", group, artifact),
        _ => maven_name.to_string(),
    }
}

// ========== Extraction Functions ==========

async fn extract_libraries(full_data: &ForgeMetaData) -> Result<Vec<Library>> {
    let mut libraries = Vec::new();

    // Extract from install_profile.json libraries
    // These already have complete URL and path in the artifact
    for lib_entry in &full_data.install_profile.libraries {
        libraries.push(Library {
            name: lib_entry.name.clone(),
            url: Some(lib_entry.downloads.artifact.url.clone()),
            path: Some(lib_entry.downloads.artifact.path.clone()),
            sha1: Some(lib_entry.downloads.artifact.sha1.clone()),
            size: Some(lib_entry.downloads.artifact.size),
        });
    }

    // Extract from version.json libraries
    for lib_entry in &full_data.version_manifest.libraries {
        // Skip if rules don't apply
        if let Some(rules) = &lib_entry.rules {
            if !should_apply_rules(rules) {
                continue;
            }
        }

        if let Some(downloads) = &lib_entry.downloads {
            if let Some(artifact) = &downloads.artifact {
                // These also have complete URL and path
                libraries.push(Library {
                    name: lib_entry.name.clone(),
                    url: Some(artifact.url.clone()),
                    path: Some(artifact.path.clone()),
                    sha1: Some(artifact.sha1.clone()),
                    size: Some(artifact.size),
                });
            }
        } else if let Some(base_url) = &lib_entry.url {
            // Fallback for libraries without downloads field
            let (path, url) = maven_artifact_to_path_and_url(&lib_entry.name, base_url);

            // Try to fetch SHA1 and size
            let (sha1, size) = tokio::join!(
                fetch_maven_sha1(&url),
                fetch_file_size(&url)
            );

            libraries.push(Library {
                name: lib_entry.name.clone(),
                url: Some(url),
                path: Some(path),
                sha1,
                size,
            });
        }
    }

    // FIX: Forge sometimes references JARs in arguments that aren't in the libraries list
    // Parse arguments to find missing library references and add them
    let missing_libs = find_missing_libraries_from_arguments(full_data, &libraries);
    libraries.extend(missing_libs);

    Ok(libraries)
}

fn extract_arguments(full_data: &ForgeMetaData) -> Arguments {
    let manifest = &full_data.version_manifest;

    // Handle legacy minecraftArguments (pre-1.13)
    if let Some(legacy_args) = &manifest.minecraft_arguments {
        return Arguments {
            game: legacy_args.split_whitespace().map(String::from).collect(),
            jvm: None,
        };
    }

    // Modern arguments format
    Arguments {
        game: process_forge_arguments(&manifest.arguments.game),
        jvm: Some(process_forge_arguments(&manifest.arguments.jvm)),
    }
}

fn process_forge_arguments(args: &[ForgeArgument]) -> Vec<String> {
    let mut result = Vec::new();

    for arg in args {
        match arg {
            ForgeArgument::Simple(s) => result.push(s.clone()),
            ForgeArgument::Conditional(cond) => {
                if should_apply_rules(&cond.rules) {
                    match &cond.value {
                        ForgeArgumentValue::Single(s) => result.push(s.clone()),
                        ForgeArgumentValue::Multiple(v) => result.extend(v.clone()),
                    }
                }
            }
        }
    }

    result
}

fn extract_main_class(full_data: &ForgeMetaData) -> MainClass {
    MainClass {
        main_class: full_data.version_manifest.main_class.clone(),
    }
}

// ========== Helper Functions ==========

fn build_forge_installer_url<V: VersionInfo>(version: &V) -> String {
    format!(
        "https://maven.minecraftforge.net/net/minecraftforge/forge/{}-{}/forge-{}-{}-installer.jar",
        version.minecraft_version(),
        version.loader_version(),
        version.minecraft_version(),
        version.loader_version()
    )
}

async fn should_download_installer(installer_path: &PathBuf, installer_url: &str) -> bool {
    if !installer_path.exists() {
        return true;
    }

    // Verify SHA1 if file exists
    match verify_installer_sha1(installer_path, installer_url).await {
        Ok(true) => {
            lighty_core::trace_debug!("Forge installer already exists and SHA1 is valid");
            false
        }
        Ok(false) => {
            lighty_core::trace_debug!("Forge installer exists but SHA1 mismatch, re-downloading");
            true
        }
        Err(e) => {
            lighty_core::trace_debug!("Could not verify SHA1 ({}), using existing file", e);
            false
        }
    }
}

async fn read_forge_metadata_from_jar(installer_path: &PathBuf) -> Result<ForgeMetaData> {
    let path = installer_path.clone();

    tokio::task::spawn_blocking(move || {
        let file = File::open(&path).map_err(|e| QueryError::Conversion {
            message: format!("Failed to open Forge installer JAR: {}", e),
        })?;

        let mut archive = ZipArchive::new(file).map_err(|e| QueryError::Conversion {
            message: format!("Failed to open ZIP archive: {}", e),
        })?;

        // Read install_profile.json
        let install_profile = {
            let mut file = archive
                .by_name("install_profile.json")
                .map_err(|_| QueryError::MissingField {
                    field: "install_profile.json in installer JAR".to_string(),
                })?;

            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .map_err(|e| QueryError::Conversion {
                    message: format!("Failed to read install_profile.json: {}", e),
                })?;

            serde_json::from_str::<ForgeInstallProfile>(&contents)?
        };

        // Read version.json
        let version_manifest = {
            let mut file = archive.by_name("version.json").map_err(|_| {
                QueryError::MissingField {
                    field: "version.json in installer JAR".to_string(),
                }
            })?;

            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .map_err(|e| QueryError::Conversion {
                    message: format!("Failed to read version.json: {}", e),
                })?;

            serde_json::from_str::<ForgeVersionManifest>(&contents)?
        };

        Ok(ForgeMetaData {
            install_profile,
            version_manifest,
        })
    })
    .await
    .map_err(|e| QueryError::Conversion {
        message: format!("Failed to spawn blocking task: {}", e),
    })?
}

fn maven_artifact_to_path_and_url(maven_name: &str, base_url: &str) -> (String, String) {
    let mut parts = maven_name.split(':');

    let (group_id, artifact_id, version) = match (parts.next(), parts.next(), parts.next()) {
        (Some(g), Some(a), Some(v)) => (g, a, v),
        _ => return (String::new(), String::new()),
    };

    // Convert group.id to path (e.g., "net.minecraftforge" -> "net/minecraftforge")
    let group_path = group_id.replace('.', "/");

    // Build JAR filename
    let jar_name = format!("{}-{}.jar", artifact_id, version);

    // Build relative path
    let path = format!("{}/{}/{}/{}", group_path, artifact_id, version, jar_name);

    // Build full URL
    let base = base_url.trim_end_matches('/');
    let full_url = format!("{}/{}", base, path);

    (path, full_url)
}

fn extract_base_url(full_url: &str) -> String {
    // Extract base URL from a full artifact URL
    // e.g., "https://maven.minecraftforge.net/.../artifact.jar" -> "https://maven.minecraftforge.net/"
    if let Some(idx) = full_url.rfind("/net/") {
        full_url[..idx + 1].to_string()
    } else if let Some(idx) = full_url.rfind("/com/") {
        full_url[..idx + 1].to_string()
    } else if let Some(idx) = full_url.rfind("/org/") {
        full_url[..idx + 1].to_string()
    } else {
        "https://maven.minecraftforge.net/".to_string()
    }
}

async fn fetch_maven_sha1(jar_url: &str) -> Option<String> {
    let sha1_url = format!("{}.sha1", jar_url);

    match CLIENT.get(&sha1_url).send().await {
        Ok(response) if response.status().is_success() => {
            response.text().await.ok().and_then(|text| {
                let sha1 = text.trim().split_whitespace().next()?.to_string();
                (sha1.len() == 40).then_some(sha1)
            })
        }
        _ => None,
    }
}

async fn fetch_file_size(url: &str) -> Option<u64> {
    CLIENT
        .head(url)
        .send()
        .await
        .ok()?
        .headers()
        .get("content-length")?
        .to_str()
        .ok()?
        .parse()
        .ok()
}

fn calculate_file_sha1(path: &PathBuf) -> Result<String> {
    let mut file = File::open(path).map_err(|e| QueryError::Conversion {
        message: format!("Failed to open file for SHA1 calculation: {}", e),
    })?;

    let mut hasher = Sha1::new();
    let mut buffer = [0u8; 8192];

    loop {
        let n = file.read(&mut buffer).map_err(|e| QueryError::Conversion {
            message: format!("Failed to read file for SHA1 calculation: {}", e),
        })?;

        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    Ok(hex::encode(hasher.finalize()))
}

async fn verify_installer_sha1(installer_path: &PathBuf, installer_url: &str) -> Result<bool> {
    // Fetch expected SHA1
    let expected_sha1 = fetch_maven_sha1(installer_url)
        .await
        .ok_or_else(|| QueryError::Conversion {
            message: "Failed to fetch SHA1 from Maven".to_string(),
        })?;

    // Calculate local file SHA1
    let actual_sha1 = calculate_file_sha1(installer_path)?;

    Ok(expected_sha1.eq_ignore_ascii_case(&actual_sha1))
}

fn should_apply_rules(rules: &[super::forge_metadata::ForgeRule]) -> bool {
    let mut allowed = false;

    for rule in rules {
        let matches = if let Some(os) = &rule.os {
            check_os_rule(os)
        } else {
            true
        };

        if matches {
            allowed = rule.action == "allow";
        }
    }

    allowed
}

fn check_os_rule(os_rule: &super::forge_metadata::ForgeOsRule) -> bool {
    if let Some(name) = &os_rule.name {
        let current_os = if cfg!(target_os = "windows") {
            "windows"
        } else if cfg!(target_os = "macos") {
            "osx"
        } else if cfg!(target_os = "linux") {
            "linux"
        } else {
            return false;
        };

        if name != current_os {
            return false;
        }
    }

    // TODO: Check version and arch if needed
    true
}

/// Finds libraries referenced in arguments but missing from the libraries list
/// This fixes Forge bugs where JARs are used in module path but not declared as dependencies
fn find_missing_libraries_from_arguments(
    full_data: &ForgeMetaData,
    existing_libraries: &[Library],
) -> Vec<Library> {
    let mut missing = Vec::new();
    let manifest = &full_data.version_manifest;

    // Create a set of existing library paths for fast lookup
    let existing_paths: std::collections::HashSet<String> = existing_libraries
        .iter()
        .filter_map(|lib| lib.path.clone())
        .collect();

    // Parse all JVM arguments looking for ${library_directory}/path/to/jar references
    for arg in &manifest.arguments.jvm {
        let arg_strings = process_forge_arguments(&[arg.clone()]);
        for arg_str in arg_strings {
            // Find all ${library_directory}/... patterns
            let mut start_idx = 0;
            while let Some(lib_start) = arg_str[start_idx..].find("${library_directory}/") {
                let absolute_start = start_idx + lib_start + "${library_directory}/".len();
                
                // Find the end of the path (either separator, quote, or end of string)
                let remaining = &arg_str[absolute_start..];
                let path_end = remaining
                    .find("${classpath_separator}")
                    .or_else(|| remaining.find(';'))
                    .or_else(|| remaining.find(':'))
                    .or_else(|| remaining.find('"'))
                    .or_else(|| remaining.find(' '))
                    .unwrap_or(remaining.len());

                let jar_path = &remaining[..path_end];
                
                // Only process .jar files
                if jar_path.ends_with(".jar") && !existing_paths.contains(jar_path) {
                    // Try to extract Maven coordinates from path
                    // Path format: group/artifact/version/artifact-version.jar
                    if let Some(library) = path_to_library(jar_path) {
                        lighty_core::trace_warn!(
                            jar_path = %jar_path,
                            maven_name = %library.name,
                            "Forge references JAR in arguments but not in libraries - adding automatically"
                        );
                        missing.push(library);
                    }
                }

                start_idx = absolute_start + path_end;
            }
        }
    }

    missing
}

/// Converts a library path to a Library struct with Maven coordinates
/// Example: net/sf/jopt-simple/jopt-simple/6.0-alpha-3/jopt-simple-6.0-alpha-3.jar
///       -> net.sf.jopt-simple:jopt-simple:6.0-alpha-3
fn path_to_library(path: &str) -> Option<Library> {
    let parts: Vec<&str> = path.split('/').collect();
    if parts.len() < 4 {
        return None;
    }

    // Extract version (second-to-last directory)
    let version = parts[parts.len() - 2];
    
    // Extract artifact (third-to-last directory)
    let artifact = parts[parts.len() - 3];
    
    // Extract group (everything before artifact)
    let group = parts[..parts.len() - 3].join(".");

    let maven_name = format!("{}:{}:{}", group, artifact, version);
    let maven_url = format!("https://repo1.maven.org/maven2/{}", path);

    Some(Library {
        name: maven_name,
        url: Some(maven_url),
        path: Some(path.to_string()),
        sha1: None, // Will be fetched during download
        size: None,
    })
}