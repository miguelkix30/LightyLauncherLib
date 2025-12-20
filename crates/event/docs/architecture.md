# Event System Architecture

## Design

```mermaid
flowchart TD
    E[Auth Module] -->|emit| A[EventBus]
    F[Java Module] -->|emit| A
    G[Launch Module] -->|emit| A
    H[Loader Module] -->|emit| A

    A -->|broadcast| B[Receiver 1: UI]
    A -->|broadcast| C[Receiver 2: Logger]
    A -->|broadcast| D[Receiver 3: Analytics]
```

## Flow

1. **Event Emission** - Modules emit events to EventBus
2. **Broadcasting** - EventBus broadcasts to all subscribers
3. **Processing** - Each receiver handles events independently

## Thread Safety

- Uses tokio `broadcast` channels
- Lock-free concurrent access
- Multiple subscribers supported

## See Also

- [Event Reference](./events.md)
- [Examples](./examples.md)
