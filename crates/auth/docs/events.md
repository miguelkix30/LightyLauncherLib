# Events

## Overview

`lighty-auth` emits `AuthEvent` types through the event bus system provided by `lighty-event`. These events track authentication flow progress.

**Feature**: Requires `events` feature flag

**Export**:
- Event types: `lighty_event::AuthEvent`
- Re-export: `lighty_launcher::event::AuthEvent`

## AuthEvent Types

### AuthenticationStarted

Emitted when authentication process begins.

**Fields**:
- `provider: AuthProvider` - The authentication provider being used

**When emitted**: At the start of any `authenticate()` call

**Example**:
```rust
use lighty_event::{EventBus, Event, AuthEvent};
use lighty_auth::{offline::OfflineAuth, Authenticator};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let event_bus = EventBus::new(1000);
    let mut receiver = event_bus.subscribe();

    tokio::spawn(async move {
        while let Ok(event) = receiver.next().await {
            if let Event::Auth(AuthEvent::AuthenticationStarted { provider }) = event {
                println!("Starting authentication with: {:?}", provider);
            }
        }
    });

    let mut auth = OfflineAuth::new("Player");
    auth.authenticate(Some(&event_bus)).await?;

    Ok(())
}
```

### DeviceCodeReceived

Emitted when Microsoft device code is received (Microsoft only).

**Fields**:
- `code: String` - The device code to display to user
- `url: String` - The URL user should visit
- `expires_in: u64` - Seconds until code expires

**When emitted**: During Microsoft authentication after device code request

**Example**:
```rust
use lighty_event::{EventBus, Event, AuthEvent};
use lighty_auth::{microsoft::MicrosoftAuth, Authenticator};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let event_bus = EventBus::new(1000);
    let mut receiver = event_bus.subscribe();

    tokio::spawn(async move {
        while let Ok(event) = receiver.next().await {
            if let Event::Auth(AuthEvent::DeviceCodeReceived { code, url, expires_in }) = event {
                println!("Visit: {}", url);
                println!("Enter code: {}", code);
                println!("Expires in: {}s", expires_in);
            }
        }
    });

    let mut auth = MicrosoftAuth::new("client-id");
    auth.authenticate(Some(&event_bus)).await?;

    Ok(())
}
```

### WaitingForUser

Emitted while polling for user to complete authentication.

**Fields**: None

**When emitted**: During Microsoft authentication polling loop

**Example**:
```rust
use lighty_event::{EventBus, Event, AuthEvent};

tokio::spawn(async move {
    while let Ok(event) = receiver.next().await {
        if let Event::Auth(AuthEvent::WaitingForUser) = event {
            println!("⏳ Waiting for user...");
        }
    }
});
```

### AuthenticationSuccess

Emitted when authentication completes successfully.

**Fields**:
- `username: String` - Authenticated username
- `provider: AuthProvider` - Provider used

**When emitted**: After successful authentication, before returning profile

**Example**:
```rust
use lighty_event::{EventBus, Event, AuthEvent};

tokio::spawn(async move {
    while let Ok(event) = receiver.next().await {
        if let Event::Auth(AuthEvent::AuthenticationSuccess { username, provider }) = event {
            println!("✓ Logged in as {} via {:?}", username, provider);
        }
    }
});
```

### AuthenticationFailed

Emitted when authentication fails.

**Fields**:
- `error: String` - Error message
- `provider: AuthProvider` - Provider that failed

**When emitted**: When authentication fails with error

**Example**:
```rust
use lighty_event::{EventBus, Event, AuthEvent};

tokio::spawn(async move {
    while let Ok(event) = receiver.next().await {
        if let Event::Auth(AuthEvent::AuthenticationFailed { error, provider }) = event {
            eprintln!("✗ Auth failed for {:?}: {}", provider, error);
        }
    }
});
```

## Complete Event Flow

### Offline Authentication

```
AuthenticationStarted
    ↓
AuthenticationSuccess
```

### Microsoft Authentication (Success)

```
AuthenticationStarted
    ↓
DeviceCodeReceived
    ↓
WaitingForUser
    ↓
WaitingForUser (repeated)
    ↓
AuthenticationSuccess
```

### Microsoft Authentication (Failure)

```
AuthenticationStarted
    ↓
DeviceCodeReceived
    ↓
WaitingForUser
    ↓
AuthenticationFailed
```

### Azuriom Authentication

```
AuthenticationStarted
    ↓
AuthenticationSuccess or AuthenticationFailed
```

## Related Documentation

- [How to Use](./how-to-use.md) - Practical authentication examples with events
- [Exports](./exports.md) - Complete export reference
- [lighty-event Events](../../event/docs/events.md) - All event types
