# LightyLauncher Documentation

Comprehensive documentation for LightyLauncher library.

## Quick Navigation

### 🎯 Getting Started

1. **[Main README](../README.md)** - Quick start and installation
2. **[Examples](./examples.md)** - Code examples with detailed explanations
3. **[Architecture](./architecture.md)** - Understand the system design

### 📚 Core Guides

| Guide | Purpose | Best For |
|-------|---------|----------|
| **[Sequence Diagrams](./sequence-diagrams.md)** | Visual workflows for all operations | Understanding flow and process |
| **[Re-exports Reference](./reexports.md)** | Complete API reference | Finding available types and functions |
| **[Architecture](./architecture.md)** | System design and patterns | Understanding internals |
| **[Examples](./examples.md)** | Example walkthroughs | Learning by example |

## Documentation Structure

```
docs/
├── README.md                 # This file
├── sequence-diagrams.md      # Visual workflows
├── reexports.md             # API reference
├── architecture.md          # System design
├── examples.md              # Example walkthroughs
└── img/                     # Images and diagrams
```

## Guide Details

### Sequence Diagrams

**File**: [sequence-diagrams.md](./sequence-diagrams.md)

Visual diagrams for:
- Complete launch sequence
- Authentication flows (Offline, Microsoft)
- Installation process (parallel downloads)
- Java management
- Instance control
- Console streaming
- Event flow
- Loader-specific sequences (Fabric, LightyUpdater)

**When to use**: Understanding how components interact and the order of operations.

### Re-exports Reference

**File**: [reexports.md](./reexports.md)

Complete reference of all types and functions re-exported by `lighty-launcher`:
- Module organization
- Type definitions
- Function signatures
- Import patterns
- Usage examples
- Links to detailed crate documentation

**When to use**: Finding what's available and how to import it.

### Architecture

**File**: [architecture.md](./architecture.md)

System architecture documentation:
- Module overview and dependencies
- Data flow diagrams
- Design patterns (Builder, Trait-based, Event-driven, etc.)
- Concurrency model
- Error handling strategy
- Platform support
- Performance optimizations
- Security considerations

**When to use**: Understanding the overall system design and implementation details.

### Examples

**File**: [examples.md](./examples.md)

Detailed walkthroughs of all examples:
- Example overview table
- Running instructions
- Code walkthroughs
- Key concepts explained
- Event types demonstrated
- Common patterns
- Troubleshooting

**When to use**: Learning by example and finding code snippets.

## By Use Case

### I want to...

#### Launch Vanilla Minecraft

1. Read: [Examples - vanilla.rs](./examples.md#vanillars)
2. See flow: [Sequence Diagrams - Complete Launch](./sequence-diagrams.md#complete-launch-sequence)
3. Run: `cargo run --example vanilla --features vanilla`

#### Use a Mod Loader (Fabric, Quilt, Forge)

1. Read: [Examples - fabric.rs](./examples.md#fabricrs)
2. See how loaders merge: [Sequence Diagrams - Fabric Loader](./sequence-diagrams.md#fabric-loader)
3. Check exports: [Re-exports - Loaders Module](./reexports.md#loaders-module-lighty_launcherloaders)

#### Track Progress with Events

1. Read: [Examples - with_events.rs](./examples.md#with_eventsrs)
2. See event flow: [Sequence Diagrams - Event Flow](./sequence-diagrams.md#event-flow-diagram)
3. Check event types: [Re-exports - Events Module](./reexports.md#events-module-lighty_launcherevent)

#### Manage Running Instances

1. Read: [Examples - Instance Control](./examples.md#instance-control-operations)
2. See instance lifecycle: [Sequence Diagrams - Instance Control](./sequence-diagrams.md#instance-control-sequence)
3. Check API: [Re-exports - Launch Module](./reexports.md#launch-module-lighty_launcherlaunch)

#### Understand System Design

1. Read: [Architecture](./architecture.md)
2. See module dependencies: [Architecture - Module Dependencies](./architecture.md#module-dependencies)
3. Check design patterns: [Architecture - Design Patterns](./architecture.md#design-patterns)

#### Create Custom Authentication

1. Read: [Re-exports - Authentication Module](./reexports.md#authentication-module-lighty_launcherauth)
2. See auth flow: [Sequence Diagrams - Authentication](./sequence-diagrams.md#authentication-sequence)
3. Check trait: [Architecture - Trait-Based Extensibility](./architecture.md#2-trait-based-extensibility)

#### Build a Custom Launcher UI

1. Read: [Architecture - System Overview](./architecture.md#system-overview)
2. Check Tauri integration: [Re-exports - Tauri Module](./reexports.md#tauri-module-lighty_launchertauri)
3. Use events: [Examples - with_events.rs](./examples.md#with_eventsrs)

## Crate-Specific Documentation

Each crate has its own detailed documentation:

### Core Crates

- **[lighty-core](../crates/core/README.md)** - Core utilities and AppState
- **[lighty-launcher](../README.md)** - Main package with re-exports

### Feature Crates

- **[lighty-auth](../crates/auth/README.md)** - Authentication providers
- **[lighty-event](../crates/event/README.md)** - Event system
- **[lighty-java](../crates/java/README.md)** - Java runtime management
- **[lighty-launch](../crates/launch/README.md)** - Launch orchestration
  - [Launch Process](../crates/launch/docs/launch.md)
  - [Arguments System](../crates/launch/docs/arguments.md)
  - [Installation](../crates/launch/docs/installation.md)
  - [Instance Control](../crates/launch/docs/instance-control.md)
  - [Events](../crates/launch/docs/events.md)
  - [How to Use](../crates/launch/docs/how-to-use.md)
  - [Exports](../crates/launch/docs/exports.md)
- **[lighty-loaders](../crates/loaders/README.md)** - Mod loader implementations
- **[lighty-version](../crates/version/README.md)** - Version builders
  - [How to Use](../crates/version/docs/how-to-use.md)
  - [Overview](../crates/version/docs/overview.md)
  - [Exports](../crates/version/docs/exports.md)
  - [VersionBuilder](../crates/version/docs/version-builder.md)
  - [LightyVersionBuilder](../crates/version/docs/lighty-version-builder.md)

## Examples Directory

All examples are located in `examples/`:

- **vanilla.rs** - Basic Vanilla launcher
- **fabric.rs** - Fabric mod loader
- **quilt.rs** - Quilt mod loader
- **neoforge.rs** - NeoForge mod loader
- **forge.rs** - Forge mod loader
- **forge_legacy.rs** - Legacy Forge (1.7.10-1.12.2)
- **optifine.rs** - OptiFine
- **lighty_updater.rs** - Custom modpack server
- **with_events.rs** - Complete event system demo

See [Examples Guide](./examples.md) for detailed walkthroughs.

## Additional Resources

### External Links

- **[Crates.io](https://crates.io/crates/lighty-launcher)** - Published packages
- **[Docs.rs](https://docs.rs/lighty-launcher)** - API documentation
- **[GitHub](https://github.com/Lighty-Launcher/LightyLauncher)** - Source code
- **[LightyUpdater](https://github.com/Lighty-Launcher/LightyUpdater)** - Custom server

### Community

- **[Issues](https://github.com/Lighty-Launcher/LightyLauncher/issues)** - Bug reports and feature requests
- **[Discussions](https://github.com/Lighty-Launcher/LightyLauncher/discussions)** - Questions and ideas

## Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) for contribution guidelines.

## License

MIT License - see [LICENSE](../LICENSE) file for details.
