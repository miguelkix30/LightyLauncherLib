use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Child;
use std::path::PathBuf;

#[cfg(feature = "events")]
use lighty_event::EventBus;

/// Handle console streams (stdout/stderr) from a running game instance
///
/// This function spawns asynchronous tasks to:
/// - Read and emit stdout lines (Minecraft includes its own timestamps in the log text)
/// - Read and emit stderr lines
/// - Wait for the process to exit and emit exit event
/// - Unregister the instance when done
///
/// Note: Frontend should not display the event timestamp for stdout as Minecraft
/// already includes timestamps in its log format
pub(crate) async fn handle_console_streams(
    pid: u32,
    instance_name: String,
    game_dir: PathBuf,
    mut child: Child,
    #[cfg(feature = "events")] event_bus: Option<EventBus>,
) {
    let stdout = child.stdout.take();
    let stderr = child.stderr.take();

    // Handler stdout
    if let Some(stdout) = stdout {
        let instance_name = instance_name.clone();
        #[cfg(feature = "events")]
        let event_bus_clone = event_bus.clone();

        tokio::spawn(async move {
            let reader = BufReader::new(stdout);
            let mut lines = reader.lines();

            while let Ok(Some(line)) = lines.next_line().await {
                #[cfg(feature = "events")]
                {
                    use lighty_event::{ConsoleOutputEvent, ConsoleStream, Event};
                    use std::time::SystemTime;

                    if let Some(ref bus) = event_bus_clone {
                        bus.emit(Event::ConsoleOutput(ConsoleOutputEvent {
                            pid,
                            instance_name: instance_name.clone(),
                            stream: ConsoleStream::Stdout,
                            line,
                            timestamp: SystemTime::now(),
                        }));
                    }
                }
            }
        });
    }

    // Handler stderr
    if let Some(stderr) = stderr {
        let instance_name = instance_name.clone();
        #[cfg(feature = "events")]
        let event_bus_clone = event_bus.clone();

        tokio::spawn(async move {
            let reader = BufReader::new(stderr);
            let mut lines = reader.lines();

            while let Ok(Some(line)) = lines.next_line().await {
                #[cfg(feature = "events")]
                {
                    use lighty_event::{ConsoleOutputEvent, ConsoleStream, Event};
                    use std::time::SystemTime;

                    if let Some(ref bus) = event_bus_clone {
                        bus.emit(Event::ConsoleOutput(ConsoleOutputEvent {
                            pid,
                            instance_name: instance_name.clone(),
                            stream: ConsoleStream::Stderr,
                            line,
                            timestamp: SystemTime::now(),
                        }));
                    }
                }
            }
        });
    }

    // Wait for process to exit
    match child.wait().await {
        Ok(status) => {
            #[cfg(feature = "events")]
            {
                use lighty_event::{Event, InstanceExitedEvent};
                use std::time::SystemTime;

                if let Some(ref bus) = event_bus {
                    let exit_code = status.code();
                    let mut log_excerpt = None;
                    let mut error_lines = None;

                    // If exit code is non-zero, try to read logs
                    if exit_code != Some(0) {
                        if let Ok(logs) = super::logs::read_latest_log(&game_dir, 50) {
                            if !logs.is_empty() {
                                let log_text = logs.join("\n");
                                let errors = super::logs::extract_errors_from_log(&log_text);
                                
                                log_excerpt = Some(logs);
                                if !errors.is_empty() {
                                    error_lines = Some(errors);
                                }
                            }
                        }
                    }

                    bus.emit(Event::InstanceExited(InstanceExitedEvent {
                        pid,
                        instance_name: instance_name.clone(),
                        exit_code,
                        timestamp: SystemTime::now(),
                        log_excerpt,
                        error_lines,
                    }));
                }
            }

            lighty_core::trace_info!(
                pid = pid,
                instance = %instance_name,
                exit_code = ?status.code(),
                "Instance exited"
            );
        }
        Err(e) => {
            lighty_core::trace_error!(
                pid = pid,
                instance = %instance_name,
                error = %e,
                "Error waiting for instance"
            );
        }
    }

    // Cleanup
    use super::INSTANCE_MANAGER;
    let _ = INSTANCE_MANAGER.unregister_instance(pid).await;
}
