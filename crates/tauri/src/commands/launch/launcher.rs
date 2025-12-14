use crate::commands::utils::parse::{parse_java_distribution, parse_loader};
use crate::core::{AppState, LaunchConfig, LaunchResult, VersionConfig};
use lighty_launch::launch::Launch;
use lighty_version::VersionBuilder as Version;
use lighty_auth::UserProfile;

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

    // Create UserProfile from username and uuid
    let profile = UserProfile {
        id: None,
        username: launch_config.username.clone(),
        uuid: launch_config.uuid,
        access_token: None,
        email: None,
        email_verified: false,
        money: None,
        role: None,
        banned: false,
    };

    match version
        .launch(&profile, java_dist)
        .run()
        .await
    {
        Ok(()) => Ok(LaunchResult {
            success: true,
            message: format!("Game launched successfully for {}", launch_config.username),
        }),
        Err(e) => Err(format!("Launch failed: {:?}", e)),
    }
}
