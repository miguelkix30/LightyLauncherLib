# Sequence Diagrams

## Complete Launch Sequence

```mermaid
sequenceDiagram
    participant User
    participant AppState
    participant VersionBuilder
    participant Auth
    participant LaunchBuilder
    participant Installer
    participant JavaManager
    participant Process

    User->>AppState: new(QUALIFIER, ORGANIZATION, APPLICATION)
    AppState-->>User: project_dirs

    User->>VersionBuilder: new(name, loader, loader_version, mc_version, project_dirs)
    VersionBuilder-->>User: instance

    User->>Auth: OfflineAuth::new(username)
    User->>Auth: authenticate()
    Auth-->>User: UserProfile

    User->>LaunchBuilder: instance.launch(profile, java_distribution)

    LaunchBuilder->>LaunchBuilder: Prepare Metadata
    LaunchBuilder->>JavaManager: Ensure Java Installed

    alt Java Not Found
        JavaManager->>JavaManager: Download Java Runtime
        JavaManager->>JavaManager: Extract Archive
        JavaManager-->>LaunchBuilder: java_path
    else Java Found
        JavaManager-->>LaunchBuilder: java_path
    end

    LaunchBuilder->>Installer: install(version_data)

    par Parallel Installation
        Installer->>Installer: Download Libraries
        Installer->>Installer: Download Natives
        Installer->>Installer: Download Client JAR
        Installer->>Installer: Download Assets
        Installer->>Installer: Download Mods (if applicable)
    end

    Installer->>Installer: Extract Natives
    Installer-->>LaunchBuilder: Installation Complete

    LaunchBuilder->>LaunchBuilder: Build Arguments
    LaunchBuilder->>Process: Spawn Java Process
    Process-->>LaunchBuilder: PID

    LaunchBuilder->>Process: Register Instance
    LaunchBuilder->>Process: Stream Console (stdout/stderr)

    Process-->>User: Instance Launched

    Note over Process: Game Running...

    Process->>Process: Emit ConsoleOutput Events

    alt Normal Exit
        Process->>Process: Game Exits
        Process->>Process: Emit InstanceExited Event
        Process->>Process: Unregister Instance
    else Manual Close
        User->>Process: close_instance(pid)
        Process->>Process: Kill Signal (SIGTERM/TASKKILL)
        Process->>Process: Emit InstanceExited Event
        Process->>Process: Unregister Instance
    end
```

## Authentication Sequence

### Offline Authentication

```mermaid
sequenceDiagram
    participant User
    participant OfflineAuth
    participant UUID

    User->>OfflineAuth: new(username)
    User->>OfflineAuth: authenticate()
    OfflineAuth->>UUID: generate_offline_uuid(username)
    UUID-->>OfflineAuth: deterministic UUID (v5)
    OfflineAuth-->>User: UserProfile {
        username,
        uuid,
        access_token: None,
        role: User
    }
```

### Microsoft Authentication

```mermaid
sequenceDiagram
    participant User
    participant MicrosoftAuth
    participant DeviceFlow
    participant Xbox
    participant Minecraft

    User->>MicrosoftAuth: new(client_id)
    User->>MicrosoftAuth: authenticate()

    MicrosoftAuth->>DeviceFlow: Request Device Code
    DeviceFlow-->>MicrosoftAuth: device_code, user_code, verification_url
    MicrosoftAuth->>User: Display: "Visit {url}, Enter {code}"

    loop Poll for completion
        MicrosoftAuth->>DeviceFlow: Poll for token
        alt User Authorized
            DeviceFlow-->>MicrosoftAuth: access_token
        else Still Pending
            DeviceFlow-->>MicrosoftAuth: authorization_pending
        end
    end

    MicrosoftAuth->>Xbox: Authenticate with Xbox Live
    Xbox-->>MicrosoftAuth: xbox_token, user_hash

    MicrosoftAuth->>Xbox: Get XSTS Token
    Xbox-->>MicrosoftAuth: xsts_token, xuid

    MicrosoftAuth->>Minecraft: Authenticate with Minecraft
    Minecraft-->>MicrosoftAuth: minecraft_access_token

    MicrosoftAuth->>Minecraft: Get Profile
    Minecraft-->>MicrosoftAuth: username, uuid

    MicrosoftAuth-->>User: UserProfile {
        username,
        uuid,
        access_token: Some(minecraft_access_token),
        role: User
    }
```

## Installation Sequence

