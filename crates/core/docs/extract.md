# Archive Extraction

## Overview

Support for extracting ZIP, TAR, and TAR.GZ archives with proper error handling and permission preservation.

## Quick Example

```rust
use lighty_core::extract_archive;

#[tokio::main]
async fn main()  {
    extract_archive(
        "/tmp/archive.zip",
        "/tmp/extracted"
    ).await?;

    Ok(())
}
```

## Supported Formats

| Format | Extension | Compression |
|--------|-----------|-------------|
| ZIP    | `.zip`    | Deflate     |
| TAR    | `.tar`    | None        |
| TAR.GZ | `.tar.gz` | Gzip        |

## API Reference

### `extract_archive(archive_path, destination)`

Extracts an archive to the specified destination.

**Parameters:**
- `archive_path: impl AsRef<Path>` - Path to archive file
- `destination: impl AsRef<Path>` - Output directory

**Returns:** `Result<(), ExtractError>`

**Auto-Detection:** Format is detected from file extension

```rust
// ZIP
extract_archive("file.zip", "/out").await?;

// TAR
extract_archive("file.tar", "/out").await?;

// TAR.GZ
extract_archive("file.tar.gz", "/out").await?;
```

## Error Handling

```rust
pub enum ExtractError {
    /// Unsupported archive format
    UnsupportedFormat(String),

    /// IO error during extraction
    IOError(std::io::Error),

    /// Archive corrupted or invalid
    ArchiveError(String),
}
```

## Best Practices

### 1. Validate Archive Before Extraction
```rust
use std::path::Path;

async fn safe_extract(archive: &Path, dest: &Path)  {
    // Check archive exists
    if !archive.exists() {
        return Err("Archive not found".into());
    }

    // Create destination
    tokio::fs::create_dir_all(dest).await?;

    // Extract
    extract_archive(archive, dest).await?;
    Ok(())
}
```

### 2. Clean Destination Directory
```rust
// Remove old files before extraction
if dest.exists() {
    tokio::fs::remove_dir_all(&dest).await?;
}
extract_archive(archive, dest).await?;
```

## See Also

- [Download System](./download.md)
- [Examples](./examples.md)
