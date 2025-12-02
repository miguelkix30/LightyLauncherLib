use crate::tauri::core::JavaDistInfo;

#[tauri::command]
pub fn get_java_distributions() -> Vec<JavaDistInfo> {
    vec![
        JavaDistInfo {
            name: "temurin".to_string(),
            display_name: "Eclipse Temurin".to_string(),
        },
        JavaDistInfo {
            name: "graalvm".to_string(),
            display_name: "GraalVM".to_string(),
        },
    ]
}
