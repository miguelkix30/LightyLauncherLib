# Java Distributions

## Overview

`lighty-java` supports four major Java distributions, each with different characteristics and use cases.

## Temurin (Eclipse Adoptium)

### Characteristics

- **Provider**: Eclipse Adoptium (formerly AdoptOpenJDK)
- **Type**: OpenJDK builds
- **License**: GPLv2 + Classpath Exception
- **Support**: All Java versions (8, 11, 17, 21+)
- **Platforms**: Windows (x64, ARM64), Linux (x64, ARM64), macOS (x64, ARM64)

### API

- **Base URL**: `https://api.adoptium.net/v3/assets/`
- **Query Parameters**:
  - `feature_version`: Java major version (8, 11, 17, 21)
  - `image_type`: JRE or JDK
  - `os`: Operating system
  - `architecture`: CPU architecture
  - `heap_size`: Normal heap
  - `vendor`: eclipse

### Download Sizes

| Version | Windows x64 | Linux x64 | macOS ARM64 |
|---------|-------------|-----------|-------------|
| Java 8 | ~45 MB | ~43 MB | N/A |
| Java 11 | ~100 MB | ~98 MB | ~95 MB |
| Java 17 | ~110 MB | ~108 MB | ~105 MB |
| Java 21 | ~120 MB | ~118 MB | ~115 MB |

### Usage

```rust
use lighty_java::{JavaDistribution, jre_downloader};

let distribution = JavaDistribution::Temurin;
let java_path = jre_downloader::jre_download(
    runtime_dir,
    &distribution,
    &21,
    |_, _| {}
).await?;
```

### When to Use

- General-purpose Minecraft launchers
- Maximum compatibility across all Minecraft versions
- Need for long-term support (LTS) versions
- Widest platform support

## GraalVM

### Characteristics

- **Provider**: Oracle
- **Type**: High-performance JDK with advanced JIT compiler
- **License**: GPLv2 + Classpath Exception (Community Edition)
- **Support**: Java 17, 21+ only
- **Platforms**: Windows (x64), Linux (x64, ARM64), macOS (x64, ARM64)

### Download Source

- **Source**: GitHub Releases
- **Repository**: `graalvm/graalvm-ce-builds`
- **Format**: Direct download URLs from release assets
- **Naming**: `graalvm-jdk-{version}_{os}_{arch}.{ext}`

### Download Sizes

| Version | Windows x64 | Linux x64 | macOS ARM64 |
|---------|-------------|-----------|-------------|
| Java 17 | ~180 MB | ~175 MB | ~170 MB |
| Java 21 | ~190 MB | ~185 MB | ~180 MB |

### Performance

- **JIT Compiler**: Advanced Graal compiler with better optimization
- **Startup Time**: Faster than standard OpenJDK
- **Peak Performance**: 10-20% better than standard JDK
- **Memory Usage**: Lower with G1GC

### Usage

```rust
use lighty_java::{JavaDistribution, jre_downloader};

let distribution = JavaDistribution::GraalVM;
let java_path = jre_downloader::jre_download(
    runtime_dir,
    &distribution,
    &21,
    |_, _| {}
).await?;
```

### When to Use

- Modern Minecraft versions (1.17+)
- Performance-critical applications
- Users with faster internet (larger downloads)
- Development and testing environments

### Limitations

- No Java 8 or 11 support
- Larger download size
- Windows ARM64 not supported

## Zulu (Azul Systems)

### Characteristics

- **Provider**: Azul Systems
- **Type**: TCK-certified OpenJDK builds
- **License**: GPLv2 + Classpath Exception
- **Support**: All Java versions (8, 11, 17, 21+)
- **Platforms**: Windows (x64, ARM64), Linux (x64, ARM64), macOS (x64, ARM64)

### API

- **Base URL**: `https://api.azul.com/metadata/v1/zulu/packages/`
- **Query Parameters**:
  - `java_version`: Major version
  - `os`: Operating system
  - `arch`: Architecture
  - `bundle_type`: JRE or JDK
  - `javafx`: false
  - `release_status`: GA (General Availability)

### Download Sizes

| Version | Windows x64 | Linux x64 | macOS ARM64 |
|---------|-------------|-----------|-------------|
| Java 8 | ~50 MB | ~48 MB | N/A |
| Java 11 | ~105 MB | ~103 MB | ~100 MB |
| Java 17 | ~115 MB | ~113 MB | ~110 MB |
| Java 21 | ~125 MB | ~123 MB | ~120 MB |

### Enterprise Features

- **TCK Certification**: Fully certified OpenJDK builds
- **Support Options**: Commercial support available
- **ARM Support**: Excellent ARM64 optimization
- **Security Updates**: Regular backports of security fixes

### Usage

```rust
use lighty_java::{JavaDistribution, jre_downloader};

let distribution = JavaDistribution::Zulu;
let java_path = jre_downloader::jre_download(
    runtime_dir,
    &distribution,
    &17,
    |_, _| {}
).await?;
```

