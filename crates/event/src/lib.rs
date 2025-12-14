// Copyright (c) 2025 Hamadi
// Licensed under the MIT License

//! Event system for LightyLauncher
//!
//! Simple broadcast-based event system that allows emitting events from the library
//! and subscribing to them from user code.
//!
//! # Example
//!
//! ```no_run
//! use lighty_event::{EventBus, Event, LaunchEvent, EventReceiveError};
//!
//! #[tokio::main]
//! async fn main() {
//!     let event_bus = EventBus::new(100);
//!     let mut receiver = event_bus.subscribe();
//!
//!     // Spawn a listener with error handling
//!     tokio::spawn(async move {
//!         loop {
//!             match receiver.next().await {
//!                 Ok(event) => {
//!                     match event {
//!                         Event::Launch(LaunchEvent::InstallProgress { bytes }) => {
//!                             println!("Downloaded {} bytes", bytes);
//!                         }
//!                         _ => {}
//!                     }
//!                 }
//!                 Err(EventReceiveError::BusDropped) => {
//!                     println!("Event bus closed");
//!                     break;
//!                 }
//!                 Err(EventReceiveError::Lagged { skipped }) => {
//!                     eprintln!("Receiver lagged, missed {} events", skipped);
//!                 }
//!             }
//!         }
//!     });
//!
//!     // Use the launcher with event_bus...
//! }
//! ```

use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

// Re-export event modules
mod errors;
pub mod module;

pub use errors::{
    EventReceiveError, EventReceiveResult, EventSendError, EventSendResult,
    EventTryReceiveError, EventTryReceiveResult,
};
pub use module::{AuthEvent, CoreEvent, JavaEvent, LaunchEvent, LoaderEvent};

/// Event bus for broadcasting events to multiple listeners
#[derive(Clone)]
pub struct EventBus {
    sender: broadcast::Sender<Event>,
}

impl EventBus {
    /// Create a new EventBus with the specified buffer capacity
    ///
    /// # Arguments
    /// - `capacity`: Maximum number of events to buffer. If this limit is exceeded
    ///               and receivers are slow, the oldest events will be dropped.
    ///
    /// # Recommended values
    /// - 100-1000 for most use cases
    /// - Higher values if you have very slow receivers
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self { sender }
    }

    /// Subscribe to events
    ///
    /// Returns an EventReceiver that will receive all future events emitted
    /// after this call.
    pub fn subscribe(&self) -> EventReceiver {
        EventReceiver {
            receiver: self.sender.subscribe(),
        }
    }

    /// Emit an event to all subscribers
    ///
    /// If there are no active subscribers, the event is silently dropped.
    pub fn emit(&self, event: Event) {
        let _ = self.sender.send(event);
    }
}

/// Receiver for events from an EventBus
pub struct EventReceiver {
    receiver: broadcast::Receiver<Event>,
}

impl EventReceiver {
    /// Wait for the next event (async blocking)
    ///
    /// Returns `Ok(Event)` when an event is received, or `Err(EventReceiveError)` if
    /// the EventBus is dropped or the receiver has lagged behind.
    ///
    /// # Errors
    /// - `EventReceiveError::BusDropped` - All event bus senders have been dropped
    /// - `EventReceiveError::Lagged` - The receiver fell behind and missed some events
    pub async fn next(&mut self) -> EventReceiveResult<Event> {
        self.receiver.recv().await.map_err(Into::into)
    }

    /// Try to receive an event without blocking
    ///
    /// Returns `Ok(Event)` if an event is immediately available.
    ///
    /// # Errors
    /// - `EventTryReceiveError::Empty` - No events available right now
    /// - `EventTryReceiveError::BusDropped` - All event bus senders have been dropped
    /// - `EventTryReceiveError::Lagged` - The receiver fell behind and missed some events
    pub fn try_next(&mut self) -> EventTryReceiveResult<Event> {
        self.receiver.try_recv().map_err(Into::into)
    }
}

/// Root event enum containing all event types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum Event {
    Auth(AuthEvent),
    Java(JavaEvent),
    Launch(LaunchEvent),
    Loader(LoaderEvent),
    Core(CoreEvent),
}
