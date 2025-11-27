use crate::error::TorrerResult;
use std::path::PathBuf;
use std::fs;

/// Clean temporary files and caches
pub fn clean(what: &str) -> TorrerResult<()> {
    match what {
        "logs" => clean_logs()?,
        "cache" => clean_cache()?,
        "backups" => clean_backups()?,
        "all" => {
            clean_logs()?;
            clean_cache()?;
            clean_backups()?;
        }
        _ => {
            return Err(crate::error::TorrerError::Config(
                format!("Unknown clean target: {}. Use: logs, cache, backups, or all", what)
            ));
        }
    }
    Ok(())
}

fn clean_logs() -> TorrerResult<()> {
    let log_dir = PathBuf::from("/var/log/torrer");
    
    if log_dir.exists() {
        println!("Cleaning log files...");
        let entries = fs::read_dir(&log_dir)?;
        let mut count = 0;
        
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                fs::remove_file(&path)?;
                count += 1;
            }
        }
        
        println!("✓ Removed {} log file(s)", count);
    } else {
        println!("No log directory found");
    }
    
    Ok(())
}

fn clean_cache() -> TorrerResult<()> {
    let cache_dir = PathBuf::from("/var/cache/torrer");
    
    if cache_dir.exists() {
        println!("Cleaning cache...");
        fs::remove_dir_all(&cache_dir)?;
        println!("✓ Cache cleaned");
    } else {
        println!("No cache directory found");
    }
    
    Ok(())
}

fn clean_backups() -> TorrerResult<()> {
    use crate::core::BackupManager;
    
    println!("Cleaning old backups...");
    BackupManager::clean_old_backups()?;
    println!("✓ Old backups cleaned");
    
    Ok(())
}

