use crate::error::{TorrerError, TorrerResult};

/// Validation utilities
pub struct Validator;

impl Validator {
    /// Validate IP address
    pub fn validate_ip(ip: &str) -> TorrerResult<()> {
        use std::net::IpAddr;
        ip.parse::<IpAddr>().map_err(|_| {
            TorrerError::Config(format!("Invalid IP address: {}", ip))
        })?;
        Ok(())
    }

    /// Validate port number
    pub fn validate_port(port: u16) -> TorrerResult<()> {
        if port == 0 || port > 65535 {
            return Err(TorrerError::Config(
                format!("Invalid port number: {} (must be 1-65535)", port)
            ));
        }
        Ok(())
    }

    /// Validate country code
    pub fn validate_country_code(code: &str) -> TorrerResult<()> {
        if code.len() != 2 {
            return Err(TorrerError::Config(
                "Country code must be 2 letters (e.g., CA, US)".to_string()
            ));
        }

        if !code.chars().all(|c| c.is_ascii_alphabetic()) {
            return Err(TorrerError::Config(
                "Country code must contain only letters".to_string()
            ));
        }

        Ok(())
    }

    /// Validate bridge format
    pub fn validate_bridge(bridge: &str) -> TorrerResult<()> {
        let parts: Vec<&str> = bridge.split(':').collect();
        if parts.len() != 2 {
            return Err(TorrerError::Config(
                "Invalid bridge format. Expected IP:PORT".to_string()
            ));
        }

        Self::validate_ip(parts[0])?;
        let port: u16 = parts[1].parse().map_err(|_| {
            TorrerError::Config("Invalid port number in bridge".to_string())
        })?;
        Self::validate_port(port)?;

        Ok(())
    }

    /// Validate file path
    pub fn validate_file_path(path: &str) -> TorrerResult<()> {
        use std::path::Path;
        let path = Path::new(path);
        
        if path.parent().is_none() {
            return Err(TorrerError::Config(
                "Invalid file path: missing directory".to_string()
            ));
        }

        Ok(())
    }

    /// Validate directory path
    pub fn validate_dir_path(path: &str) -> TorrerResult<()> {
        use std::path::Path;
        let path = Path::new(path);
        
        if !path.is_dir() && path.parent().is_none() {
            return Err(TorrerError::Config(
                "Invalid directory path".to_string()
            ));
        }

        Ok(())
    }
}

