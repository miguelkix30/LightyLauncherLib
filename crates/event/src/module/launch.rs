// Copyright (c) 2025 Hamadi
// Licensed under the MIT License

//! Launch and installation events

use serde::{Deserialize, Serialize};

/// Launch and installation events
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event")]
pub enum LaunchEvent {
    /// All files already up-to-date (skip installation)
    IsInstalled {
        version: String,
    },
    /// Installation started
    InstallStarted {
        version: String,
        total_bytes: u64,
    },
    /// Installation progress (global)
    InstallProgress {
        bytes: u64,
    },
    /// Installation completed
    InstallCompleted {
        version: String,
        total_bytes: u64,
    },
    /// Game launch starting (before spawn)
    Launching {
        version: String,
    },
    /// Game process spawned successfully
    Launched {
        version: String,
        pid: u32,
    },
    /// Game launch failed
    NotLaunched {
        version: String,
        error: String,
    },
    /// Game process output
    ProcessOutput {
        pid: u32,
        stream: String, // "stdout" | "stderr"
        line: String,
    },
    /// Game process exited
    ProcessExited {
        pid: u32,
        exit_code: i32,
    },
    /// Mod resolver started — about to walk the user request list and
    /// query Modrinth/CurseForge for compatible releases.
    ModResolveStarted {
        request_count: usize,
    },
    /// One mod request is being fetched from its remote source.
    ModResolveFetching {
        source: String,     // "modrinth" | "curseforge"
        identifier: String, // slug or numeric id, as the user provided
    },
    /// A `required` dependency was discovered while fetching `parent`.
    /// The resolver will enqueue and fetch it next.
    ModResolveDependency {
        parent: String,
        dependency: String,
    },
    /// Resolver finished — `total_mods` is the count of pivot entries
    /// produced (user requests + transitive deps, deduplicated).
    ModResolveCompleted {
        total_mods: usize,
    },
}
