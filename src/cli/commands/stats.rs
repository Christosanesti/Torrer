// Statistics command - Story 3.6 implementation
use crate::error::TorrerResult;
use crate::core::Monitoring;
use crate::tor::TorClient;
use serde_json;
use std::fs::File;
use std::io::Write;

/// Show detailed statistics
pub async fn show_stats(format: Option<&str>) -> TorrerResult<()> {
    let mut monitoring = Monitoring::new();
    let stats = monitoring.get_stats();
    
    match format {
        Some("json") => {
            let json = serde_json::json!({
                "uptime_seconds": stats.uptime.map(|d| d.as_secs()),
                "bytes_sent": stats.bytes_sent,
                "bytes_received": stats.bytes_received,
                "connection_attempts": stats.connection_attempts,
                "successful_connections": stats.successful_connections,
                "success_rate": stats.success_rate,
            });
            println!("{}", serde_json::to_string_pretty(&json)?);
        }
        Some("csv") => {
            println!("uptime_seconds,bytes_sent,bytes_received,connection_attempts,successful_connections,success_rate");
            let uptime_secs = stats.uptime.map(|d| d.as_secs()).unwrap_or(0);
            println!("{},{},{},{},{},{:.2}", 
                uptime_secs,
                stats.bytes_sent,
                stats.bytes_received,
                stats.connection_attempts,
                stats.successful_connections,
                stats.success_rate
            );
        }
        _ => {
            println!("=== Torrer Statistics ===");
            println!();
            if let Some(uptime) = stats.uptime {
                let hours = uptime.as_secs() / 3600;
                let minutes = (uptime.as_secs() % 3600) / 60;
                let seconds = uptime.as_secs() % 60;
                println!("Uptime: {}h {}m {}s", hours, minutes, seconds);
            } else {
                println!("Uptime: Not started");
            }
            println!("Bytes sent: {}", format_bytes(stats.bytes_sent));
            println!("Bytes received: {}", format_bytes(stats.bytes_received));
            println!("Connection attempts: {}", stats.connection_attempts);
            println!("Successful connections: {}", stats.successful_connections);
            println!("Success rate: {:.2}%", stats.success_rate);
        }
    }
    
    Ok(())
}

/// Export statistics to file
pub async fn export_stats(path: &str, format: &str) -> TorrerResult<()> {
    let mut monitoring = Monitoring::new();
    let stats = monitoring.get_stats();
    
    let content = match format {
        "json" => {
            let json = serde_json::json!({
                "uptime_seconds": stats.uptime.map(|d| d.as_secs()),
                "bytes_sent": stats.bytes_sent,
                "bytes_received": stats.bytes_received,
                "connection_attempts": stats.connection_attempts,
                "successful_connections": stats.successful_connections,
                "success_rate": stats.success_rate,
            });
            serde_json::to_string_pretty(&json)?
        }
        "csv" => {
            let mut csv = String::from("uptime_seconds,bytes_sent,bytes_received,connection_attempts,successful_connections,success_rate\n");
            let uptime_secs = stats.uptime.map(|d| d.as_secs()).unwrap_or(0);
            csv.push_str(&format!("{},{},{},{},{},{:.2}\n", 
                uptime_secs,
                stats.bytes_sent,
                stats.bytes_received,
                stats.connection_attempts,
                stats.successful_connections,
                stats.success_rate
            ));
            csv
        }
        _ => {
            return Err(crate::error::TorrerError::Config(
                format!("Unsupported export format: {}. Use 'json' or 'csv'", format)
            ));
        }
    };
    
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    println!("Statistics exported to: {}", path);
    
    Ok(())
}

/// Monitor statistics in real-time
pub async fn monitor_stats(interval: u64) -> TorrerResult<()> {
    use tokio::time::{sleep, Duration};
    use std::io::{self, Write};
    
    println!("Monitoring Torrer statistics (updating every {} seconds)...", interval);
    println!("Press Ctrl+C to stop");
    println!();
    
    let mut monitoring = Monitoring::new();
    monitoring.start();
    
    loop {
        // Clear screen (ANSI escape code)
        print!("\x1B[2J\x1B[1;1H");
        io::stdout().flush()?;
        
        let stats = monitoring.get_stats();
        
        println!("=== Torrer Statistics (Live) ===");
        println!();
        if let Some(uptime) = stats.uptime {
            let hours = uptime.as_secs() / 3600;
            let minutes = (uptime.as_secs() % 3600) / 60;
            let seconds = uptime.as_secs() % 60;
            println!("Uptime: {}h {}m {}s", hours, minutes, seconds);
        }
        println!("Bytes sent: {}", format_bytes(stats.bytes_sent));
        println!("Bytes received: {}", format_bytes(stats.bytes_received));
        println!("Connection attempts: {}", stats.connection_attempts);
        println!("Successful connections: {}", stats.successful_connections);
        println!("Success rate: {:.2}%", stats.success_rate);
        println!();
        println!("Press Ctrl+C to stop");
        
        sleep(Duration::from_secs(interval)).await;
    }
}

fn format_bytes(bytes: u64) -> String {
    if bytes >= 1_000_000_000 {
        format!("{:.2} GB", bytes as f64 / 1_000_000_000.0)
    } else if bytes >= 1_000_000 {
        format!("{:.2} MB", bytes as f64 / 1_000_000.0)
    } else if bytes >= 1_000 {
        format!("{:.2} KB", bytes as f64 / 1_000.0)
    } else {
        format!("{} B", bytes)
    }
}



