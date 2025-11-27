use crate::error::TorrerResult;
use crate::core::PersistenceManager;

/// List persisted data
pub fn list_data() -> TorrerResult<()> {
    let manager = PersistenceManager::new()?;
    
    match manager.list() {
        Ok(keys) => {
            if keys.is_empty() {
                println!("No persisted data found");
            } else {
                println!("Persisted data keys:");
                for key in keys {
                    println!("  - {}", key);
                }
            }
            Ok(())
        }
        Err(e) => {
            println!("Failed to list data: {}", e);
            Err(e)
        }
    }
}

/// Delete persisted data
pub fn delete_data(key: &str) -> TorrerResult<()> {
    let manager = PersistenceManager::new()?;
    
    match manager.delete(key) {
        Ok(_) => {
            println!("✓ Deleted data: {}", key);
            Ok(())
        }
        Err(e) => {
            println!("✗ Failed to delete data: {}", e);
            Err(e)
        }
    }
}

