// Unit tests for error types

#[cfg(test)]
mod tests {
    use torrer::error::{TorrerError, TorrerResult};

    #[test]
    fn test_tor_error_display() {
        let error = TorrerError::Tor("Test error".to_string());
        let error_msg = format!("{}", error);
        assert!(error_msg.contains("Test error"));
    }

    #[test]
    fn test_iptables_error_display() {
        let error = TorrerError::Iptables("Test iptables error".to_string());
        let error_msg = format!("{}", error);
        assert!(error_msg.contains("Test iptables error"));
    }

    #[test]
    fn test_config_error_display() {
        let error = TorrerError::Config("Test config error".to_string());
        let error_msg = format!("{}", error);
        assert!(error_msg.contains("Test config error"));
    }
}

