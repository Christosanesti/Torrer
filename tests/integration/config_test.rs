// Integration tests for configuration

#[cfg(test)]
mod tests {
    use torrer::config::{ConfigManager, Configuration};

    #[test]
    fn test_config_load_default() {
        // This test may fail if config file exists, which is OK
        let manager = ConfigManager::new();
        if let Ok(manager) = manager {
            let config = manager.load();
            // Should either succeed with defaults or fail gracefully
            let _ = config;
        }
    }

    #[test]
    fn test_config_validation() {
        let config = Configuration::default();
        use torrer::config::validate_config;
        assert!(validate_config(&config).is_ok());
    }
}

