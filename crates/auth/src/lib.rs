// Copyright (c) 2025 Hamadi
// Licensed under the MIT License

//! Authentication module for Lighty Launcher
//!
//! Provides multiple authentication providers and a trait-based system for custom implementations.
//!
//! ## Built-in Providers
//!
//! - **Offline**: No network authentication, generates deterministic UUIDs from username
//! - **Microsoft**: OAuth 2.0 authentication via Microsoft/Xbox Live
//! - **Azuriom**: Authentication via Azuriom CMS API
//!
//! ## Custom Authentication
//!
//! Implement the `Authenticator` trait to create your own authentication provider:
//!
//! ```rust,ignore
//! use lighty_auth::{Authenticator, AuthProvider, UserProfile, AuthResult};
//!
//! pub struct MyCustomAuth {
//!     api_url: String,
//!     username: String,
//! }
//!
//! impl Authenticator for MyCustomAuth {
//!     async fn authenticate(
//!         &mut self,
//!         #[cfg(feature = "events")] _event_bus: Option<&lighty_event::EventBus>,
//!     ) -> AuthResult<UserProfile> {
//!         // ... your auth flow here ...
//!         Ok(UserProfile {
//!             id: None,
//!             username: self.username.clone(),
//!             uuid: "your-uuid".to_string(),
//!             access_token: Some("your-token".to_string()),
//!             xuid: None,
//!             email: None,
//!             email_verified: false,
//!             money: None,
//!             role: None,
//!             banned: false,
//!             provider: AuthProvider::Custom { base_url: self.api_url.clone() },
//!         })
//!     }
//! }
//! ```
//!
//! ## Helpers
//!
//! Use the `generate_offline_uuid()` function to create deterministic UUIDs:
//!
//! ```rust
//! use lighty_auth::generate_offline_uuid;
//!
//! let uuid = generate_offline_uuid("PlayerName");
//! println!("UUID: {}", uuid); // Always the same for this username
//! ```

mod auth;
mod errors;

pub mod offline;
pub mod azuriom;
pub mod microsoft;

// Re-export core types
pub use auth::{
    AuthProvider, AuthResult, Authenticator, UserProfile, UserRole, generate_offline_uuid,
};
pub use errors::AuthError;
