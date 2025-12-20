# Event Reference

## Event Categories

### AuthEvent
- `AuthenticationStarted` - Login begins
- `AuthenticationSuccess` - Login successful
- `AuthenticationFailed` - Login failed

### JavaEvent
- `JavaDownloadStarted` - JRE download begins
- `JavaDownloadProgress` - Download progress
- `JavaDownloadCompleted` - Download complete
- `JavaExtractionStarted` - Extraction begins
- `JavaExtractionCompleted` - Extraction complete

### LaunchEvent
- `InstallStarted` - Installation begins
- `InstallProgress` - Progress update
- `InstallCompleted` - Installation complete
- `Launched` - Game launched
- `ProcessExited` - Game exited

### LoaderEvent
- `FetchingData` - Fetching loader manifest
- `DataFetched` - Manifest retrieved
- `ManifestCached` - Using cached data

### CoreEvent
- `DownloadStarted` - File download begins
- `DownloadProgress` - Download progress
- `ExtractionStarted` - Archive extraction begins

### InstanceEvent
- `InstanceLaunched` - Instance started (PID, version, username)
- `ConsoleOutput` - Real-time stdout/stderr
- `InstanceExited` - Instance exited (exit code)
- `InstanceDeleted` - Instance deleted

## See Also

- [Architecture](./architecture.md)
- [Examples](./examples.md)
