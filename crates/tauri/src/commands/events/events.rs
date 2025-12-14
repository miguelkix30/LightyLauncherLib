#[cfg(feature = "events")]
use lighty_event::{EventBus, Event};
#[cfg(feature = "events")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "events")]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TauriEvent {
    pub event_type: String,
    pub data: serde_json::Value,
}

#[cfg(feature = "events")]
impl TauriEvent {
    pub fn from_event(event: Event) -> Self {
        match event {
            Event::Auth(auth_event) => Self {
                event_type: "auth".to_string(),
                data: serde_json::to_value(auth_event).unwrap_or(serde_json::Value::Null),
            },
            Event::Java(java_event) => Self {
                event_type: "java".to_string(),
                data: serde_json::to_value(java_event).unwrap_or(serde_json::Value::Null),
            },
            Event::Launch(launch_event) => Self {
                event_type: "launch".to_string(),
                data: serde_json::to_value(launch_event).unwrap_or(serde_json::Value::Null),
            },
            Event::Loader(loader_event) => Self {
                event_type: "loader".to_string(),
                data: serde_json::to_value(loader_event).unwrap_or(serde_json::Value::Null),
            },
            Event::Core(core_event) => Self {
                event_type: "core".to_string(),
                data: serde_json::to_value(core_event).unwrap_or(serde_json::Value::Null),
            },
        }
    }
}

#[cfg(feature = "events")]
/// Subscribe to events from the EventBus and emit them as Tauri events
///
/// # Example
/// ```rust,no_run
/// use lighty_event::EventBus;
/// use lighty_tauri::events::subscribe_to_events;
///
/// #[tauri::command]
/// async fn launch_with_events(app: tauri::AppHandle) -> Result<(), String> {
///     let event_bus = EventBus::new(100);
///
///     // Spawn event listener
///     subscribe_to_events(app.clone(), event_bus.clone());
///
///     // Launch game with event_bus
///     // version.launch(&profile, java_dist)
///     //     .with_event_bus(&event_bus)
///     //     .run()
///     //     .await
///
///     Ok(())
/// }
/// ```
pub fn subscribe_to_events(app: tauri::AppHandle, event_bus: EventBus) {
    tokio::spawn(async move {
        let mut receiver = event_bus.subscribe();

        while let Ok(event) = receiver.recv().await {
            let tauri_event = TauriEvent::from_event(event);

            if let Err(e) = app.emit("lighty-event", &tauri_event) {
                tracing::error!("Failed to emit Tauri event: {:?}", e);
            }
        }
    });
}
