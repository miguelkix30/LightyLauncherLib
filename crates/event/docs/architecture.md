# Event System Architecture

## Design

```mermaid
graph TD
    A[EventBus] -->|broadcast| B[Receiver 1: UI]
    A -->|broadcast| C[Receiver 2: Logger]
    A -->|broadcast| D[Receiver 3: Analytics]

    E[Auth Module] -->|emit| A
    F[Java Module] -->|emit| A
    G[Launch Module] -->|emit| A
    H[Loader Module] -->|emit| A

    style A fill:#4CAF50
    style E fill:#2196F3
    style F fill:#FF9800
    style G fill:#9C27B0
    style H fill:#FFC107
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
