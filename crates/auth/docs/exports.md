# Exports

## Overview

Complete reference of all exports from `lighty-auth` and their re-exports in `lighty-launcher`.

## In `lighty_auth`

### Core Trait

```rust
use lighty_auth::Authenticator;
```

### Types

```rust
use lighty_auth::{
    UserProfile,     // Authenticated user data
    UserRole,        // User role/rank information
    AuthProvider,    // Provider type enum
    AuthResult<T>,   // Result type alias
};
```

### Helper Functions

```rust
use lighty_auth::generate_offline_uuid;
```

### Authentication Providers

```rust
use lighty_auth::{
    offline::OfflineAuth,
    microsoft::MicrosoftAuth,
    azuriom::AzuriomAuth,
};
```

### Errors

```rust
use lighty_auth::AuthError;
```

## In `lighty_launcher` (Re-exports)

```rust
use lighty_launcher::auth::{
    // Trait
    Authenticator,

    // Types
    UserProfile,
    UserRole,
    AuthProvider,
    AuthResult,

    // Helper
    generate_offline_uuid,

    // Providers
    offline::OfflineAuth,
    microsoft::MicrosoftAuth,
    azuriom::AzuriomAuth,

    // Errors
    AuthError,
};
```

## Usage Patterns

### Pattern 1: Direct Crate Import

```rust
use lighty_auth::{Authenticator, offline::OfflineAuth};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut auth = OfflineAuth::new("Player");

    #[cfg(not(feature = "events"))]
    let profile = auth.authenticate().await?;

    println!("{}", profile.username);
    Ok(())
}
```

### Pattern 2: Via Main Launcher Crate

```rust
use lighty_launcher::auth::{Authenticator, microsoft::MicrosoftAuth};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut auth = MicrosoftAuth::new("client-id");

    #[cfg(not(feature = "events"))]
    let profile = auth.authenticate().await?;

    Ok(())
}
```

## Type Details

### UserProfile

```rust
pub struct UserProfile {
    pub id: Option<u64>,                 // Server-side user ID (Azuriom only)
    pub username: String,
    pub uuid: String,                    // Minecraft UUID, with dashes
    pub access_token: Option<String>,    // Session / MC access token
    pub xuid: Option<String>,            // Xbox User ID (Microsoft only)
    pub email: Option<String>,
    pub email_verified: bool,
    pub money: Option<f64>,
    pub role: Option<UserRole>,
    pub banned: bool,
    pub provider: AuthProvider,          // Which authenticator produced this profile
}
```

`UserProfile` is `Serialize` + `Deserialize`, so persisting the whole
struct (e.g. in an OS keyring) is the recommended way to enable
"remember me" without exposing extra surface from the library.

### UserRole

```rust
pub struct UserRole {
    pub name: String,
    pub color: Option<String>,        // Hex format: #RRGGBB
}
```

### AuthProvider

```rust
pub enum AuthProvider {
    Offline,
    Azuriom {
        base_url: String,
    },
    Microsoft {
        client_id: String,
        /// MS refresh token (~90 days, rotates per RFC 6749).
        /// Populated by the device-code flow and consumed by
        /// `MicrosoftAuth::authenticate_with_refresh_token` to skip
        /// the device-code prompt on subsequent launches.
        refresh_token: Option<String>,
    },
    Custom {
        base_url: String,
    },
}
```

The variant drives the `${user_type}` launch placeholder at JVM start:
`Microsoft` → `"msa"`, `Azuriom` → `"mojang"`, `Offline`/`Custom` →
`"legacy"`.

### AuthError

```rust
pub enum AuthError {
    InvalidCredentials,
    TwoFactorRequired,
    Invalid2FACode,
    AccountBanned(String),
    EmailNotVerified,
    Network(reqwest::Error),
    InvalidResponse(String),
    InvalidToken,
    Cancelled,
    DeviceCodeExpired,
    Timeout,
    Serialization(serde_json::Error),
    Io(std::io::Error),
    Custom(String),
}
```

## Module Structure

```
lighty_auth
├── auth
│   ├── Authenticator (trait)
│   ├── UserProfile
│   ├── UserRole
│   ├── AuthProvider
│   ├── AuthResult<T>
│   └── generate_offline_uuid
├── offline
│   └── OfflineAuth
├── microsoft
│   └── MicrosoftAuth
├── azuriom
│   └── AzuriomAuth
└── errors
    └── AuthError
```

## Related Documentation

- [How to Use](./how-to-use.md) - Practical usage examples
- [Events](./events.md) - AuthEvent types
- [Trait](./trait.md) - Implementing custom authenticators
- [Overview](./overview.md) - Architecture overview
