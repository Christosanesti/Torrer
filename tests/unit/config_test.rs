// Unit tests for configuration

#[cfg(test)]
mod tests {
    use torrer::config::{Configuration, validate_config};

    #[test]
    fn test_default_configuration() {
        let config = Configuration::default();
        assert_eq!(config.tor_control_port, 9051);
        assert_eq!(config.tor_transport_port, 9040);
        assert_eq!(config.tor_dns_port, 5353);
        assert!(!config.ipv6_enabled);
        assert!(config.auto_fallback);
    }

    #[test]
    fn test_config_validation_valid() {
        let config = Configuration::default();
        assert!(validate_config(&config).is_ok());
    }

    #[test]
    fn test_config_validation_invalid_port() {
        let mut config = Configuration::default();
        config.tor_control_port = 0;
        assert!(validate_config(&config).is_err());
    }

    #[test]
    fn test_config_validation_invalid_country_code() {
        let mut config = Configuration::default();
        config.country_code = Some("INVALID".to_string());
        assert!(validate_config(&config).is_err());
    }

    #[test]
    fn test_config_validation_valid_country_code() {
        let mut config = Configuration::default();
        config.country_code = Some("CA".to_string());
        assert!(validate_config(&config).is_ok());
    }
}

