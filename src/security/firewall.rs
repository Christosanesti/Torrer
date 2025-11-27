use crate::error::{TorrerError, TorrerResult};
use std::process::Command;

/// Firewall management utilities
pub struct FirewallManager;

impl FirewallManager {
    /// Create a new firewall manager
    pub fn new() -> Self {
        Self
    }

    /// Check if firewall is active
    pub fn is_active(&self) -> bool {
        // Check ufw
        if Command::new("ufw")
            .args(&["status"])
            .output()
            .map(|o| {
                let output = String::from_utf8_lossy(&o.stdout);
                output.contains("Status: active")
            })
            .unwrap_or(false)
        {
            return true;
        }

        // Check firewalld
        if Command::new("systemctl")
            .args(&["is-active", "--quiet", "firewalld"])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            return true;
        }

        false
    }

    /// Get firewall type
    pub fn get_firewall_type(&self) -> Option<FirewallType> {
        if Command::new("which").arg("ufw").output().map(|o| o.status.success()).unwrap_or(false) {
            return Some(FirewallType::Ufw);
        }

        if Command::new("which").arg("firewall-cmd").output().map(|o| o.status.success()).unwrap_or(false) {
            return Some(FirewallType::Firewalld);
        }

        None
    }

    /// Configure firewall for Tor
    pub fn configure_tor(&self) -> TorrerResult<()> {
        match self.get_firewall_type() {
            Some(FirewallType::Ufw) => self.configure_ufw(),
            Some(FirewallType::Firewalld) => self.configure_firewalld(),
            None => {
                log::warn!("No firewall detected, skipping firewall configuration");
                Ok(())
            }
        }
    }

    fn configure_ufw(&self) -> TorrerResult<()> {
        log::info!("Configuring UFW for Tor");

        // Allow Tor ports
        let ports = vec!["9040/tcp", "5353/udp", "9051/tcp"];
        
        for port in ports {
            let output = Command::new("ufw")
                .args(&["allow", port])
                .output()
                .map_err(|e| {
                    TorrerError::Iptables(format!("Failed to configure UFW: {}", e))
                })?;

            if !output.status.success() {
                log::warn!("Failed to allow {} in UFW", port);
            }
        }

        Ok(())
    }

    fn configure_firewalld(&self) -> TorrerResult<()> {
        log::info!("Configuring firewalld for Tor");

        // Allow Tor ports
        let ports = vec!["9040/tcp", "5353/udp", "9051/tcp"];
        
        for port in ports {
            let output = Command::new("firewall-cmd")
                .args(&["--permanent", "--add-port", port])
                .output()
                .map_err(|e| {
                    TorrerError::Iptables(format!("Failed to configure firewalld: {}", e))
                })?;

            if !output.status.success() {
                log::warn!("Failed to allow {} in firewalld", port);
            }
        }

        // Reload firewalld
        let _ = Command::new("firewall-cmd")
            .arg("--reload")
            .output();

        Ok(())
    }
}

impl Default for FirewallManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Firewall type
#[derive(Debug, Clone, Copy)]
pub enum FirewallType {
    Ufw,
    Firewalld,
}

