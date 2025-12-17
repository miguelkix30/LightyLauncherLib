//! Tauri event integration for LightyLauncher EventBus
//!
//! This module provides utilities to bridge lighty-event's EventBus
//! to Tauri's frontend event system.

#[cfg(feature = "events")]
pub use crate::commands::events::events::{subscribe_to_events, TauriEvent};
