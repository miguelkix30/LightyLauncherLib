# Hash Utilities

## Overview

`lighty-core` provides SHA1 hash verification utilities for both async and sync contexts. These are essential for verifying downloaded files and ensuring data integrity.

**Export**:
- Module: `lighty_core::hash`
- Re-export: `lighty_launcher::core::hash`

## Functions

### verify_file_sha1

Async SHA1 verification for small to medium files.

```rust
pub async fn verify_file_sha1(path: &Path, expected_sha1: &str) -> HashResult<bool>
```

**Use case**: Files that fit comfortably in memory (< 100MB)

**Example**:
```rust
use lighty_core::hash::verify_file_sha1;
use std::path::Path;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let is_valid = verify_file_sha1(
        Path::new("minecraft-1.21.1.jar"),
        "abc123def456..."
    ).await?;

    if is_valid {
        println!("✓ File integrity verified");
    } else {
        println!("✗ Hash mismatch!");
    }

    Ok(())
}
```

### verify_file_sha1_streaming

Async SHA1 verification with streaming for large files.

```rust
pub async fn verify_file_sha1_streaming(path: &Path, expected_sha1: &str) -> HashResult<bool>
```

**Use case**: Large files (> 100MB) to minimize memory usage

**Example**:
```rust
use lighty_core::hash::verify_file_sha1_streaming;
use std::path::Path;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Large Java runtime archive
    let is_valid = verify_file_sha1_streaming(
        Path::new("java-17.tar.gz"),
        "expected-sha1-hash"
    ).await?;

    println!("Large file valid: {}", is_valid);

    Ok(())
}
```

**Performance**: Uses 8KB buffer for streaming reads.

### calculate_file_sha1_sync

Synchronous SHA1 calculation.

```rust
pub fn calculate_file_sha1_sync(path: &Path) -> HashResult<String>
```

**Use case**: Non-async contexts, sync archive processing

**Example**:
```rust
use lighty_core::hash::calculate_file_sha1_sync;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let hash = calculate_file_sha1_sync(Path::new("file.jar"))?;
    println!("SHA1: {}", hash);

    Ok(())
}
```

### verify_file_sha1_sync

Synchronous SHA1 verification.

```rust
pub fn verify_file_sha1_sync(path: &Path, expected_sha1: &str) -> HashResult<bool>
```

**Use case**: Sync contexts

**Example**:
```rust
use lighty_core::hash::verify_file_sha1_sync;
use std::path::Path;

fn verify_mod(path: &Path, expected: &str) -> anyhow::Result<()> {
    let is_valid = verify_file_sha1_sync(path, expected)?;

    if !is_valid {
        return Err(anyhow::anyhow!("Mod file corrupted"));
    }

    Ok(())
}
```

### calculate_sha1_bytes

Calculate SHA1 hash of arbitrary bytes.

```rust
pub fn calculate_sha1_bytes(data: &[u8]) -> String
```

**Use case**: Hashing strings, usernames, in-memory data

**Example**:
```rust
use lighty_core::hash::calculate_sha1_bytes;

fn main() {
    // Hash username for offline mode UUID
    let username = "Player123";
    let hash = calculate_sha1_bytes(username.as_bytes());
    println!("Username hash: {}", hash);

    // Hash any data
    let data = vec![1, 2, 3, 4, 5];
    let hash = calculate_sha1_bytes(&data);
    println!("Data hash: {}", hash);
}
```

### calculate_sha1_bytes_raw

Calculate SHA1 hash returning raw bytes.

```rust
pub fn calculate_sha1_bytes_raw(data: &[u8]) -> [u8; 20]
```

**Use case**: When you need raw hash bytes instead of hex string

**Example**:
```rust
use lighty_core::hash::calculate_sha1_bytes_raw;

fn main() {
    let data = b"Hello, World!";
    let raw_hash = calculate_sha1_bytes_raw(data);

    println!("Raw hash bytes: {:?}", raw_hash);
    println!("Length: {}", raw_hash.len()); // Always 20 bytes
}
```

## Error Handling

```rust
use lighty_core::hash::{verify_file_sha1, HashError};
use std::path::Path;

#[tokio::main]
async fn main() {
    match verify_file_sha1(Path::new("file.jar"), "expected").await {
        Ok(true) => {
            println!("✓ Hash matches");
        }
        Ok(false) => {
            println!("✗ Hash mismatch");
        }
        Err(HashError::Io(e)) => {
            eprintln!("IO error reading file: {}", e);
        }
        Err(HashError::Mismatch { expected, actual }) => {
            eprintln!("Hash mismatch:");
            eprintln!("  Expected: {}", expected);
            eprintln!("  Got:      {}", actual);
        }
    }
}
```

## Usage in Download System

SHA1 verification is commonly used after downloading files:

```rust
use lighty_core::download::download_file_untracked;
use lighty_core::hash::verify_file_sha1;
use std::path::Path;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let url = "https://example.com/library.jar";
    let path = Path::new("libraries/library.jar");
    let expected_sha1 = "abc123def456...";

    // Download file
    download_file_untracked(url, path).await?;

    // Verify hash
    let is_valid = verify_file_sha1(path, expected_sha1).await?;

    if !is_valid {
        // Delete corrupted file
        tokio::fs::remove_file(path).await?;
        return Err(anyhow::anyhow!("Download corrupted"));
    }

    println!("✓ Downloaded and verified");

    Ok(())
}
```

## Performance Comparison

### Small Files (< 10MB)

**Use**: `verify_file_sha1` - Reads entire file into memory
- Faster for small files
- Simple implementation

### Medium Files (10MB - 100MB)

**Use**: Either function works well
- `verify_file_sha1` for speed
- `verify_file_sha1_streaming` to save memory

### Large Files (> 100MB)

**Use**: `verify_file_sha1_streaming` - Streams file in chunks
- Minimal memory usage
- Slightly slower but more memory-efficient

## Case Insensitivity

All hash comparison functions are **case-insensitive**:

```rust
use lighty_core::hash::verify_file_sha1;
use std::path::Path;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // These all work the same
    let hash1 = "ABC123DEF456";
    let hash2 = "abc123def456";
    let hash3 = "AbC123dEf456";

    // All return true if file hash matches (case-insensitive)
    verify_file_sha1(Path::new("file.jar"), hash1).await?;
    verify_file_sha1(Path::new("file.jar"), hash2).await?;
    verify_file_sha1(Path::new("file.jar"), hash3).await?;

    Ok(())
}
```

## Exports

**In lighty_core**:
```rust
use lighty_core::hash::{
    // Async
    verify_file_sha1,
    verify_file_sha1_streaming,

    // Sync
    calculate_file_sha1_sync,
    verify_file_sha1_sync,

    // Bytes
    calculate_sha1_bytes,
    calculate_sha1_bytes_raw,

    // Errors
    HashError,
    HashResult,
};
```

**In lighty_launcher**:
```rust
use lighty_launcher::core::hash::{
    verify_file_sha1,
    verify_file_sha1_streaming,
    // ... etc
};
```

## Related Documentation

- [How to Use](./how-to-use.md) - Practical hash verification examples
- [Download](./download.md) - Using hashes with downloads
- [Exports](./exports.md) - Complete export reference
- [Overview](./overview.md) - Architecture overview
