fn main() {
    tauri_plugin::Builder::new(&[
        "init_app_state",
        "get_launcher_path",
        "authenticate_offline",
        "authenticate_microsoft",
        "authenticate_azuriom",
        "launch",
        "get_java_distributions",
        "get_loaders",
        "check_version_exists",
    ])
    .build();
}
