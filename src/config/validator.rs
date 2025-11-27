use crate::error::{TorrerError, TorrerResult};
use crate::config::Configuration;

/// Validate configuration values
pub fn validate_config(config: &Configuration) -> TorrerResult<()> {
    // Validate ports
    if config.tor_control_port == 0 || config.tor_control_port > 65535 {
        return Err(TorrerError::Config(
            "Invalid Tor control port (must be 1-65535)".to_string(),
        ));
    }

    if config.tor_transport_port == 0 || config.tor_transport_port > 65535 {
        return Err(TorrerError::Config(
            "Invalid Tor transport port (must be 1-65535)".to_string(),
        ));
    }

    if config.tor_dns_port == 0 || config.tor_dns_port > 65535 {
        return Err(TorrerError::Config(
            "Invalid Tor DNS port (must be 1-65535)".to_string(),
        ));
    }

    // Validate country code if present
    if let Some(ref country) = config.country_code {
        if country.len() != 2 {
            return Err(TorrerError::Config(
                "Invalid country code (must be 2 letters, e.g., CA, US)".to_string(),
            ));
        }

        if !country.chars().all(|c| c.is_ascii_alphabetic()) {
            return Err(TorrerError::Config(
                "Country code must contain only letters".to_string(),
            ));
        }
    }

    Ok(())
}

