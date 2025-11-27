use crate::config::Configuration;

/// Default configuration values
pub struct Defaults;

impl Defaults {
    /// Get default configuration
    pub fn config() -> Configuration {
        Configuration::default()
    }

    /// Get default Tor control port
    pub fn tor_control_port() -> u16 {
        9051
    }

    /// Get default Tor transport port
    pub fn tor_transport_port() -> u16 {
        9040
    }

    /// Get default Tor DNS port
    pub fn tor_dns_port() -> u16 {
        5353
    }

    /// Get default IPv6 setting
    pub fn ipv6_enabled() -> bool {
        false
    }

    /// Get default auto-fallback setting
    pub fn auto_fallback() -> bool {
        true
    }

    /// Get default country code
    pub fn country_code() -> Option<String> {
        None
    }

    /// Get configuration file path
    pub fn config_path() -> &'static str {
        "/etc/torrer/config.toml"
    }

    /// Get bridge config directory
    pub fn bridge_config_dir() -> &'static str {
        "/etc/tor/torrer-bridges"
    }

    /// Get log directory
    pub fn log_dir() -> &'static str {
        "/var/log/torrer"
    }

    /// Get backup directory
    pub fn backup_dir() -> &'static str {
        "/var/lib/torrer/backups"
    }
}

