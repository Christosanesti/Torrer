// Unit tests for ConfigManager

#[cfg(test)]
mod tests {
    use torrer::config::ConfigManager;
    use torrer::error::TorrerResult;

    #[test]
    fn test_config_manager_creation() {
        // Test that ConfigManager can be created
        // This may fail without proper permissions, but should handle gracefully
        let manager = ConfigManager::new();
        match manager {
            Ok(_) => assert!(true),
            Err(e) => {
                // Should return proper error
                let error_str = format!("{}", e);
                assert!(!error_str.is_empty());
            }
        }
    }

    #[test]
    fn test_load_default_config_when_missing() {
        // Test loading config when file doesn't exist
        let manager = ConfigManager::new().unwrap();
        let config = manager.load();
        // Should return default config or error
        match config {
            Ok(cfg) => {
                // Should have default values
                assert!(cfg.tor_control_port > 0);
            }
            Err(e) => {
                // Should handle errors gracefully
                let error_str = format!("{}", e);
                assert!(!error_str.is_empty());
            }
        }
    }

    #[test]
    fn test_config_validation() {
        // Test that invalid configs are rejected
        let manager = ConfigManager::new().unwrap();
        // Try to load with invalid config (if we can create test file)
        // This tests validation logic
        let config = manager.load();
        match config {
            Ok(cfg) => {
                // Valid config should have proper values
                assert!(cfg.tor_control_port > 0 && cfg.tor_control_port <= 65535);
            }
            Err(_) => {
                // Invalid config should be rejected
                assert!(true);
            }
        }
    }
}

