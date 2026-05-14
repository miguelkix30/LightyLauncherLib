use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::RwLock;
use std::time::SystemTime;

use super::errors::{InstanceError, InstanceResult};

/// Internal representation of a running game instance.
///
/// `version`/`username`/`game_dir`/`started_at` are stored for future
/// audit/debug APIs (e.g. `pub fn started_at()` exposed via `InstanceControl`).
pub(crate) struct GameInstance {
    /// Process ID
    pub pid: u32,
    /// Instance name
    pub instance_name: String,
    /// Version string (e.g., "1.20.1-fabric-0.15.0")
    #[allow(dead_code)]
    pub version: String,
    /// Username used to launch
    #[allow(dead_code)]
    pub username: String,
    /// Game directory path
    #[allow(dead_code)]
    pub game_dir: PathBuf,
    /// Launch timestamp
    #[allow(dead_code)]
    pub started_at: SystemTime,
}

/// Internal manager for tracking running game instances
pub(crate) struct InstanceManager {
    instances: RwLock<HashMap<u32, GameInstance>>,
}

/// Global instance manager
pub(crate) static INSTANCE_MANAGER: Lazy<InstanceManager> = Lazy::new(InstanceManager::new);

impl InstanceManager {
    /// Create a new instance manager
    pub fn new() -> Self {
        Self {
            instances: RwLock::new(HashMap::new()),
        }
    }

    /// Get the first PID for a given instance name
    pub fn get_pid(&self, instance_name: &str) -> Option<u32> {
        let instances = self.instances.read().unwrap();
        instances
            .values()
            .find(|inst| inst.instance_name == instance_name)
            .map(|inst| inst.pid)
    }

    /// Get all PIDs for a given instance name
    pub fn get_pids(&self, instance_name: &str) -> Vec<u32> {
        let instances = self.instances.read().unwrap();
        instances
            .values()
            .filter(|inst| inst.instance_name == instance_name)
            .map(|inst| inst.pid)
            .collect()
    }

    /// Register a new running instance
    pub async fn register_instance(&self, instance: GameInstance) {
        let mut instances = self.instances.write().unwrap();
        instances.insert(instance.pid, instance);
    }

    /// Unregister an instance by PID
    pub async fn unregister_instance(&self, pid: u32) {
        let mut instances = self.instances.write().unwrap();
        instances.remove(&pid);
    }

    /// Close an instance by PID
    ///
    /// Kills the process using the system's kill mechanism.
    /// The instance will be unregistered automatically by the console handler.
    pub async fn close_instance(&self, pid: u32) -> InstanceResult<()> {
        let mut instances = self.instances.write().unwrap();
        instances
            .remove(&pid)
            .ok_or(InstanceError::NotFound { pid })?;

        drop(instances); // Release the lock

        // Platform-specific kill:
        // - Windows: shell out to `taskkill /F`, which terminates the process
        //   tree without raising a console close event.
        // - Unix: send SIGTERM so the JVM runs its shutdown hooks. SIGKILL is
        //   deliberately avoided so the world/save state has a chance to flush.
        #[cfg(target_os = "windows")]
        {
            use std::process::Command;
            let output = Command::new("taskkill")
                .args(&["/PID", &pid.to_string(), "/F"])
                .output()?;

            if !output.status.success() {
                lighty_core::trace_warn!(pid = pid, "Failed to kill process");
            } else {
                lighty_core::trace_info!(pid = pid, "Instance killed");
            }
        }

        #[cfg(not(target_os = "windows"))]
        {
            use nix::sys::signal::{kill, Signal};
            use nix::unistd::Pid;

            match kill(Pid::from_raw(pid as i32), Signal::SIGTERM) {
                Ok(_) => {
                    lighty_core::trace_info!(pid = pid, "Instance killed");
                }
                Err(e) => {
                    lighty_core::trace_warn!(pid = pid, error = %e, "Failed to kill process");
                    return Err(InstanceError::Io(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Failed to kill process: {}", e),
                    )));
                }
            }
        }

        Ok(())
    }
}
