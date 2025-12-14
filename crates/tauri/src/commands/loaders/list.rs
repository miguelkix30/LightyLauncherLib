use crate::core::LoaderInfo;

#[tauri::command]
pub fn get_loaders() -> Vec<LoaderInfo> {
    vec![
        LoaderInfo {
            name: "vanilla".to_string(),
            display_name: "Vanilla".to_string(),
        },
        LoaderInfo {
            name: "fabric".to_string(),
            display_name: "Fabric".to_string(),
        },
        LoaderInfo {
            name: "quilt".to_string(),
            display_name: "Quilt".to_string(),
        },
        LoaderInfo {
            name: "neoforge".to_string(),
            display_name: "NeoForge".to_string(),
        },
        LoaderInfo {
            name: "forge".to_string(),
            display_name: "Forge".to_string(),
        },
        LoaderInfo {
            name: "optifine".to_string(),
            display_name: "OptiFine".to_string(),
        },
        LoaderInfo {
            name: "lighty_updater".to_string(),
            display_name: "LightyUpdater".to_string(),
        },
    ]
}
