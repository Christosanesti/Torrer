use std::path::PathBuf;
use std::fs;
use serde::{Serialize, Deserialize};
use crate::error::{TorrerError, TorrerResult};

/// Persistent data storage
pub struct PersistenceManager {
    data_dir: PathBuf,
}

impl PersistenceManager {
    /// Create a new persistence manager
    pub fn new() -> TorrerResult<Self> {
        let data_dir = PathBuf::from("/var/lib/torrer/data");
        
        if !data_dir.exists() {
            fs::create_dir_all(&data_dir).map_err(|e| {
                TorrerError::Config(format!("Failed to create data directory: {}", e))
            })?;
        }

        Ok(Self { data_dir })
    }

    /// Save data to file
    pub fn save<T: Serialize>(&self, key: &str, data: &T) -> TorrerResult<()> {
        let file_path = self.data_dir.join(format!("{}.json", key));
        
        let content = serde_json::to_string_pretty(data).map_err(|e| {
            TorrerError::Config(format!("Failed to serialize data: {}", e))
        })?;

        fs::write(&file_path, content).map_err(|e| {
            TorrerError::Config(format!("Failed to write data file: {}", e))
        })?;

        log::debug!("Saved data to: {:?}", file_path);
        Ok(())
    }

    /// Load data from file
    pub fn load<T: for<'de> Deserialize<'de>>(&self, key: &str) -> TorrerResult<Option<T>> {
        let file_path = self.data_dir.join(format!("{}.json", key));
        
        if !file_path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&file_path).map_err(|e| {
            TorrerError::Config(format!("Failed to read data file: {}", e))
        })?;

        let data: T = serde_json::from_str(&content).map_err(|e| {
            TorrerError::Config(format!("Failed to parse data file: {}", e))
        })?;

        Ok(Some(data))
    }

    /// Delete data file
    pub fn delete(&self, key: &str) -> TorrerResult<()> {
        let file_path = self.data_dir.join(format!("{}.json", key));
        
        if file_path.exists() {
            fs::remove_file(&file_path).map_err(|e| {
                TorrerError::Config(format!("Failed to delete data file: {}", e))
            })?;
        }

        Ok(())
    }

    /// List all data files
    pub fn list(&self) -> TorrerResult<Vec<String>> {
        let mut keys = Vec::new();

        if self.data_dir.exists() {
            let entries = fs::read_dir(&self.data_dir).map_err(|e| {
                TorrerError::Config(format!("Failed to read data directory: {}", e))
            })?;

            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(name) = entry.file_name().to_str() {
                        if name.ends_with(".json") {
                            let key = name.trim_end_matches(".json");
                            keys.push(key.to_string());
                        }
                    }
                }
            }
        }

        Ok(keys)
    }
}

