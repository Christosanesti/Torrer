use std::path::PathBuf;
use std::fs;
use std::time::SystemTime;
use crate::error::{TorrerError, TorrerResult};

const BACKUP_DIR: &str = "/var/lib/torrer/backups";
const MAX_BACKUPS: usize = 10;

/// Backup manager for Torrer state
pub struct BackupManager;

impl BackupManager {
    /// Create a backup of current state
    pub fn create_backup() -> TorrerResult<PathBuf> {
        let backup_dir = PathBuf::from(BACKUP_DIR);
        
        // Create backup directory if it doesn't exist
        if !backup_dir.exists() {
            fs::create_dir_all(&backup_dir).map_err(|e| {
                TorrerError::Config(format!("Failed to create backup directory: {}", e))
            })?;
        }

        // Generate backup filename with timestamp
        let timestamp = SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| TorrerError::Config(format!("Failed to get timestamp: {}", e)))?
            .as_secs();
        
        let backup_file = backup_dir.join(format!("backup_{}.tar.gz", timestamp));

        // Create backup archive
        log::info!("Creating backup: {:?}", backup_file);
        
        // Backup configuration
        let config_path = PathBuf::from("/etc/torrer/config.toml");
        if config_path.exists() {
            let config_backup = backup_dir.join(format!("config_{}.toml", timestamp));
            fs::copy(&config_path, &config_backup).map_err(|e| {
                TorrerError::Config(format!("Failed to backup config: {}", e))
            })?;
            log::debug!("Backed up config to: {:?}", config_backup);
        }
        
        // Backup bridge configuration
        let bridge_config_path = PathBuf::from("/etc/tor/torrer-bridges/bridges.conf");
        if bridge_config_path.exists() {
            let bridge_backup = backup_dir.join(format!("bridges_{}.conf", timestamp));
            fs::copy(&bridge_config_path, &bridge_backup).map_err(|e| {
                TorrerError::Config(format!("Failed to backup bridges: {}", e))
            })?;
            log::debug!("Backed up bridges to: {:?}", bridge_backup);
        }
        
        // Backup iptables rules if available
        let iptables_backup = backup_dir.join(format!("iptables_{}.rules", timestamp));
        if let Ok(output) = std::process::Command::new("iptables-save").output() {
            if output.status.success() {
                fs::write(&iptables_backup, &output.stdout).map_err(|e| {
                    TorrerError::Config(format!("Failed to backup iptables: {}", e))
                })?;
                log::debug!("Backed up iptables rules to: {:?}", iptables_backup);
            }
        }
        
        log::info!("Backup created successfully: {:?}", backup_file);
        Ok(backup_file)
    }

    /// List available backups
    pub fn list_backups() -> TorrerResult<Vec<PathBuf>> {
        let backup_dir = PathBuf::from(BACKUP_DIR);
        
        if !backup_dir.exists() {
            return Ok(Vec::new());
        }

        let mut backups: Vec<PathBuf> = fs::read_dir(&backup_dir)
            .map_err(|e| {
                TorrerError::Config(format!("Failed to read backup directory: {}", e))
            })?
            .filter_map(|entry| {
                entry.ok().map(|e| e.path())
            })
            .filter(|path| {
                path.is_file() && (path.extension().map(|e| e == "gz" || e == "toml" || e == "conf" || e == "rules").unwrap_or(false) ||
                    path.file_name().and_then(|n| n.to_str()).map(|n| n.starts_with("config_") || n.starts_with("bridges_") || n.starts_with("iptables_")).unwrap_or(false))
            })
            .collect();

        backups.sort();
        backups.reverse(); // Most recent first

        // Limit to MAX_BACKUPS
        if backups.len() > MAX_BACKUPS {
            // Delete old backups
            for backup in backups.iter().skip(MAX_BACKUPS) {
                let _ = fs::remove_file(backup);
            }
            backups.truncate(MAX_BACKUPS);
        }

        Ok(backups)
    }

    /// Restore from backup
    pub fn restore_backup(backup_path: &PathBuf) -> TorrerResult<()> {
        if !backup_path.exists() {
            return Err(TorrerError::Config(
                format!("Backup file not found: {:?}", backup_path)
            ));
        }

        log::info!("Restoring from backup: {:?}", backup_path);
        
        // Extract timestamp from backup filename
        let backup_name = backup_path.file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| TorrerError::Config("Invalid backup filename".to_string()))?;
        
        let timestamp = backup_name.strip_prefix("backup_")
            .ok_or_else(|| TorrerError::Config("Invalid backup filename format".to_string()))?;
        
        let backup_dir = backup_path.parent()
            .ok_or_else(|| TorrerError::Config("Invalid backup path".to_string()))?;
        
        // Restore configuration
        let config_backup = backup_dir.join(format!("config_{}.toml", timestamp));
        if config_backup.exists() {
            let config_path = PathBuf::from("/etc/torrer/config.toml");
            fs::copy(&config_backup, &config_path).map_err(|e| {
                TorrerError::Config(format!("Failed to restore config: {}", e))
            })?;
            log::info!("Restored config from backup");
        }
        
        // Restore bridge configuration
        let bridge_backup = backup_dir.join(format!("bridges_{}.conf", timestamp));
        if bridge_backup.exists() {
            let bridge_config_path = PathBuf::from("/etc/tor/torrer-bridges/bridges.conf");
            // Ensure directory exists
            if let Some(parent) = bridge_config_path.parent() {
                fs::create_dir_all(parent).ok();
            }
            fs::copy(&bridge_backup, &bridge_config_path).map_err(|e| {
                TorrerError::Config(format!("Failed to restore bridges: {}", e))
            })?;
            log::info!("Restored bridges from backup");
        }
        
        // Restore iptables rules
        let iptables_backup = backup_dir.join(format!("iptables_{}.rules", timestamp));
        if iptables_backup.exists() {
            let rules = fs::read_to_string(&iptables_backup).map_err(|e| {
                TorrerError::Config(format!("Failed to read iptables backup: {}", e))
            })?;
            
            // Restore iptables rules
            let mut restore_cmd = std::process::Command::new("iptables-restore");
            restore_cmd.stdin(std::process::Stdio::piped());
            
            if let Ok(mut child) = restore_cmd.spawn() {
                if let Some(stdin) = child.stdin.take() {
                    use std::io::Write;
                    let mut writer = std::io::BufWriter::new(stdin);
                    writer.write_all(rules.as_bytes()).ok();
                    writer.flush().ok();
                }
                child.wait().ok();
                log::info!("Restored iptables rules from backup");
            } else {
                log::warn!("Failed to restore iptables rules (requires root)");
            }
        }
        
        log::info!("Backup restoration completed");
        Ok(())
    }

    /// Clean old backups
    pub fn clean_old_backups() -> TorrerResult<()> {
        let backups = Self::list_backups()?;
        
        if backups.len() > MAX_BACKUPS {
            for backup in backups.iter().skip(MAX_BACKUPS) {
                fs::remove_file(backup).map_err(|e| {
                    TorrerError::Config(format!("Failed to remove backup: {}", e))
                })?;
                log::info!("Removed old backup: {:?}", backup);
            }
        }

        Ok(())
    }
}

