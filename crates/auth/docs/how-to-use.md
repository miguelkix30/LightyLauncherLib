# How to Use lighty-auth

## Basic Usage

### Offline Authentication

For local development and testing without network access:

```rust
use lighty_auth::{offline::OfflineAuth, Authenticator};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create offline authenticator
    let mut auth = OfflineAuth::new("Player123");

    // Authenticate (without events)
    #[cfg(not(feature = "events"))]
    let profile = auth.authenticate().await?;

    // Authenticate (with events)
    #[cfg(feature = "events")]
    let profile = auth.authenticate(None).await?;

    println!("Username: {}", profile.username);
    println!("UUID: {}", profile.uuid); // Deterministic UUID

    Ok(())
}
```

**Key features**:
- No network required
- Deterministic UUID generation (same username = same UUID)
- Perfect for development and testing

### Microsoft Authentication

OAuth 2.0 Device Code Flow for legitimate Minecraft accounts:

```rust
use lighty_auth::{microsoft::MicrosoftAuth, Authenticator};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize with Azure client ID
    let mut auth = MicrosoftAuth::new("your-azure-client-id");

    // Set callback for device code display
    auth.set_device_code_callback(|code, url| {
        println!("\n=== Microsoft Login ===");
        println!("Visit: {}", url);
        println!("Enter code: {}", code);
        println!("======================\n");
    });

    // Authenticate (without events)
    #[cfg(not(feature = "events"))]
    let profile = auth.authenticate().await?;

    // Authenticate (with events)
    #[cfg(feature = "events")]
    let profile = auth.authenticate(None).await?;

    println!("Logged in as: {}", profile.username);
    println!("UUID: {}", profile.uuid);
    println!("Access token: {}", profile.access_token.unwrap_or_default());

    Ok(())
}
```

**Key features**:
- Device code flow (no embedded browser needed)
- Full Xbox Live and Minecraft Services integration
- Returns Minecraft access token for session validation

### Azuriom Authentication

Server-based authentication with custom CMS:

```rust
use lighty_auth::{azuriom::AzuriomAuth, Authenticator, AuthError};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize with server URL and credentials
    let mut auth = AzuriomAuth::new(
        "https://yourserver.com",
        "user@example.com",
        "password123"
    );

    // Authenticate (without events)
    #[cfg(not(feature = "events"))]
    match auth.authenticate().await {
        Ok(profile) => {
            println!("Logged in as: {}", profile.username);
            println!("Email: {}", profile.email.unwrap_or_default());
            println!("Money: {}", profile.money.unwrap_or(0.0));

            if let Some(role) = &profile.role {
                println!("Role: {} ({})", role.name, role.color.as_ref().unwrap_or(&"#FFFFFF".to_string()));
            }
        }
        Err(AuthError::TwoFactorRequired) => {
            println!("2FA required!");
            // Get code from user
            let code = "123456"; // Get from UI input
            auth.set_two_factor_code(code);

            // Retry authentication with 2FA code
            let profile = auth.authenticate().await?;
            println!("Logged in with 2FA: {}", profile.username);
        }
        Err(e) => {
            eprintln!("Authentication failed: {}", e);
        }
    }

    Ok(())
}
```

**Key features**:
- Two-factor authentication support
- User roles with colors
- Money/credits tracking
- Email verification status

## With Events

Track authentication progress with events:

```rust
use lighty_auth::{microsoft::MicrosoftAuth, Authenticator};
use lighty_event::{EventBus, Event, AuthEvent};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create event bus
    let event_bus = EventBus::new(1000);
    let mut receiver = event_bus.subscribe();

    // Spawn event listener
    tokio::spawn(async move {
        while let Ok(event) = receiver.next().await {
            match event {
                Event::Auth(AuthEvent::AuthenticationStarted { provider }) => {
                    println!("🔐 Starting authentication with: {:?}", provider);
                }
                Event::Auth(AuthEvent::DeviceCodeReceived { code, url, .. }) => {
                    println!("📱 Device code: {}", code);
                    println!("🌐 URL: {}", url);
                }
                Event::Auth(AuthEvent::WaitingForUser) => {
                    println!("⏳ Waiting for user to complete authentication...");
                }
                Event::Auth(AuthEvent::AuthenticationSuccess { username, provider }) => {
                    println!("✓ Successfully authenticated as {} via {:?}", username, provider);
                }
                Event::Auth(AuthEvent::AuthenticationFailed { error, .. }) => {
                    eprintln!("✗ Authentication failed: {}", error);
                }
                _ => {}
            }
        }
    });

    // Authenticate with events
    let mut auth = MicrosoftAuth::new("your-client-id");
    auth.set_device_code_callback(|code, url| {
        println!("Visit {} and enter {}", url, code);
    });

    let profile = auth.authenticate(Some(&event_bus)).await?;
    println!("Profile: {:?}", profile);

    Ok(())
}
```

## Custom Authenticator

Implement the `Authenticator` trait for your own authentication system:

