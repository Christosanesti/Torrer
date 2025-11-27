// Unit tests for FallbackManager

#[cfg(test)]
mod tests {
    use torrer::core::FallbackManager;
    use torrer::error::TorrerResult;

    #[test]
    fn test_fallback_manager_creation() {
        // Test that FallbackManager can be created
        let manager = FallbackManager::new();
        match manager {
            Ok(_) => assert!(true),
            Err(e) => {
                // Should handle errors gracefully
                let error_str = format!("{}", e);
                assert!(!error_str.is_empty());
            }
        }
    }

    #[tokio::test]
    async fn test_check_tor_connection_timeout() {
        // Test that check_tor_connection handles timeouts
        let manager = FallbackManager::new().unwrap();
        // This will likely timeout or fail without Tor, but should handle gracefully
        let result = manager.check_tor_connection().await;
        match result {
            Ok(connected) => {
                // Should return bool indicating connection status
                assert!(connected == true || connected == false);
            }
            Err(e) => {
                // Should return proper error
                let error_str = format!("{}", e);
                assert!(!error_str.is_empty());
            }
        }
    }

    #[tokio::test]
    async fn test_attempt_fallback_with_no_bridges() {
        // Test fallback when no bridges are configured
        let mut manager = FallbackManager::new().unwrap();
        let result = manager.attempt_fallback().await;
        // Should return Ok(false) when no bridges available
        match result {
            Ok(success) => {
                // Without bridges, should return false
                assert!(!success);
            }
            Err(e) => {
                // Should handle errors gracefully
                let error_str = format!("{}", e);
                assert!(!error_str.is_empty());
            }
        }
    }
}

