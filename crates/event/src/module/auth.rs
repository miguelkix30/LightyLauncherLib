// Copyright (c) 2025 Hamadi
// Licensed under the MIT License

//! Authentication events

use serde::{Deserialize, Serialize};

/// Authentication events
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event")]
pub enum AuthEvent {
    /// Authentication process begins
    AuthenticationStarted {
        provider: String,
    },
    /// Authentication is ongoing (generic progress)
    AuthenticationInProgress {
        provider: String,
        step: String,
    },
    /// Authentication succeeded
    AuthenticationSuccess {
        provider: String,
        username: String,
        uuid: String,
    },
    /// Authentication failed
    AuthenticationFailed {
        provider: String,
        error: String,
    },
    /// Valid session already exists (skip auth)
    AlreadyAuthenticated {
        provider: String,
        username: String,
    },
}