```rust
use lighty_auth::{Authenticator, UserProfile, UserRole, AuthResult, AuthError, AuthProvider};
use lighty_core::hosts::HTTP_CLIENT;

#[cfg(feature = "events")]
use lighty_event::EventBus;

pub struct CustomAuth {
    api_url: String,
    username: String,
    password: String,
}

impl CustomAuth {
    pub fn new(api_url: &str, username: &str, password: &str) -> Self {
        Self {
            api_url: api_url.to_string(),
            username: username.to_string(),
            password: password.to_string(),
        }
    }
}

impl Authenticator for CustomAuth {
    #[cfg(feature = "events")]
    async fn authenticate(
        &mut self,
        event_bus: Option<&EventBus>,
    ) -> AuthResult<UserProfile> {
        // Emit start event
        if let Some(bus) = event_bus {
            use lighty_event::{Event, AuthEvent};
            bus.emit(Event::Auth(AuthEvent::AuthenticationStarted {
                provider: AuthProvider::Custom {
                    base_url: self.api_url.clone(),
                },
            }));
        }

        // Make API request
        let response = HTTP_CLIENT
            .post(format!("{}/api/login", self.api_url))
            .json(&serde_json::json!({
                "username": self.username,
                "password": self.password,
            }))
            .send()
            .await
            .map_err(|e| AuthError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(AuthError::InvalidCredentials);
        }

        let data: serde_json::Value = response.json().await
            .map_err(|e| AuthError::ParseError(e.to_string()))?;

        // Emit success event
        if let Some(bus) = event_bus {
            use lighty_event::{Event, AuthEvent};
            bus.emit(Event::Auth(AuthEvent::AuthenticationSuccess {
                username: self.username.clone(),
                provider: AuthProvider::Custom {
                    base_url: self.api_url.clone(),
                },
            }));
        }

        Ok(UserProfile {
            id: data["id"].as_u64(),
            username: self.username.clone(),
            uuid: data["uuid"].as_str().unwrap_or("").to_string(),
            access_token: data["token"].as_str().map(String::from),
            email: data["email"].as_str().map(String::from),
            email_verified: data["email_verified"].as_bool().unwrap_or(false),
            money: data["money"].as_f64(),
            role: data["role"].as_object().map(|r| UserRole {
                name: r["name"].as_str().unwrap_or("User").to_string(),
                color: r["color"].as_str().map(String::from),
            }),
            banned: data["banned"].as_bool().unwrap_or(false),
        })
    }

    #[cfg(not(feature = "events"))]
    async fn authenticate(&mut self) -> AuthResult<UserProfile> {
        // Same implementation without events
        // ...
        todo!()
    }
}
```

## Offline UUID Generation

Generate deterministic UUIDs for offline mode:

```rust
use lighty_auth::generate_offline_uuid;

fn main() {
    // Generate UUID from username
    let uuid1 = generate_offline_uuid("Player123");
    let uuid2 = generate_offline_uuid("Player123");
    let uuid3 = generate_offline_uuid("Different");

    println!("UUID 1: {}", uuid1);
    println!("UUID 2: {}", uuid2); // Same as UUID 1
    println!("UUID 3: {}", uuid3); // Different

    assert_eq!(uuid1, uuid2); // Always the same for same username
    assert_ne!(uuid1, uuid3); // Different for different username
}
```

**How it works**:
- Uses SHA1 hash of username
- Formats as Minecraft-compatible UUID with dashes
- Deterministic: same input always produces same output

## Error Handling

```rust
use lighty_auth::{microsoft::MicrosoftAuth, Authenticator, AuthError};

#[tokio::main]
async fn main() {
    let mut auth = MicrosoftAuth::new("client-id");
    auth.set_device_code_callback(|code, url| {
        println!("Code: {}, URL: {}", code, url);
    });

    #[cfg(not(feature = "events"))]
    match auth.authenticate().await {
        Ok(profile) => {
            println!("Success: {}", profile.username);
        }
        Err(AuthError::NetworkError(e)) => {
            eprintln!("Network error: {}", e);
        }
        Err(AuthError::InvalidCredentials) => {
            eprintln!("Invalid username or password");
        }
        Err(AuthError::TwoFactorRequired) => {
            eprintln!("2FA code required");
        }
        Err(AuthError::AccountBanned) => {
            eprintln!("Account is banned");
        }
        Err(AuthError::MicrosoftAuthFailed(msg)) => {
            eprintln!("Microsoft auth failed: {}", msg);
        }
        Err(AuthError::XboxLiveAuthFailed(msg)) => {
            eprintln!("Xbox Live auth failed: {}", msg);
        }
        Err(AuthError::MinecraftAuthFailed(msg)) => {
            eprintln!("Minecraft auth failed: {}", msg);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
```

## Feature Flags

```toml
[dependencies]
lighty-auth = { version = "0.8.6", features = ["events", "tracing"] }
```

Available features:
- `events` - Enables AuthEvent emission (requires lighty-event)
- `tracing` - Enables logging macros

## Exports

**In lighty_auth**:
```rust
use lighty_auth::{
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

**In lighty_launcher**:
```rust
use lighty_launcher::auth::{
    Authenticator,
    UserProfile,
    // ... etc
};
```

## Related Documentation

- [Overview](./overview.md) - Architecture and design
- [Events](./events.md) - AuthEvent types
- [Exports](./exports.md) - Complete export reference
- [Offline](./offline.md) - Offline authentication details
- [Microsoft](./microsoft.md) - Microsoft OAuth flow details
- [Azuriom](./azuriom.md) - Azuriom authentication details
- [Trait](./trait.md) - Implementing custom authenticators

## Related Crates

- **[lighty-event](../../event/README.md)** - Event system
- **[lighty-core](../../core/README.md)** - Hash utilities for offline UUID
- **[lighty-launch](../../launch/README.md)** - Uses UserProfile for launching
