// Unit tests for iptables manager

#[cfg(test)]
mod tests {
    use torrer::iptables::IptablesManager;

    #[test]
    fn test_iptables_manager_creation() {
        // This test may fail if not run as root, which is expected
        let manager = IptablesManager::new();
        // Just verify it doesn't panic
        let _ = manager;
    }

    // Note: Actual iptables operations require root access
    // and should be tested in integration tests
}

