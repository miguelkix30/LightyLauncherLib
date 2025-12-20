# lighty-auth

Multi-provider authentication system for Minecraft launchers with OAuth2 and CMS integrations.

## Overview

`lighty-auth` provides a unified trait-based authentication system supporting multiple providers:
- **Offline Mode** - Local authentication without network, generates deterministic UUIDs
- **Microsoft Account** - OAuth 2.0 Device Code Flow via Microsoft/Xbox Live/Minecraft Services
- **Azuriom CMS** - Server authentication with 2FA support, roles, and permissions
- **Custom Providers** - Implement the `Authenticator` trait for your own auth system

## Quick Start

```toml
[dependencies]
lighty-auth = "0.8.6"
```

### Offline Authentication

```rust
use lighty_auth::{offline::OfflineAuth, Authenticator};

#[tokio::main]
async fn main() {
    let mut auth = OfflineAuth::new("PlayerName");
    let profile = auth.authenticate().await.unwrap();

    println!("Username: {}", profile.username);
    println!("UUID: {}", profile.uuid);
}
```

### Microsoft Authentication

```rust
use lighty_auth::{microsoft::MicrosoftAuth, Authenticator};

#[tokio::main]
async fn main() {
    let mut auth = MicrosoftAuth::new("your-azure-client-id");

    // Display device code to user
    auth.set_device_code_callback(|code, url| {
        println!("Please visit: {}", url);
        println!("And enter code: {}", code);
    });

    let profile = auth.authenticate().await.unwrap();
    println!("Logged in as: {}", profile.username);
}
```

### Azuriom Authentication

```rust
use lighty_auth::{azuriom::AzuriomAuth, Authenticator, AuthError};

#[tokio::main]
async fn main() {
    let mut auth = AzuriomAuth::new(
        "https://your-server.com",
        "user@example.com",
        "password123"
    );

    match auth.authenticate().await {
        Ok(profile) => {
            println!("Logged in as: {}", profile.username);
            if let Some(role) = profile.role {
                println!("Role: {}", role.name);
            }
        }
        Err(AuthError::TwoFactorRequired) => {
            auth.set_two_factor_code("123456");
            let profile = auth.authenticate().await.unwrap();
            println!("Logged in with 2FA: {}", profile.username);
        }
        Err(e) => eprintln!("Authentication failed: {}", e),
    }
}
```

## Authentication Providers

| Provider | Status | Network | Use Cases |
|----------|--------|---------|-----------|
| **Offline** | ✅ Stable | Not required | Testing, offline play, development |
| **Microsoft** | ✅ Stable | Required | Legitimate Minecraft accounts |
| **Azuriom** | ✅ Stable | Required | Custom server authentication, launcher whitelisting |

## Documentation

📚 **[Complete Documentation](./docs)**

| Guide | Description |
|-------|-------------|
| [Overview](./docs/overview.md) | Authentication system design and patterns |
| [Offline Authentication](./docs/offline.md) | Offline mode implementation and UUID generation |
| [Microsoft OAuth2](./docs/microsoft.md) | Complete Microsoft authentication flow guide |
| [Azuriom Integration](./docs/azuriom.md) | Azuriom CMS authentication with 2FA |
| [Examples](./docs/examples.md) | Complete usage examples and patterns |

## License

MIT

## Links

- **Main Package**: [lighty-launcher](https://crates.io/crates/lighty-launcher)
- **Repository**: [GitHub](https://github.com/Lighty-Launcher/LightyLauncherLib)
- **Documentation**: [docs.rs/lighty-auth](https://docs.rs/lighty-auth)
