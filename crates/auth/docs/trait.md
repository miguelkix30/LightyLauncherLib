# Authenticator Trait

## Overview

The `Authenticator` trait is the core interface for all authentication providers in `lighty-auth`. Implement this trait to create custom authentication systems.

**Export**: `lighty_auth::Authenticator`

## Trait Definition

```rust
pub trait Authenticator {
    #[cfg(feature = "events")]
    async fn authenticate(
        &mut self,
        event_bus: Option<&EventBus>,
    ) -> AuthResult<UserProfile>;

    #[cfg(not(feature = "events"))]
    async fn authenticate(&mut self) -> AuthResult<UserProfile>;
}
```

## Implementing Custom Authenticator

### Basic Implementation

```rust
use lighty_auth::{Authenticator, UserProfile, AuthResult, AuthError};
use lighty_core::hosts::HTTP_CLIENT;

pub struct CustomAuth {
    api_url: String,
    api_key: String,
    username: String,
}

impl CustomAuth {
    pub fn new(api_url: &str, api_key: &str, username: &str) -> Self {
        Self {
            api_url: api_url.to_string(),
            api_key: api_key.to_string(),
            username: username.to_string(),
        }
    }
}

impl Authenticator for CustomAuth {
    #[cfg(not(feature = "events"))]
    async fn authenticate(&mut self) -> AuthResult<UserProfile> {
        // Make API request
        let response = HTTP_CLIENT
            .get(format!("{}/auth", self.api_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .map_err(|e| AuthError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(AuthError::InvalidCredentials);
        }

        let data: serde_json::Value = response.json().await
            .map_err(|e| AuthError::ParseError(e.to_string()))?;

        Ok(UserProfile {
            id: data["id"].as_u64(),
            username: self.username.clone(),
            uuid: data["uuid"].as_str().unwrap_or("").to_string(),
            access_token: data["token"].as_str().map(String::from),
            email: None,
            email_verified: false,
            money: None,
            role: None,
            banned: false,
        })
    }

    #[cfg(feature = "events")]
    async fn authenticate(
        &mut self,
        event_bus: Option<&lighty_event::EventBus>,
    ) -> AuthResult<UserProfile> {
        use lighty_event::{Event, AuthEvent};
        use lighty_auth::AuthProvider;

        // Emit start event
        if let Some(bus) = event_bus {
            bus.emit(Event::Auth(AuthEvent::AuthenticationStarted {
                provider: AuthProvider::Custom {
                    base_url: self.api_url.clone(),
                },
            }));
        }

        // Perform authentication (same logic as non-events version)
        let profile = self.do_authenticate().await;

        // Emit result event
        if let Some(bus) = event_bus {
            match &profile {
                Ok(p) => {
                    bus.emit(Event::Auth(AuthEvent::AuthenticationSuccess {
                        username: p.username.clone(),
                        provider: AuthProvider::Custom {
                            base_url: self.api_url.clone(),
                        },
                    }));
                }
                Err(e) => {
                    bus.emit(Event::Auth(AuthEvent::AuthenticationFailed {
                        error: e.to_string(),
                        provider: AuthProvider::Custom {
                            base_url: self.api_url.clone(),
                        },
                    }));
                }
            }
        }

        profile
    }
}

impl CustomAuth {
    async fn do_authenticate(&self) -> AuthResult<UserProfile> {
        // Shared authentication logic
        todo!()
    }
}
```

## Usage

```rust
use lighty_auth::Authenticator;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut auth = CustomAuth::new(
        "https://api.example.com",
        "your-api-key",
        "username"
    );

    #[cfg(not(feature = "events"))]
    let profile = auth.authenticate().await?;

    #[cfg(feature = "events")]
    {
        use lighty_event::EventBus;
        let event_bus = EventBus::new(1000);
        let profile = auth.authenticate(Some(&event_bus)).await?;
    }

    println!("Authenticated: {}", profile.username);

    Ok(())
}
```

## Related Documentation

- [How to Use](./how-to-use.md) - Examples of using built-in authenticators
- [Events](./events.md) - AuthEvent types to emit
- [Exports](./exports.md) - All exported types
