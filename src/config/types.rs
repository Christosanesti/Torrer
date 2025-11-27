use serde::{Deserialize, Serialize};

/// Torrer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
    pub tor_control_port: u16,
    pub tor_transport_port: u16,
    pub tor_dns_port: u16,
    pub ipv6_enabled: bool,
    pub auto_fallback: bool,
    pub country_code: Option<String>,
    #[serde(default = "default_auto_collect_bridges")]
    pub auto_collect_bridges: bool,
    #[serde(default = "default_bridge_collection_interval")]
    pub bridge_collection_interval_days: u32,
}

fn default_auto_collect_bridges() -> bool {
    true
}

fn default_bridge_collection_interval() -> u32 {
    7
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            tor_control_port: 9051,
            tor_transport_port: 9040,
            tor_dns_port: 5353,
            ipv6_enabled: false,
            auto_fallback: true,
            country_code: None,
            auto_collect_bridges: true,
            bridge_collection_interval_days: 7,
        }
    }
}

