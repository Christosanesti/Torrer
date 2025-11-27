use serde::{Deserialize, Serialize};

/// Tor bridge configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bridge {
    pub address: String,
    pub port: u16,
    pub fingerprint: Option<String>,
    pub transport: Option<String>,
}

impl Bridge {
    /// Create a new bridge
    pub fn new(address: String, port: u16) -> Self {
        Self {
            address,
            port,
            fingerprint: None,
            transport: None,
        }
    }

    /// Parse bridge from string (format: "IP:PORT" or "IP:PORT FINGERPRINT" or "Bridge IP:PORT FINGERPRINT TRANSPORT")
    pub fn from_str(s: &str) -> Result<Self, String> {
        let line = s.trim();
        if line.is_empty() {
            return Err("Empty bridge string".to_string());
        }

        // Remove "Bridge" prefix if present (from Tor config format)
        let line = if line.to_uppercase().starts_with("BRIDGE ") {
            &line[7..]
        } else {
            line
        };

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            return Err("Invalid bridge format. Expected IP:PORT".to_string());
        }

        let addr_part = parts[0];
        let addr_parts: Vec<&str> = addr_part.split(':').collect();
        if addr_parts.len() != 2 {
            return Err("Invalid bridge format. Expected IP:PORT (e.g., 1.2.3.4:443)".to_string());
        }

        let address = addr_parts[0].to_string();
        
        // Validate IP address or hostname
        if address.is_empty() {
            return Err("Bridge address cannot be empty".to_string());
        }

        let port = addr_parts[1].parse::<u16>()
            .map_err(|_| "Invalid port number. Must be between 1 and 65535".to_string())?;
        
        if port == 0 {
            return Err("Port number cannot be 0".to_string());
        }

        // Parse fingerprint (optional, typically 40 hex characters)
        let fingerprint = if parts.len() > 1 {
            let fp = parts[1].to_string();
            if fp.len() == 40 && fp.chars().all(|c| c.is_ascii_hexdigit()) {
                Some(fp)
            } else if !fp.is_empty() {
                // Allow non-standard fingerprints but warn
                Some(fp)
            } else {
                None
            }
        } else {
            None
        };

        // Parse transport (optional, e.g., obfs4, meek)
        let transport = if parts.len() > 2 {
            Some(parts[2].to_string())
        } else {
            None
        };

        Ok(Self {
            address,
            port,
            fingerprint,
            transport,
        })
    }

    /// Validate bridge format
    pub fn validate(&self) -> Result<(), String> {
        // Validate address (IP or hostname)
        if self.address.is_empty() {
            return Err("Bridge address cannot be empty".to_string());
        }

        // Validate port
        if self.port == 0 || self.port > 65535 {
            return Err(format!("Invalid port number: {}. Must be between 1 and 65535", self.port));
        }

        // Validate fingerprint if present
        if let Some(ref fp) = self.fingerprint {
            if !fp.is_empty() && fp.len() != 40 {
                // Warn but don't fail - some bridges may have non-standard fingerprints
                log::debug!("Non-standard fingerprint length: {} (expected 40)", fp.len());
            }
        }

        Ok(())
    }

    /// Convert bridge to Tor configuration format
    pub fn to_tor_config(&self) -> String {
        let mut config = format!("Bridge {}:{}", self.address, self.port);
        if let Some(ref fp) = self.fingerprint {
            config.push_str(&format!(" {}", fp));
        }
        if let Some(ref transport) = self.transport {
            config.push_str(&format!(" {}", transport));
        }
        config
    }
}

