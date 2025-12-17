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

    tauri::plugin::Builder::new("lighty-launcher")
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
        .build()
}

