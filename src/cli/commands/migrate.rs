use crate::error::TorrerResult;
use crate::config::ConfigMigration;
use std::path::PathBuf;

/// Migrate configuration
pub fn migrate_config() -> TorrerResult<()> {
    println!("Migrating configuration...");

    let config_path = PathBuf::from("/etc/torrer/config.toml");
    
    match ConfigMigration::migrate(&config_path) {
        Ok(_) => {
            println!("✓ Configuration migrated successfully");
            Ok(())
        }
        Err(e) => {
            println!("✗ Migration failed: {}", e);
            Err(e)
        }
    }
}

