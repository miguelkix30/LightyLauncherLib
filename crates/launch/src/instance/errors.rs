use std::fmt;

/// Result type for instance operations
pub type InstanceResult<T> = Result<T, InstanceError>;

/// Errors that can occur during instance management
#[derive(Debug)]
pub enum InstanceError {
    /// Instance not found by PID
    NotFound { pid: u32 },

    /// Instance is still running and cannot be deleted
    StillRunning {
        instance_name: String,
        pids: Vec<u32>,
    },

    /// I/O error during instance operations
    Io(std::io::Error),
}

impl fmt::Display for InstanceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstanceError::NotFound { pid } => {
                write!(f, "Instance with PID {} not found", pid)
            }
            InstanceError::StillRunning {
                instance_name,
                pids,
            } => {
                write!(
                    f,
                    "Cannot delete instance '{}': still running with PIDs {:?}",
                    instance_name, pids
                )
            }
            InstanceError::Io(err) => write!(f, "I/O error: {}", err),
        }
    }
}

impl std::error::Error for InstanceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            InstanceError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for InstanceError {
    fn from(err: std::io::Error) -> Self {
        InstanceError::Io(err)
    }
}
