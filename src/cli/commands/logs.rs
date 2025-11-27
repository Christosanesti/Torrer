use std::fs;
use std::path::PathBuf;
use std::io::BufReader;
use crate::error::TorrerResult;
use serde_json;

const LOG_DIR: &str = "/var/log/torrer";
const LOG_FILE: &str = "torrer.log";

/// View logs with filtering and formatting options
pub fn view_logs(follow: bool, tail: usize, level: Option<&str>, format: &str) -> TorrerResult<()> {
    let log_path = PathBuf::from(LOG_DIR).join(LOG_FILE);

    if !log_path.exists() {
        println!("No log file found at {:?}", log_path);
        return Ok(());
    }

    let content = fs::read_to_string(&log_path)?;
    let lines: Vec<&str> = content.lines().collect();
    
    // Apply tail filter
    let lines_to_show = if tail > 0 && tail < lines.len() {
        &lines[lines.len() - tail..]
    } else {
        &lines[..]
    };

    // Apply level filter
    let filtered_lines: Vec<&str> = if let Some(level_filter) = level {
        let level_upper = level_filter.to_uppercase();
        lines_to_show
            .iter()
            .filter(|line| {
                line.contains(&format!("[{}]", level_upper)) ||
                line.contains(&format!(" {} ", level_upper)) ||
                line.to_uppercase().contains(&level_upper)
            })
            .copied()
            .collect()
    } else {
        lines_to_show.to_vec()
    };

    // Format and display
    match format {
        "json" => {
            let json_lines: Vec<serde_json::Value> = filtered_lines
                .iter()
                .map(|line| {
                    serde_json::json!({
                        "message": line,
                        "raw": line
                    })
                })
                .collect();
            println!("{}", serde_json::to_string_pretty(&json_lines)?);
        }
        _ => {
            for line in filtered_lines {
                println!("{}", line);
            }
        }
    }

    if follow {
        println!("\nFollowing logs (press Ctrl+C to stop)...");
        println!("Note: Full tail -f implementation requires file watching");
        // In a full implementation, this would use inotify or similar
        // For now, we show the filtered logs
    }

    Ok(())
}

