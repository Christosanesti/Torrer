use crate::error::TorrerResult;
use crate::tor::TorClient;

/// Health check for Torrer system
pub struct HealthChecker;

impl HealthChecker {
    /// Perform comprehensive health check
    pub async fn check_all() -> TorrerResult<HealthStatus> {
        let mut status = HealthStatus {
            tor_daemon: false,
            tor_control: false,
            tor_circuit: false,
            iptables: false,
            dns: false,
        };

        // Check Tor daemon
        status.tor_daemon = Self::check_tor_daemon().await;

        // Check Tor control port
        if status.tor_daemon {
            status.tor_control = Self::check_tor_control().await;
            
            // Check circuit if control is working
            if status.tor_control {
                status.tor_circuit = Self::check_tor_circuit().await;
            }
        }

        // Check iptables
        status.iptables = Self::check_iptables();

        // Check DNS
        status.dns = Self::check_dns().await;

        Ok(status)
    }

    async fn check_tor_daemon() -> bool {
        use std::process::Command;
        Command::new("systemctl")
            .args(&["is-active", "--quiet", "tor"])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    async fn check_tor_control() -> bool {
        let mut client = TorClient::new();
        client.connect().await.is_ok()
    }

    async fn check_tor_circuit() -> bool {
        let mut client = TorClient::new();
        if let Ok(_) = client.connect().await {
            if let Ok(_) = client.authenticate().await {
                if let Ok(status) = client.get_status().await {
                    return status.circuit_established;
                }
            }
        }
        false
    }

    fn check_iptables() -> bool {
        use std::process::Command;
        Command::new("iptables")
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    async fn check_dns() -> bool {
        use std::process::Command;
        use tokio::time::{timeout, Duration};

        // Try to resolve a domain through Tor DNS
        let result = timeout(
            Duration::from_secs(5),
            tokio::process::Command::new("dig")
                .args(&["@127.0.0.1", "-p", "5353", "example.com", "+short"])
                .output(),
        )
        .await;

        match result {
            Ok(Ok(output)) => output.status.success(),
            _ => false,
        }
    }
}

/// Health status
#[derive(Debug, Clone)]
pub struct HealthStatus {
    pub tor_daemon: bool,
    pub tor_control: bool,
    pub tor_circuit: bool,
    pub iptables: bool,
    pub dns: bool,
}

impl HealthStatus {
    /// Check if all systems are healthy
    pub fn is_healthy(&self) -> bool {
        self.tor_daemon && self.tor_control && self.tor_circuit && self.iptables && self.dns
    }

    /// Get health score (0-100)
    pub fn score(&self) -> u8 {
        let mut score = 0;
        if self.tor_daemon { score += 20; }
        if self.tor_control { score += 20; }
        if self.tor_circuit { score += 20; }
        if self.iptables { score += 20; }
        if self.dns { score += 20; }
        score
    }
}

