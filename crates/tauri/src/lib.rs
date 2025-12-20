pub mod commands;
pub mod core;

#[cfg(feature = "events")]
pub mod events;

// Re-export for convenience - use public wildcard exports
pub use commands::auth::*;
pub use commands::core::*;
pub use commands::java::*;
pub use commands::launch::*;
pub use commands::loaders::*;
pub use commands::version::*;
pub use core::*;

#[cfg(feature = "events")]
pub use events::*;

#[cfg(feature = "tauri-commands")]
use tauri::{plugin::{Builder, TauriPlugin}, Runtime};

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

    lighty_core::trace_info!("[LightyLauncher] Initializing Tauri plugin...");

    let plugin = Builder::new("lighty-launcher")
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
        .setup(|_app, _api| {
            lighty_core::trace_info!("[LightyLauncher] Plugin setup complete!");
            lighty_core::trace_info!("[LightyLauncher] Registered commands:");
            lighty_core::trace_info!("   - plugin:lighty-launcher|init_app_state");
            lighty_core::trace_info!("   - plugin:lighty-launcher|get_launcher_path");
            lighty_core::trace_info!("   - plugin:lighty-launcher|authenticate_offline");
            lighty_core::trace_info!("   - plugin:lighty-launcher|authenticate_microsoft");
            lighty_core::trace_info!("   - plugin:lighty-launcher|authenticate_azuriom");
            lighty_core::trace_info!("   - plugin:lighty-launcher|launch");
            lighty_core::trace_info!("   - plugin:lighty-launcher|get_java_distributions");
            lighty_core::trace_info!("   - plugin:lighty-launcher|get_loaders");
            lighty_core::trace_info!("   - plugin:lighty-launcher|check_version_exists");
            Ok(())
        })
        .build();

    lighty_core::trace_info!("[LightyLauncher] Plugin built successfully!");
    plugin
}

