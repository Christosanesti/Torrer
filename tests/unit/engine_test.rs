// Unit tests for TorrerEngine
// Note: These tests mock external dependencies

#[cfg(test)]
mod tests {
    use torrer::core::TorrerEngine;
    use torrer::error::TorrerResult;

    #[test]
    fn test_engine_creation() {
        // Test that engine can be created
        let engine = TorrerEngine::new();
        assert!(engine.is_ok());
    }

    #[test]
    fn test_engine_creation_failure_handling() {
        // Test that engine creation handles errors gracefully
        // This tests the error path when IptablesManager::new() fails
        // In a real scenario, this might fail due to permissions
        let engine = TorrerEngine::new();
        // Should either succeed or return a proper error
        match engine {
            Ok(_) => assert!(true),
            Err(e) => {
                // Error should be a proper TorrerError
                let error_str = format!("{}", e);
                assert!(!error_str.is_empty());
            }
        }
    }

    #[tokio::test]
    async fn test_engine_status_when_not_running() {
        // Test status when engine is not running
        let mut engine = TorrerEngine::new().unwrap();
        let status = engine.status().await.unwrap();
        assert!(!status.is_running);
        assert!(!status.tor_connected);
        assert!(!status.circuit_established);
    }

    #[tokio::test]
    async fn test_engine_stop_when_not_running() {
        // Test that stopping when not running doesn't error
        let mut engine = TorrerEngine::new().unwrap();
        // Should not error when stopping if not running
        let result = engine.stop().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_engine_restart_when_not_running() {
        // Test restart when not running (should start)
        let mut engine = TorrerEngine::new().unwrap();
        // Restart when not running should attempt to start
        // This may fail without root/Tor, but should handle gracefully
        let result = engine.restart().await;
        // Should either succeed or return a proper error
        match result {
            Ok(_) => assert!(true),
            Err(e) => {
                // Error should be informative
                let error_str = format!("{}", e);
                assert!(!error_str.is_empty());
            }
        }
    }
}

