# System Detection

## Overview

Cross-platform detection for operating systems and architectures at runtime.

## Quick Example

```rust
use lighty_core::{get_os, get_architecture};

fn main() {
    let os = get_os();
    let arch = get_architecture();

    println!("OS: {:?}, Arch: {:?}", os, arch);
    // Output: OS: Windows, Arch: X86_64
}
```

## API Reference

### `get_os() -> OS`

Returns the current operating system.

**Returns:**
```rust
pub enum OS {
    Windows,
    MacOS,
    Linux,
    Unknown,
}
```

### `get_architecture() -> Architecture`

Returns the CPU architecture.

**Returns:**
```rust
pub enum Architecture {
    X86_64,  // 64-bit Intel/AMD
    X86,     // 32-bit Intel/AMD
    Aarch64, // 64-bit ARM (Apple Silicon, etc.)
    Arm,     // 32-bit ARM
    Unknown,
}
```

## Use Cases

### Platform-Specific Downloads
```rust
use lighty_core::{get_os, get_architecture, OS, Architecture};

fn get_download_url() -> &'static str {
    match (get_os(), get_architecture()) {
        (OS::Windows, Architecture::X86_64) => "https://example.com/win-x64.zip",
        (OS::MacOS, Architecture::Aarch64) => "https://example.com/mac-arm64.zip",
        (OS::Linux, Architecture::X86_64) => "https://example.com/linux-x64.tar.gz",
        _ => panic!("Unsupported platform"),
    }
}
```

### Java Distribution Selection
```rust
fn select_java_dist() -> &'static str {
    match get_architecture() {
        Architecture::Aarch64 => "temurin-aarch64",
        Architecture::X86_64 => "temurin-x64",
        _ => "temurin-default",
    }
}
```

## See Also

- [Overview](./overview.md)
- [Examples](./examples.md)
