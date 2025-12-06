# lighty-auth

Authentication modules for [LightyLauncher](https://crates.io/crates/lighty-launcher).

## Note

This is an internal crate for the LightyLauncher ecosystem. Most users should use the main [`lighty-launcher`](https://crates.io/crates/lighty-launcher) crate instead.

## Features

- **Microsoft Authentication**: OAuth2 flow for Microsoft accounts
- **Offline Authentication**: Local authentication for offline play
- **Azuriom Integration**: Authentication with Azuriom CMS

## Usage

```toml
[dependencies]
lighty-auth = "0.6.3"
```

```rust
use lighty_auth::offline::OfflineAuth;

#[tokio::main]
async fn main() {
    // Offline authentication
    let auth = OfflineAuth::new("PlayerName".to_string());
    let profile = auth.authenticate().await?;

    println!("UUID: {}", profile.uuid);
    println!("Username: {}", profile.username);
}
```

## Structure

```
lighty-auth/
└── src/
    ├── lib.rs          # Module declarations
    ├── offline.rs      # Offline authentication (Stable)
    ├── microsoft.rs    # Microsoft OAuth2 authentication (WIP)
    └── azuriom.rs      # Azuriom CMS authentication (WIP)
```

## Authentication Methods

### Offline Mode

Stable implementation for local/offline authentication.

```rust
use lighty_auth::offline::OfflineAuth;

let auth = OfflineAuth::new("PlayerName".to_string());
let profile = auth.authenticate().await?;
```

**Status**: Stable

### Microsoft Account

OAuth2 flow for Microsoft accounts (Xbox Live).

```rust
use lighty_auth::microsoft::MicrosoftAuth;

// Implementation in progress
```

**Status**: Work in Progress

### Azuriom CMS

Integration with Azuriom CMS authentication system.

```rust
use lighty_auth::azuriom::AzuriomAuth;

// Implementation in progress
```

**Status**: Work in Progress

## License

MIT

## Links

- **Main Package**: [lighty-launcher](https://crates.io/crates/lighty-launcher)
- **Repository**: [GitHub](https://github.com/Lighty-Launcher/LightyLauncherLib)
- **Documentation**: [docs.rs/lighty-auth](https://docs.rs/lighty-auth)
