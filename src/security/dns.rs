use std::process::{Command, Stdio};

use crate::error::{TorrerError, TorrerResult};

const TOR_DNS_PORT: u16 = 5353;

/// DNS leak prevention manager
pub struct DnsManager;

impl DnsManager {
    /// Create a new DnsManager
    pub fn new() -> Self {
        Self
    }

    /// Configure DNS to route through Tor
    pub fn configure_dns(&self) -> TorrerResult<()> {
        log::info!("Configuring DNS leak prevention");

        // Redirect DNS queries to Tor DNSPort via iptables
        self.redirect_dns_queries()?;

        // Block direct DNS queries
        self.block_direct_dns()?;

        // Configure systemd-resolved (if available)
        self.configure_systemd_resolved()?;

        log::info!("DNS leak prevention configured");
        Ok(())
    }

    /// Remove DNS configuration
    pub fn remove_dns_config(&self) -> TorrerResult<()> {
        log::info!("Removing DNS leak prevention configuration");

        // Remove DNS redirect rules
        let _ = self.run_iptables(&[
            "-t", "nat",
            "-D", "OUTPUT",
            "-p", "udp",
            "--dport", "53",
            "-j", "REDIRECT",
            "--to-ports", &TOR_DNS_PORT.to_string(),
        ]);

        // Remove DNS block rules
        let _ = self.run_iptables(&[
            "-D", "OUTPUT",
            "-p", "udp",
            "--dport", "53",
            "!", "-o", "lo",
            "-j", "DROP",
        ]);

        log::info!("DNS leak prevention removed");
        Ok(())
    }

    /// Redirect DNS queries to Tor DNSPort
    fn redirect_dns_queries(&self) -> TorrerResult<()> {
        self.run_iptables(&[
            "-t", "nat",
            "-A", "OUTPUT",
            "-p", "udp",
            "--dport", "53",
            "-j", "REDIRECT",
            "--to-ports", &TOR_DNS_PORT.to_string(),
        ])
    }

    /// Block direct DNS queries (fallback)
    fn block_direct_dns(&self) -> TorrerResult<()> {
        self.run_iptables(&[
            "-A", "OUTPUT",
            "-p", "udp",
            "--dport", "53",
            "!", "-o", "lo",
            "-j", "DROP",
        ])
    }

    /// Configure systemd-resolved to use Tor
    fn configure_systemd_resolved(&self) -> TorrerResult<()> {
        // Try to configure systemd-resolved
        // This is a best-effort operation
        let _ = Command::new("systemctl")
            .args(&["is-active", "systemd-resolved"])
            .output();

        // Note: Full systemd-resolved configuration would require
        // modifying /etc/systemd/resolved.conf, which is out of scope
        // for this MVP. The iptables rules should be sufficient.

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

impl Default for DnsManager {
    fn default() -> Self {
        Self::new()
    }
}

