// Copyright (c) 2025 Hamadi
// Licensed under the MIT License

//! User-mod resolver — bridge between [`VersionBuilder.mod_requests`]
//! and the existing mods installer pipeline.
//!
//! Lives in the launch crate (not in loaders) because:
//! - it consumes [`lighty_event::EventBus`] to emit progress events;
//! - it produces the pivot `Vec<Mods>` that the launch crate then
//!   merges into [`Version.mods`] before [`super::mods::collect_mod_tasks`]
//!   takes over.

#![cfg(any(feature = "modrinth", feature = "curseforge"))]

use lighty_loaders::mods::resolver::{resolve, ResolveCallbacks};
use lighty_loaders::mods::request::ModRequest;
use lighty_loaders::types::version_metadata::Mods;
use lighty_loaders::types::Loader;

use crate::errors::InstallerResult;

#[cfg(feature = "events")]
use lighty_event::{Event, EventBus, LaunchEvent};

/// Resolves every user-attached [`ModRequest`] for an instance,
/// transitively pulling required dependencies, and returns the flat
/// list of pivot [`Mods`] entries ready for the standard mods installer.
///
/// Returns an empty vec when `requests` is empty (no network calls).
pub(crate) async fn resolve_user_mods(
    requests: &[ModRequest],
    mc_version: &str,
    loader: &Loader,
    #[cfg(feature = "events")] event_bus: Option<&EventBus>,
) -> InstallerResult<Vec<Mods>> {
    if requests.is_empty() {
        return Ok(Vec::new());
    }

    lighty_core::trace_info!(
        "[Installer] Resolving {} user mod request(s)...",
        requests.len()
    );

    #[cfg(feature = "events")]
    if let Some(bus) = event_bus {
        bus.emit(Event::Launch(LaunchEvent::ModResolveStarted {
            request_count: requests.len(),
        }));
    }

    // Capture the bus for the callbacks. Cheap clone — it's an Arc inside.
    #[cfg(feature = "events")]
    let bus_for_fetch = event_bus.cloned();
    #[cfg(feature = "events")]
    let bus_for_dep = event_bus.cloned();

    let on_fetch = move |provider: &'static str, ident: &str| {
        lighty_core::trace_debug!(provider = %provider, ident = %ident, "Fetching mod");
        #[cfg(feature = "events")]
        if let Some(ref bus) = bus_for_fetch {
            bus.emit(Event::Launch(LaunchEvent::ModResolveFetching {
                source: provider.to_string(),
                identifier: ident.to_string(),
            }));
        }
    };

    let on_dependency = move |parent: &str, dep: &str| {
        lighty_core::trace_debug!(parent = %parent, dep = %dep, "Enqueueing required mod dep");
        #[cfg(feature = "events")]
        if let Some(ref bus) = bus_for_dep {
            bus.emit(Event::Launch(LaunchEvent::ModResolveDependency {
                parent: parent.to_string(),
                dependency: dep.to_string(),
            }));
        }
    };

    let callbacks = ResolveCallbacks {
        on_fetch: &on_fetch,
        on_dependency: &on_dependency,
    };

    let resolved = resolve(requests, mc_version, loader, Some(&callbacks)).await?;

    #[cfg(feature = "events")]
    if let Some(bus) = event_bus {
        bus.emit(Event::Launch(LaunchEvent::ModResolveCompleted {
            total_mods: resolved.len(),
        }));
    }

    lighty_core::trace_info!(
        "[Installer] ✓ {} mod(s) resolved (user requests + transitive deps)",
        resolved.len()
    );

    Ok(resolved)
}

