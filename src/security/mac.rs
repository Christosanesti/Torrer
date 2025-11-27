use std::process::Command;
use crate::error::{TorrerError, TorrerResult};

/// MAC address randomization manager
pub struct MacManager;

impl MacManager {
    /// Create a new MacManager
    pub fn new() -> Self {
        Self
    }

    /// Randomize MAC address for a network interface
    pub fn randomize_mac(&self, interface: &str) -> TorrerResult<()> {
        log::info!("Randomizing MAC address for interface: {}", interface);

        // Use macchanger to randomize MAC address
        let output = Command::new("macchanger")
            .args(&["-r", interface])
            .output()
            .map_err(|e| {
                TorrerError::Tor(format!("Failed to run macchanger: {}. Is macchanger installed?", e))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(TorrerError::Tor(format!(
                "macchanger failed: {}",
                stderr
            )));
        }

        log::info!("MAC address randomized for interface: {}", interface);
        Ok(())
    }

    /// Get current MAC address
    pub fn get_mac(&self, interface: &str) -> TorrerResult<String> {
        let output = Command::new("ip")
            .args(&["link", "show", interface])
            .output()
            .map_err(|e| {
                TorrerError::Tor(format!("Failed to get MAC address: {}", e))
            })?;

        if !output.status.success() {
            return Err(TorrerError::Tor("Failed to get MAC address".to_string()));
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        
        // Parse MAC address from ip link output
        for line in output_str.lines() {
            if line.contains("link/ether") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if let Some(mac) = parts.get(1) {
                    return Ok(mac.to_string());
                }
            }
        }

        Err(TorrerError::Tor("Could not parse MAC address".to_string()))
    }

    /// Randomize MAC for all active interfaces
    pub fn randomize_all(&self) -> TorrerResult<()> {
        log::info!("Randomizing MAC addresses for all active interfaces");

        // Get list of active interfaces
        let output = Command::new("ip")
            .args(&["link", "show", "up"])
            .output()
            .map_err(|e| {
                TorrerError::Tor(format!("Failed to list interfaces: {}", e))
            })?;

        if !output.status.success() {
            return Err(TorrerError::Tor("Failed to list network interfaces".to_string()));
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut interfaces = Vec::new();

        for line in output_str.lines() {
            if line.contains(":") && !line.contains("lo:") {
                let parts: Vec<&str> = line.split(':').collect();
                if let Some(iface) = parts.get(1) {
                    let iface_name = iface.trim().split_whitespace().next().unwrap_or("");
                    if !iface_name.is_empty() && iface_name != "lo" {
                        interfaces.push(iface_name.to_string());
                    }
                }
            }
        }

        // Randomize MAC for each interface
        for interface in interfaces {
            if let Err(e) = self.randomize_mac(&interface) {
                log::warn!("Failed to randomize MAC for {}: {}", interface, e);
            }
        }

        log::info!("MAC address randomization completed");
        Ok(())
    }
}

impl Default for MacManager {
    fn default() -> Self {
        Self::new()
    }
}

