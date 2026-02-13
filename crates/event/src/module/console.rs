use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Event emitted when a game instance is launched
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceLaunchedEvent {
    /// Process ID of the launched instance
    pub pid: u32,
    /// Name of the instance
    pub instance_name: String,
    /// Version string (e.g., "1.20.1-fabric-0.15.0")
    pub version: String,
    /// Username used to launch the instance
    pub username: String,
    /// Timestamp when the instance was launched
    #[serde(with = "system_time_serializer")]
    pub timestamp: SystemTime,
}

/// Event emitted when a game instance window appears
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceWindowAppearedEvent {
    /// Process ID of the instance
    pub pid: u32,
    /// Name of the instance
    pub instance_name: String,
    /// Version string (e.g., "1.20.1-fabric-0.15.0")
    pub version: String,
    /// Timestamp when the window appeared
    #[serde(with = "system_time_serializer")]
    pub timestamp: SystemTime,
}

/// Event emitted when a game instance exits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceExitedEvent {
    /// Process ID of the exited instance
    pub pid: u32,
    /// Name of the instance
    pub instance_name: String,
    /// Exit code (None if terminated abnormally)
    pub exit_code: Option<i32>,
    /// Timestamp when the instance exited
    #[serde(with = "system_time_serializer")]
    pub timestamp: SystemTime,
    /// Last N lines from logs/latest.log (if available and exit code != 0)
    pub log_excerpt: Option<Vec<String>>,
    /// Extracted error lines from the log (if available)
    pub error_lines: Option<Vec<String>>,
}

/// Event emitted for each line of console output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsoleOutputEvent {
    /// Process ID of the instance
    pub pid: u32,
    /// Name of the instance
    pub instance_name: String,
    /// Stream type (stdout or stderr)
    pub stream: ConsoleStream,
    /// Console line content
    pub line: String,
    /// Timestamp when the line was emitted
    #[serde(with = "system_time_serializer")]
    pub timestamp: SystemTime,
}

/// Event emitted when an instance is deleted
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceDeletedEvent {
    /// Name of the deleted instance
    pub instance_name: String,
    /// Timestamp when the instance was deleted
    #[serde(with = "system_time_serializer")]
    pub timestamp: SystemTime,
}

/// Console stream type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ConsoleStream {
    /// Standard output stream
    Stdout,
    /// Standard error stream
    Stderr,
}

/// Module for serializing/deserializing SystemTime
mod system_time_serializer {
    use serde::{Deserialize, Deserializer, Serializer};
    use std::time::{SystemTime, UNIX_EPOCH};

    pub fn serialize<S>(time: &SystemTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let duration = time
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        serializer.serialize_u64(duration.as_secs())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<SystemTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secs = u64::deserialize(deserializer)?;
        Ok(UNIX_EPOCH + std::time::Duration::from_secs(secs))
    }
}
