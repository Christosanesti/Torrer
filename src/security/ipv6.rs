use std::process::{Command, Stdio};

use crate::error::{TorrerError, TorrerResult};

/// IPv6 management
pub struct Ipv6Manager {
    enabled: bool,
}

impl Ipv6Manager {
    /// Create a new Ipv6Manager
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }

    /// Enable or disable IPv6
    pub fn set_enabled(&mut self, enabled: bool) -> TorrerResult<()> {
        self.enabled = enabled;
        
        if enabled {
            self.enable_ipv6()?;
        } else {
            self.disable_ipv6()?;
        }
        
        Ok(())
    }

    /// Check if IPv6 is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Enable IPv6
    fn enable_ipv6(&self) -> TorrerResult<()> {
        log::info!("Enabling IPv6");
        // Remove IPv6 blocking rules
        self.remove_ipv6_block()
    }

    /// Disable IPv6
    fn disable_ipv6(&self) -> TorrerResult<()> {
        log::info!("Disabling IPv6 to prevent leaks");
        
        // Block IPv6 traffic via iptables
        self.run_iptables(&["-A", "OUTPUT", "-p", "ipv6", "-j", "DROP"])?;
        
        Ok(())
    }

    /// Remove IPv6 block rules
    fn remove_ipv6_block(&self) -> TorrerResult<()> {
        let _ = self.run_iptables(&["-D", "OUTPUT", "-p", "ipv6", "-j", "DROP"]);
        Ok(())
    }

    /// Run an iptables command
    fn run_iptables(&self, args: &[&str]) -> TorrerResult<()> {
        let output = Command::new("iptables")
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| {
                TorrerError::Iptables(format!("Failed to run iptables: {}", e))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            log::debug!("iptables command failed: {} (args: {:?})", stderr, args);
        }

        Ok(())
    }
}

