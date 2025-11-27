use crate::error::TorrerResult;
use std::collections::HashMap;

/// Diagnostic information collector
pub struct Diagnostics;

impl Diagnostics {
    /// Collect comprehensive diagnostic information
    pub async fn collect_all() -> TorrerResult<DiagnosticInfo> {
        let mut info = DiagnosticInfo {
            system: Self::collect_system_info(),
            tor: Self::collect_tor_info().await,
            network: Self::collect_network_info().await,
            configuration: Self::collect_config_info(),
            bridges: Self::collect_bridge_info(),
        };

        Ok(info)
    }

    fn collect_system_info() -> HashMap<String, String> {
        let mut info = HashMap::new();
        
        // OS info
        if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
            for line in content.lines() {
                if line.starts_with("PRETTY_NAME=") {
                    info.insert("os".to_string(), 
                        line.split('=').nth(1)
                            .unwrap_or("Unknown")
                            .trim_matches('"')
                            .to_string());
                }
            }
        }

        // Kernel version
        if let Ok(output) = std::process::Command::new("uname")
            .arg("-r")
            .output()
        {
            if let Ok(version) = String::from_utf8(output.stdout) {
                info.insert("kernel".to_string(), version.trim().to_string());
            }
        }

        // Rust version
        if let Ok(output) = std::process::Command::new("rustc")
            .arg("--version")
            .output()
        {
            if let Ok(version) = String::from_utf8(output.stdout) {
                info.insert("rust_version".to_string(), version.trim().to_string());
            }
        }

        info
    }

    async fn collect_tor_info() -> HashMap<String, String> {
        let mut info = HashMap::new();

        // Tor daemon status
        let tor_running = std::process::Command::new("systemctl")
            .args(&["is-active", "--quiet", "tor"])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);
        
        info.insert("tor_running".to_string(), tor_running.to_string());

        // Tor version
        if let Ok(output) = std::process::Command::new("tor")
            .arg("--version")
            .output()
        {
            if let Ok(version) = String::from_utf8(output.stdout) {
                info.insert("tor_version".to_string(), 
                    version.lines().next().unwrap_or("Unknown").to_string());
            }
        }

        info
    }

    async fn collect_network_info() -> HashMap<String, String> {
        let mut info = HashMap::new();

        // Network interfaces
        if let Ok(output) = std::process::Command::new("ip")
            .args(&["link", "show"])
            .output()
        {
            if let Ok(output_str) = String::from_utf8(output.stdout) {
                let interfaces: Vec<&str> = output_str
                    .lines()
                    .filter_map(|line| {
                        if line.contains(":") && !line.contains("lo:") {
                            line.split(':').nth(1)
                                .and_then(|s| s.split_whitespace().next())
                        } else {
                            None
                        }
                    })
                    .collect();
                
                info.insert("interfaces".to_string(), interfaces.join(", "));
            }
        }

        info
    }

    fn collect_config_info() -> HashMap<String, String> {
        let mut info = HashMap::new();

        // Configuration file exists
        let config_exists = std::path::Path::new("/etc/torrer/config.toml").exists();
        info.insert("config_exists".to_string(), config_exists.to_string());

        // Try to load config
        if let Ok(config_manager) = crate::config::ConfigManager::new() {
            if let Ok(config) = config_manager.load() {
                info.insert("ipv6_enabled".to_string(), config.ipv6_enabled.to_string());
                info.insert("auto_fallback".to_string(), config.auto_fallback.to_string());
                if let Some(ref country) = config.country_code {
                    info.insert("country_code".to_string(), country.clone());
                }
            }
        }

        info
    }

    fn collect_bridge_info() -> HashMap<String, String> {
        let mut info = HashMap::new();

        if let Ok(bridge_manager) = crate::bridge::BridgeManager::new() {
            if let Ok(bridges) = bridge_manager.list_bridges() {
                info.insert("bridge_count".to_string(), bridges.len().to_string());
            }
        }

        info
    }
}

/// Diagnostic information
#[derive(Debug, Clone)]
pub struct DiagnosticInfo {
    pub system: HashMap<String, String>,
    pub tor: HashMap<String, String>,
    pub network: HashMap<String, String>,
    pub configuration: HashMap<String, String>,
    pub bridges: HashMap<String, String>,
}

impl DiagnosticInfo {
    /// Format as JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Format as human-readable text
    pub fn to_text(&self) -> String {
        let mut output = String::new();
        
        output.push_str("=== System Information ===\n");
        for (key, value) in &self.system {
            output.push_str(&format!("{}: {}\n", key, value));
        }
        
        output.push_str("\n=== Tor Information ===\n");
        for (key, value) in &self.tor {
            output.push_str(&format!("{}: {}\n", key, value));
        }
        
        output.push_str("\n=== Network Information ===\n");
        for (key, value) in &self.network {
            output.push_str(&format!("{}: {}\n", key, value));
        }
        
        output.push_str("\n=== Configuration ===\n");
        for (key, value) in &self.configuration {
            output.push_str(&format!("{}: {}\n", key, value));
        }
        
        output.push_str("\n=== Bridges ===\n");
        for (key, value) in &self.bridges {
            output.push_str(&format!("{}: {}\n", key, value));
        }
        
        output
    }
}

impl serde::Serialize for DiagnosticInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("DiagnosticInfo", 5)?;
        state.serialize_field("system", &self.system)?;
        state.serialize_field("tor", &self.tor)?;
        state.serialize_field("network", &self.network)?;
        state.serialize_field("configuration", &self.configuration)?;
        state.serialize_field("bridges", &self.bridges)?;
        state.end()
    }
}

