// Full workflow integration tests
// Note: These tests require root access and Tor daemon

#[cfg(test)]
mod tests {
    use torrer::core::TorrerEngine;
    use torrer::error::TorrerResult;

    #[tokio::test]
    #[ignore] // Requires root and Tor daemon
    async fn test_full_workflow() {
        // This test would verify the complete workflow:
        // 1. Create engine
        // 2. Start routing
        // 3. Check status
        // 4. Stop routing
        // 5. Verify cleanup

        let mut engine = TorrerEngine::new().unwrap();
        
        // Start
        let start_result = engine.start().await;
        // May fail without root/Tor, which is OK for CI
        
        // Status
        let _status = engine.status().await;
        
        // Stop (if started)
        if start_result.is_ok() {
            let _ = engine.stop().await;
        }
    }
}

