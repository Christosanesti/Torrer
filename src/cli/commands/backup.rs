use crate::error::TorrerResult;
use crate::core::BackupManager;
use std::path::PathBuf;

/// Create a backup
pub fn create_backup() -> TorrerResult<()> {
    println!("Creating backup...");
    
    match BackupManager::create_backup() {
        Ok(path) => {
            println!("✓ Backup created: {:?}", path);
            Ok(())
        }
        Err(e) => {
            println!("✗ Backup failed: {}", e);
            Err(e)
        }
    }
}

/// List backups
pub fn list_backups() -> TorrerResult<()> {
    println!("Available backups:");
    println!();

    match BackupManager::list_backups() {
        Ok(backups) => {
            if backups.is_empty() {
                println!("No backups found");
            } else {
                for (i, backup) in backups.iter().enumerate() {
                    println!("{}. {:?}", i + 1, backup);
                }
            }
            Ok(())
        }
        Err(e) => {
            println!("Failed to list backups: {}", e);
            Err(e)
        }
    }
}

/// Restore from backup
pub fn restore_backup(path: &str) -> TorrerResult<()> {
    println!("Restoring from backup: {}", path);
    
    let backup_path = PathBuf::from(path);
    
    match BackupManager::restore_backup(&backup_path) {
        Ok(_) => {
            println!("✓ Backup restored successfully");
            Ok(())
        }
        Err(e) => {
            println!("✗ Restore failed: {}", e);
            Err(e)
        }
    }
}

/// Clean old backups
pub fn clean_backups() -> TorrerResult<()> {
    println!("Cleaning old backups...");
    
    match BackupManager::clean_old_backups() {
        Ok(_) => {
            println!("✓ Old backups cleaned");
            Ok(())
        }
        Err(e) => {
            println!("✗ Clean failed: {}", e);
            Err(e)
        }
    }
}

