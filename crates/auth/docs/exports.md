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
    pub id: Option<u64>,
    pub username: String,
    pub uuid: String,
    pub access_token: Option<String>,
    pub email: Option<String>,
    pub email_verified: bool,
    pub money: Option<f64>,
    pub role: Option<UserRole>,
    pub banned: bool,
}
```

### UserRole

```rust
pub struct UserRole {
    pub name: String,
    pub color: Option<String>,
}
```

### AuthProvider

```rust
pub enum AuthProvider {
    Offline,
    Azuriom { base_url: String },
    Microsoft { client_id: String },
    Custom { base_url: String },
}
```

### AuthError

```rust
pub enum AuthError {
    NetworkError(String),
    InvalidCredentials,
    TwoFactorRequired,
    AccountBanned,
    MicrosoftAuthFailed(String),
    XboxLiveAuthFailed(String),
    MinecraftAuthFailed(String),
    ParseError(String),
    AzuriomError(String),
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