```mermaid
sequenceDiagram
    participant Installer
    participant Libraries
    participant Natives
    participant Client
    participant Assets
    participant Mods
    participant EventBus

    Installer->>Installer: Phase 1: Verification (SHA1 Check)

    par Collect Tasks
        Installer->>Libraries: Check libraries
        Libraries-->>Installer: library_tasks[]

        Installer->>Natives: Check natives
        Natives-->>Installer: native_tasks[]

        Installer->>Client: Check client JAR
        Client-->>Installer: client_task?

        Installer->>Assets: Check assets
        Assets-->>Installer: asset_tasks[]

        Installer->>Mods: Check mods
        Mods-->>Installer: mod_tasks[]
    end

    alt All Files Valid (total_downloads == 0)
        Installer->>EventBus: Emit IsInstalled
        Installer->>Natives: Extract natives only
        Natives-->>Installer: Done
    else Files Need Download
        Installer->>EventBus: Emit InstallStarted

        par Phase 2: Parallel Download
            Installer->>Libraries: Download library_tasks
            Libraries->>EventBus: Emit DownloadingLibraries

            Installer->>Natives: Download & extract native_tasks
            Natives->>EventBus: Emit DownloadingNatives

            Installer->>Client: Download client_task
            Client->>EventBus: Emit DownloadingClient

            Installer->>Assets: Download asset_tasks
            Assets->>EventBus: Emit DownloadingAssets

            Installer->>Mods: Download mod_tasks
            Mods->>EventBus: Emit DownloadingMods
        end

        Installer->>EventBus: Emit InstallCompleted
    end
```

## Java Management Sequence

```mermaid
sequenceDiagram
    participant LaunchBuilder
    participant JavaManager
    participant Downloader
    participant Extractor
    participant EventBus

    LaunchBuilder->>JavaManager: ensure_java_installed(java_dirs, distribution, version)

    JavaManager->>JavaManager: find_java_binary(java_dirs, distribution, version)

    alt Java Found
        JavaManager->>EventBus: Emit JavaAlreadyInstalled
        JavaManager-->>LaunchBuilder: java_path
    else Java Not Found
        JavaManager->>EventBus: Emit JavaNotFound

        JavaManager->>Downloader: Download JRE archive
        Downloader->>EventBus: Emit JavaDownloadStarted

        loop Download Progress
            Downloader->>EventBus: Emit JavaDownloadProgress
        end

        Downloader->>EventBus: Emit JavaDownloadCompleted
        Downloader-->>JavaManager: archive_path

        JavaManager->>Extractor: Extract archive
        Extractor->>EventBus: Emit JavaExtractionStarted

        loop Extraction Progress
            Extractor->>EventBus: Emit ExtractionProgress
        end

        Extractor->>EventBus: Emit JavaExtractionCompleted
        Extractor-->>JavaManager: extracted_path

        JavaManager->>JavaManager: find_java_binary(extracted_path)
        JavaManager-->>LaunchBuilder: java_path
    end
```

## Instance Control Sequence

```mermaid
sequenceDiagram
    participant User
    participant InstanceControl
    participant InstanceManager
    participant Process
    participant EventBus

    Note over User,EventBus: Get Running Instance

    User->>InstanceControl: get_pid()
    InstanceControl->>InstanceManager: get_pid(instance_name)
    InstanceManager-->>InstanceControl: pid?
    InstanceControl-->>User: Option<u32>

    Note over User,EventBus: Close Instance

    User->>InstanceControl: close_instance(pid)
    InstanceControl->>InstanceManager: close_instance(pid)

    alt Windows
        InstanceManager->>Process: taskkill /PID {pid} /F
    else Unix (Linux/macOS)
        InstanceManager->>Process: kill -SIGTERM {pid}
    end

    Process->>Process: Terminate
    Process->>EventBus: Emit InstanceExited
    Process->>InstanceManager: Unregister instance
    InstanceManager-->>User: Result<()>

    Note over User,EventBus: Delete Instance

    User->>InstanceControl: delete_instance()
    InstanceControl->>InstanceManager: has_running_instances()

    alt Instance Running
        InstanceManager-->>User: Error: InstanceRunning
    else Not Running
        InstanceControl->>InstanceControl: Delete game directory
        InstanceControl->>EventBus: Emit InstanceDeleted
        InstanceControl-->>User: Result<()>
    end
```

## Console Streaming Sequence

```mermaid
sequenceDiagram
    participant Process
    participant StdoutHandler
    participant StderrHandler
    participant EventBus
    participant User

    Process->>Process: Spawn Java Process
    Process->>StdoutHandler: Spawn stdout task
    Process->>StderrHandler: Spawn stderr task

    par Console Streaming
        loop Read stdout
            StdoutHandler->>StdoutHandler: Read line
            StdoutHandler->>EventBus: Emit ConsoleOutput(Stdout, line)
            EventBus->>User: Console line
        end

        loop Read stderr
            StderrHandler->>StderrHandler: Read line
            StderrHandler->>EventBus: Emit ConsoleOutput(Stderr, line)
            EventBus->>User: Console line
        end
    end

    Process->>Process: Wait for exit
    Process->>EventBus: Emit InstanceExited(exit_code)
    Process->>Process: Unregister instance
```

