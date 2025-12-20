# Examples

## Basic Event Listener

```rust
use lighty_event::{EventBus, Event};

#[tokio::main]
async fn main()  {
    let event_bus = EventBus::new(1000);
    let mut receiver = event_bus.subscribe();

    tokio::spawn(async move {
        while let Ok(event) = receiver.next().await {
            match event {
                Event::DownloadProgress(e) => {
                    println!("Progress: {}/{}", e.current, e.total);
                }
                Event::InstanceLaunched(e) => {
                    println!("Launched: {}", e.instance_name);
                }
                _ => {}
            }
        }
    });

    Ok(())
}
```

## Console Streaming

```rust
use lighty_event::{Event, ConsoleStream};

while let Ok(event) = receiver.next().await {
    if let Event::ConsoleOutput(e) = event {
        match e.stream {
            ConsoleStream::Stdout => print!("[OUT] {}", e.line),
            ConsoleStream::Stderr => eprint!("[ERR] {}", e.line),
        }
    }
}
```

## Multi-Subscriber

```rust
let event_bus = EventBus::new(1000);

// UI subscriber
let mut ui_rx = event_bus.subscribe();
tokio::spawn(async move {
    while let Ok(event) = ui_rx.next().await {
        update_ui(event);
    }
});

// Logger subscriber
let mut log_rx = event_bus.subscribe();
tokio::spawn(async move {
    while let Ok(event) = log_rx.next().await {
        log_event(&event);
    }
});
```
