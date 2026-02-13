use std::path::{Path, PathBuf};
use std::io::Result as IoResult;

/// Read the latest Minecraft log file from the game directory
///
/// Minecraft creates logs in the `logs/` subdirectory:
/// - `logs/latest.log` - Current session log
/// - `logs/debug.log` - Detailed debug log (if debug logging enabled)
///
/// This function attempts to read `latest.log` and returns the last N lines.
pub fn read_latest_log(game_dir: &Path, max_lines: usize) -> IoResult<Vec<String>> {
    let log_path = game_dir.join("logs").join("latest.log");
    
    if !log_path.exists() {
        return Ok(vec![format!("Log file not found: {}", log_path.display())]);
    }

    let content = std::fs::read_to_string(&log_path)?;
    let lines: Vec<String> = content
        .lines()
        .rev()
        .take(max_lines)
        .map(String::from)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();

    Ok(lines)
}

/// Read all available Minecraft logs from the game directory
pub fn read_all_logs(game_dir: &Path) -> IoResult<String> {
    let mut output = String::new();
    
    // Try latest.log
    let latest_log = game_dir.join("logs").join("latest.log");
    if latest_log.exists() {
        output.push_str(&format!("=== latest.log ===\n"));
        output.push_str(&std::fs::read_to_string(&latest_log)?);
        output.push_str("\n\n");
    }
    
    // Try debug.log
    let debug_log = game_dir.join("logs").join("debug.log");
    if debug_log.exists() {
        output.push_str(&format!("=== debug.log ===\n"));
        output.push_str(&std::fs::read_to_string(&debug_log)?);
    }
    
    if output.is_empty() {
        output.push_str("No log files found in logs/ directory");
    }
    
    Ok(output)
}

/// Extract error messages from Minecraft log content
///
/// Looks for common error patterns:
/// - Lines containing "ERROR"
/// - Lines containing "FATAL"
/// - Java exceptions (lines starting with "at " or "Caused by:")
/// - Crash report references
pub fn extract_errors_from_log(log_content: &str) -> Vec<String> {
    let mut errors = Vec::new();
    let mut in_exception = false;
    
    for line in log_content.lines() {
        let trimmed = line.trim();
        
        // Error/Fatal markers
        if trimmed.contains("ERROR") || trimmed.contains("FATAL") {
            errors.push(line.to_string());
            in_exception = true;
            continue;
        }
        
        // Exception stack traces
        if trimmed.starts_with("at ") 
            || trimmed.starts_with("Caused by:") 
            || trimmed.starts_with("Suppressed:") {
            if in_exception {
                errors.push(line.to_string());
            }
            continue;
        }
        
        // Crash report
        if trimmed.contains("crash report") || trimmed.contains("crash-reports") {
            errors.push(line.to_string());
            continue;
        }
        
        // End of exception block
        if in_exception && trimmed.is_empty() {
            in_exception = false;
        }
    }
    
    errors
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_extract_errors() {
        let log = r#"
[12:00:00] [main/INFO]: Starting Minecraft
[12:00:01] [main/ERROR]: Failed to load mod
at net.minecraft.Main.main(Main.java:123)
at java.base/java.lang.Thread.run(Thread.java:834)
[12:00:02] [main/INFO]: Game crashed
"#;
        
        let errors = extract_errors_from_log(log);
        assert_eq!(errors.len(), 3);
        assert!(errors[0].contains("ERROR"));
    }
}
