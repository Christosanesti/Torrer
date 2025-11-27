use serde::{Serialize, Deserialize};
use crate::config::Configuration;

/// Configuration schema for validation
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigSchema {
    pub version: String,
    pub schema: ConfigurationSchema,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigurationSchema {
    pub tor_control_port: FieldSchema,
    pub tor_transport_port: FieldSchema,
    pub tor_dns_port: FieldSchema,
    pub ipv6_enabled: FieldSchema,
    pub auto_fallback: FieldSchema,
    pub country_code: FieldSchema,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FieldSchema {
    pub r#type: String,
    pub required: bool,
    pub default: Option<serde_json::Value>,
    pub description: String,
}

impl ConfigSchema {
    /// Get default schema
    pub fn default() -> Self {
        Self {
            version: "1.0".to_string(),
            schema: ConfigurationSchema {
                tor_control_port: FieldSchema {
                    r#type: "integer".to_string(),
                    required: false,
                    default: Some(serde_json::Value::Number(9051.into())),
                    description: "Tor control port".to_string(),
                },
                tor_transport_port: FieldSchema {
                    r#type: "integer".to_string(),
                    required: false,
                    default: Some(serde_json::Value::Number(9040.into())),
                    description: "Tor transport port".to_string(),
                },
                tor_dns_port: FieldSchema {
                    r#type: "integer".to_string(),
                    required: false,
                    default: Some(serde_json::Value::Number(5353.into())),
                    description: "Tor DNS port".to_string(),
                },
                ipv6_enabled: FieldSchema {
                    r#type: "boolean".to_string(),
                    required: false,
                    default: Some(serde_json::Value::Bool(false)),
                    description: "Enable IPv6".to_string(),
                },
                auto_fallback: FieldSchema {
                    r#type: "boolean".to_string(),
                    required: false,
                    default: Some(serde_json::Value::Bool(true)),
                    description: "Enable automatic fallback".to_string(),
                },
                country_code: FieldSchema {
                    r#type: "string".to_string(),
                    required: false,
                    default: None,
                    description: "Exit node country code".to_string(),
                },
            },
        }
    }

    /// Validate configuration against schema
    pub fn validate(&self, config: &Configuration) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Validate ports
        if config.tor_control_port == 0 || config.tor_control_port > 65535 {
            errors.push("tor_control_port must be 1-65535".to_string());
        }
        if config.tor_transport_port == 0 || config.tor_transport_port > 65535 {
            errors.push("tor_transport_port must be 1-65535".to_string());
        }
        if config.tor_dns_port == 0 || config.tor_dns_port > 65535 {
            errors.push("tor_dns_port must be 1-65535".to_string());
        }

        // Validate country code if present
        if let Some(ref country) = config.country_code {
            if country.len() != 2 {
                errors.push("country_code must be 2 letters".to_string());
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