## Event Flow Diagram

```mermaid
flowchart TB
    Start([User Initiates Launch]) --> InitState[Initialize AppState]
    InitState --> CreateVersion[Create VersionBuilder]
    CreateVersion --> Auth[Authenticate User]

    Auth --> |OfflineAuth| OfflineEvent[Emit AuthenticationStarted/Success]
    Auth --> |MicrosoftAuth| MSEvents[Emit Device Code Events]

    OfflineEvent --> StartLaunch[Call launch()]
    MSEvents --> StartLaunch

    StartLaunch --> FetchMeta[Fetch Loader Metadata]
    FetchMeta --> |Emit LoaderEvent| CheckJava[Check Java Installation]

    CheckJava --> |Not Found| DownloadJava[Download Java]
    CheckJava --> |Found| InstallDeps[Install Dependencies]

    DownloadJava --> |Emit JavaEvents| InstallDeps

    InstallDeps --> Verify{All Files Valid?}

    Verify --> |Yes| EmitInstalled[Emit IsInstalled]
    Verify --> |No| EmitStart[Emit InstallStarted]

    EmitInstalled --> ExtractNatives[Extract Natives Only]
    EmitStart --> ParallelDownload[Parallel Download]

    ParallelDownload --> |Libraries| LibEvents[Emit DownloadingLibraries]
    ParallelDownload --> |Natives| NatEvents[Emit DownloadingNatives]
    ParallelDownload --> |Client| ClientEvents[Emit DownloadingClient]
    ParallelDownload --> |Assets| AssetEvents[Emit DownloadingAssets]
    ParallelDownload --> |Mods| ModEvents[Emit DownloadingMods]

    LibEvents --> EmitComplete[Emit InstallCompleted]
    NatEvents --> EmitComplete
    ClientEvents --> EmitComplete
    AssetEvents --> EmitComplete
    ModEvents --> EmitComplete
    ExtractNatives --> EmitComplete

    EmitComplete --> BuildArgs[Build Arguments]
    BuildArgs --> SpawnProcess[Spawn Java Process]
    SpawnProcess --> Register[Register Instance]
    Register --> EmitLaunched[Emit InstanceLaunched]

    EmitLaunched --> StreamConsole[Stream Console Output]
    StreamConsole --> |Each Line| EmitConsole[Emit ConsoleOutput]

    EmitConsole --> WaitExit{Process Running?}
    WaitExit --> |Yes| StreamConsole
    WaitExit --> |No| EmitExited[Emit InstanceExited]

    EmitExited --> Cleanup[Unregister Instance]
    Cleanup --> End([Launch Complete])
```

## Loader-Specific Sequences

### Fabric Loader

```mermaid
sequenceDiagram
    participant User
    participant FabricLoader
    participant VanillaAPI
    participant FabricAPI
    participant Merger

    User->>FabricLoader: get_metadata()

    FabricLoader->>VanillaAPI: Fetch Vanilla manifest
    VanillaAPI-->>FabricLoader: vanilla_metadata

    FabricLoader->>FabricAPI: Fetch Fabric loader data
    FabricAPI-->>FabricLoader: fabric_loader_data

    FabricLoader->>Merger: Merge vanilla + fabric
    Merger->>Merger: Add Fabric libraries
    Merger->>Merger: Update main class
    Merger->>Merger: Merge arguments
    Merger-->>FabricLoader: merged_metadata

    FabricLoader-->>User: VersionMetaData
```

### LightyUpdater

```mermaid
sequenceDiagram
    participant User
    participant LightyBuilder
    participant ServerAPI
    participant Vanilla

    User->>LightyBuilder: new(name, server_url, project_dirs)
    User->>LightyBuilder: get_metadata()

    LightyBuilder->>ServerAPI: GET {server_url}/version
    ServerAPI-->>LightyBuilder: {
        minecraft_version,
        loader,
        loader_version,
        mods: [...]
    }

    LightyBuilder->>Vanilla: Fetch vanilla metadata
    Vanilla-->>LightyBuilder: vanilla_metadata

    LightyBuilder->>LightyBuilder: Add server mods to metadata
    LightyBuilder-->>User: VersionMetaData with custom mods
```

## Related Documentation

- [Launch Process](../crates/launch/docs/launch.md) - Detailed launch flow
- [Installation](../crates/launch/docs/installation.md) - Installation details
- [Instance Control](../crates/launch/docs/instance-control.md) - Process management
- [Events](../crates/launch/docs/events.md) - Event types reference
