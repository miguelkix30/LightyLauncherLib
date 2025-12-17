pub mod commands;
pub mod core;

#[cfg(feature = "events")]
pub mod events;

// Re-export for convenience
pub use commands::*;
pub use core::*;

#[cfg(feature = "events")]
pub use events::*;

#[cfg(feature = "tauri-commands")]
use tauri::{plugin::TauriPlugin, Runtime};

/// Creates the Lighty Launcher Tauri plugin with all commands registered
///
/// # Example
/// ```no_run
/// use lighty_launcher::tauri::lighty_plugin;
///
/// #[cfg_attr(mobile, tauri::mobile_entry_point)]
/// pub fn run() {
///     tauri::Builder::default()
///         .plugin(lighty_plugin())
///         .run(tauri::generate_context!())
///         .expect("error running tauri application");
/// }
/// ```
#[cfg(feature = "tauri-commands")]
pub fn lighty_plugin<R: Runtime>() -> TauriPlugin<R> {
    use crate::commands::auth::*;
    use crate::commands::core::*;
    use crate::commands::java::*;
    use crate::commands::launch::*;
    use crate::commands::loaders::*;
    use crate::commands::version::*;

    println!("[LightyLauncher] Initializing Tauri plugin...");

    let plugin = tauri_plugin::Builder::new("lighty-launcher")
        .invoke_handler(tauri::generate_handler![
            init_app_state,
            get_launcher_path,
            authenticate_offline,
            authenticate_microsoft,
            authenticate_azuriom,
            launch,
            get_java_distributions,
            get_loaders,
            check_version_exists,
        ])
        .setup(|app, _api| {
            println!("[LightyLauncher] Plugin setup complete!");
            println!("[LightyLauncher] Registered commands:");
            println!("   - plugin:lighty-launcher|init_app_state");
            println!("   - plugin:lighty-launcher|get_launcher_path");
            println!("   - plugin:lighty-launcher|authenticate_offline");
            println!("   - plugin:lighty-launcher|authenticate_microsoft");
            println!("   - plugin:lighty-launcher|authenticate_azuriom");
            println!("   - plugin:lighty-launcher|launch");
            println!("   - plugin:lighty-launcher|get_java_distributions");
            println!("   - plugin:lighty-launcher|get_loaders");
            println!("   - plugin:lighty-launcher|check_version_exists");
            Ok(())
        })
        .build();

    println!("[LightyLauncher] Plugin built successfully!");
    plugin
}