### When to Use

- Enterprise environments requiring certification
- ARM-based systems (Raspberry Pi, Apple Silicon)
- Need for commercial support
- Regulatory compliance requirements

## Liberica (BellSoft)

### Characteristics

- **Provider**: BellSoft
- **Type**: Lightweight OpenJDK distribution
- **License**: GPLv2 + Classpath Exception
- **Support**: All Java versions (8, 11, 17, 21+)
- **Platforms**: Windows (x64, ARM64), Linux (x64, ARM64), macOS (x64, ARM64)

### API

- **Base URL**: `https://api.foojay.io/disco/v3.0/packages`
- **Query Parameters**:
  - `version`: Java major version
  - `distribution`: liberica
  - `operating_system`: OS name
  - `architecture`: CPU architecture
  - `archive_type`: Archive format
  - `package_type`: JRE

### Download Sizes

| Version | Windows x64 | Linux x64 | macOS ARM64 |
|---------|-------------|-----------|-------------|
| Java 8 | ~40 MB | ~38 MB | N/A |
| Java 11 | ~95 MB | ~93 MB | ~90 MB |
| Java 17 | ~105 MB | ~103 MB | ~100 MB |
| Java 21 | ~115 MB | ~113 MB | ~110 MB |

### Optimization Focus

- **Size**: Smallest download sizes
- **Memory**: Lower memory footprint
- **Embedded**: Optimized for embedded systems
- **JavaFX**: Optional JavaFX bundles available

### Usage

```rust
use lighty_java::{JavaDistribution, jre_downloader};

let distribution = JavaDistribution::Liberica;
let java_path = jre_downloader::jre_download(
    runtime_dir,
    &distribution,
    &11,
    |_, _| {}
).await?;
```

### When to Use

- Resource-constrained systems
- Older hardware with limited disk space
- Slow internet connections
- Embedded or IoT devices

## Distribution Comparison

| Feature | Temurin | GraalVM | Zulu | Liberica |
|---------|---------|---------|------|----------|
| **Java 8 Support** | ✅ | ❌ | ✅ | ✅ |
| **Java 11 Support** | ✅ | ❌ | ✅ | ✅ |
| **Java 17 Support** | ✅ | ✅ | ✅ | ✅ |
| **Java 21 Support** | ✅ | ✅ | ✅ | ✅ |
| **Windows ARM64** | ✅ | ❌ | ✅ | ✅ |
| **Performance** | Standard | High | Standard | Standard |
| **Download Size** | Medium | Large | Medium | Small |
| **TCK Certified** | ✅ | ✅ | ✅ | ✅ |
| **Commercial Support** | ❌ | ✅ | ✅ | ✅ |

## Version Detection

Automatically select appropriate Java version for Minecraft:

```rust
use lighty_java::runtime::get_jre_version;

// Minecraft 1.20.4 → Java 17
let version = get_jre_version("1.20.4");
assert_eq!(version, 17);

// Minecraft 1.16.5 → Java 8
let version = get_jre_version("1.16.5");
assert_eq!(version, 8);

// Minecraft 1.18.2 → Java 17
let version = get_jre_version("1.18.2");
assert_eq!(version, 17);
```

### Version Requirements

| Minecraft Version | Required Java | Recommended Distribution |
|------------------|---------------|--------------------------|
| < 1.12 | Java 8 | Temurin (smallest) |
| 1.12 - 1.16.5 | Java 8 | Temurin or Liberica |
| 1.17 - 1.17.1 | Java 16 | Temurin or GraalVM |
| 1.18+ | Java 17 | GraalVM (performance) or Temurin (compatibility) |

## Platform Support Matrix

### Windows

| Distribution | x64 | ARM64 | Notes |
|--------------|-----|-------|-------|
| Temurin | ✅ | ✅ | Full support |
| GraalVM | ✅ | ❌ | x64 only |
| Zulu | ✅ | ✅ | Excellent ARM support |
| Liberica | ✅ | ✅ | Full support |

### Linux

| Distribution | x64 | ARM64 | Notes |
|--------------|-----|-------|-------|
| Temurin | ✅ | ✅ | Full support |
| GraalVM | ✅ | ✅ | Full support |
| Zulu | ✅ | ✅ | Best ARM optimization |
| Liberica | ✅ | ✅ | Full support |

### macOS

| Distribution | Intel (x64) | Apple Silicon (ARM64) | Notes |
|--------------|-------------|----------------------|-------|
| Temurin | ✅ | ✅ | Full support |
| GraalVM | ✅ | ✅ | Native ARM builds |
| Zulu | ✅ | ✅ | Optimized for M1/M2 |
| Liberica | ✅ | ✅ | Full support |

## See Also

- [Overview](./overview.md) - Architecture and design
- [Installation](./installation.md) - Download and installation process
- [Examples](./examples.md) - Usage examples
