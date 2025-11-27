// Integration tests for TorrerEngine
// Note: These tests require root access and Tor daemon

#[cfg(test)]
mod tests {
    use torrer::core::TorrerEngine;
    use torrer::error::TorrerResult;

    #[tokio::test]
    #[ignore] // Requires root and Tor daemon
    async fn test_engine_creation() {
        let engine = TorrerEngine::new();
        assert!(engine.is_ok());
    }

    #[tokio::test]
    #[ignore] // Requires root and Tor daemon
    async fn test_engine_status_when_stopped() {
        let mut engine = TorrerEngine::new().unwrap();
        let status = engine.status().await.unwrap();
        assert!(!status.is_running);
    }

    // Note: Start/stop tests require root access and should be run manually
    // in a controlled environment
}

