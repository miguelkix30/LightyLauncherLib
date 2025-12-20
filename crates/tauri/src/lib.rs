//! # DEPRECATED: lighty-tauri crate
//!
//! **This crate is deprecated and will be removed in a future version.**
//!
//! The Tauri integration has been moved to a separate repository to maintain
//! better separation of concerns and allow independent versioning.
//!
//! Please migrate to the new standalone Tauri plugin repository.

#![deprecated(
    since = "0.9.0",
    note = "This crate is deprecated. Use the standalone Tauri plugin repository instead."
)]

pub mod commands;
pub mod core;

#[cfg(feature = "events")]
pub mod events;

// Re-export for convenience - use public wildcard exports
#[deprecated(since = "0.9.0", note = "Use the standalone Tauri plugin instead")]
pub use commands::auth::*;
#[deprecated(since = "0.9.0", note = "Use the standalone Tauri plugin instead")]
pub use commands::core::*;
#[deprecated(since = "0.9.0", note = "Use the standalone Tauri plugin instead")]
pub use commands::java::*;
#[deprecated(since = "0.9.0", note = "Use the standalone Tauri plugin instead")]
pub use commands::launch::*;
#[deprecated(since = "0.9.0", note = "Use the standalone Tauri plugin instead")]
pub use commands::loaders::*;
#[deprecated(since = "0.9.0", note = "Use the standalone Tauri plugin instead")]
pub use commands::version::*;
#[deprecated(since = "0.9.0", note = "Use the standalone Tauri plugin instead")]
pub use core::*;

#[cfg(feature = "events")]
#[deprecated(since = "0.9.0", note = "Use the standalone Tauri plugin instead")]
pub use events::*;

#[cfg(feature = "tauri-commands")]
use tauri::{plugin::{Builder, TauriPlugin}, Runtime};

/// Creates the Lighty Launcher Tauri plugin with all commands registered
///
/// # Deprecated
///
/// This function is deprecated. Please use the standalone Tauri plugin repository instead.
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
#[deprecated(
    since = "0.9.0",
    note = "Use the standalone Tauri plugin repository instead. This crate will be removed in a future version."
)]
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

