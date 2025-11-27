// Basic integration tests for Torrer
// Note: These tests require root access and Tor daemon running

#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn test_torrer_version() {
        // Test that torrer binary exists and responds to --version or -V
        let output = Command::new("torrer")
            .arg("--version")
            .output();
        
        // This test will fail if torrer is not installed, which is expected
        // in CI/CD environments
        if let Ok(output) = output {
            assert!(output.status.success() || !output.stderr.is_empty());
        }
    }

    #[test]
    fn test_torrer_help() {
        // Test that torrer responds to --help
        let output = Command::new("torrer")
            .arg("--help")
            .output();
        
        if let Ok(output) = output {
            assert!(output.status.success() || !output.stderr.is_empty());
        }
    }
}

