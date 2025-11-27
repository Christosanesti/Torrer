// Unit tests for validation utilities

#[cfg(test)]
mod tests {
    use torrer::utils::Validator;

    #[test]
    fn test_validate_ip() {
        assert!(Validator::validate_ip("192.168.1.1").is_ok());
        assert!(Validator::validate_ip("::1").is_ok());
        assert!(Validator::validate_ip("invalid").is_err());
    }

    #[test]
    fn test_validate_port() {
        assert!(Validator::validate_port(80).is_ok());
        assert!(Validator::validate_port(65535).is_ok());
        assert!(Validator::validate_port(0).is_err());
        assert!(Validator::validate_port(65536).is_err());
    }

    #[test]
    fn test_validate_country_code() {
        assert!(Validator::validate_country_code("CA").is_ok());
        assert!(Validator::validate_country_code("US").is_ok());
        assert!(Validator::validate_country_code("INVALID").is_err());
        assert!(Validator::validate_country_code("C").is_err());
        assert!(Validator::validate_country_code("C1").is_err());
    }

    #[test]
    fn test_validate_bridge() {
        assert!(Validator::validate_bridge("192.168.1.1:443").is_ok());
        assert!(Validator::validate_bridge("invalid").is_err());
        assert!(Validator::validate_bridge("192.168.1.1").is_err());
    }
}

