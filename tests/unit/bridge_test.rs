// Unit tests for bridge parsing

#[cfg(test)]
mod tests {
    use torrer::bridge::Bridge;

    #[test]
    fn test_bridge_from_str() {
        let bridge = Bridge::from_str("192.168.1.1:443");
        assert!(bridge.is_ok());
        
        let bridge = bridge.unwrap();
        assert_eq!(bridge.address, "192.168.1.1");
        assert_eq!(bridge.port, 443);
    }

    #[test]
    fn test_bridge_from_str_with_fingerprint() {
        let bridge = Bridge::from_str("192.168.1.1:443 ABC123DEF456");
        assert!(bridge.is_ok());
        
        let bridge = bridge.unwrap();
        assert_eq!(bridge.address, "192.168.1.1");
        assert_eq!(bridge.port, 443);
        assert_eq!(bridge.fingerprint, Some("ABC123DEF456".to_string()));
    }

    #[test]
    fn test_bridge_invalid_format() {
        let bridge = Bridge::from_str("invalid");
        assert!(bridge.is_err());
    }

    #[test]
    fn test_bridge_to_tor_config() {
        let bridge = Bridge::new("192.168.1.1".to_string(), 443);
        let config = bridge.to_tor_config();
        assert_eq!(config, "Bridge 192.168.1.1:443");
    }
}

