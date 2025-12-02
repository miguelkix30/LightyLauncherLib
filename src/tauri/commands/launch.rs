use crate::tauri::commands::utils::parse::{parse_java_distribution, parse_loader};
use crate::tauri::core::{AppState, LaunchConfig, LaunchResult, VersionConfig};
use crate::{Launch, Version};

#[tauri::command]
pub async fn launch(
    version_config: VersionConfig,
    launch_config: LaunchConfig,
) -> Result<LaunchResult, String> {
    let loader = parse_loader(&version_config.loader)?;
    let java_dist = parse_java_distribution(&launch_config.java_distribution)?;

    let mut version = Version::new(
        &version_config.name,
        loader,
        &version_config.loader_version,
        &version_config.minecraft_version,
        AppState::get_project_dirs(),
    );

    match version
        .launch(&launch_config.username, &launch_config.uuid, java_dist)
        .await
    {
        Ok(()) => Ok(LaunchResult {
            success: true,
            message: format!("Game launched successfully for {}", launch_config.username),
        }),
        Err(e) => Err(format!("Launch failed: {:?}", e)),
    }
}
