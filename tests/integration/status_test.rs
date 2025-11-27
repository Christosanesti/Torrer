// Integration tests for status command
// Note: These tests may require root access and Tor daemon

#[cfg(test)]
mod tests {
    use torrer::core::TorrerEngine;
    use torrer::error::TorrerResult;

    #[tokio::test]
    #[ignore] // Requires root and Tor daemon - run manually in controlled environment
    async fn test_status_with_inactive_routing() {
        // Test status when routing is not active
        let mut engine = TorrerEngine::new().unwrap();
        
        let status = engine.status().await.unwrap();
        assert!(!status.is_running);
        assert!(!status.tor_connected);
        assert!(!status.circuit_established);
    }

    #[tokio::test]
    #[ignore] // Requires root and Tor daemon
    async fn test_status_output_formatting() {
        // Test that status output is properly formatted
        let mut engine = TorrerEngine::new().unwrap();
        
        let status = engine.status().await.unwrap();
        
        // Verify status can be formatted as string
        let status_str = format!("{}", status);
        assert!(!status_str.is_empty());
        assert!(status_str.contains("Running"));
        assert!(status_str.contains("Tor Connected"));
        assert!(status_str.contains("Circuit Established"));
    }

    #[tokio::test]
    #[ignore] // Requires root and Tor daemon
    async fn test_status_error_handling() {
        // Test that status handles errors gracefully
        let mut engine = TorrerEngine::new().unwrap();
        
        // Status should not panic even if Tor is not available
        let result = engine.status().await;
        assert!(result.is_ok());
        
        let status = result.unwrap();
        // When not running, should return inactive status
        assert!(!status.is_running);
    }

    // Note: Tests for active routing require:
    // 1. Root/sudo access
    // 2. Tor daemon running
    // 3. Proper iptables setup
    // These should be run manually in a controlled test environment
}

