use crate::error::TorrerResult;
use std::net::UdpSocket;
use std::time::Duration;

/// DNS leak detection
pub struct LeakDetector;

impl LeakDetector {
    /// Create a new leak detector
    pub fn new() -> Self {
        Self
    }

    /// Test for DNS leaks
    pub async fn test_dns_leak(&self) -> TorrerResult<LeakTestResult> {
        log::info!("Testing for DNS leaks...");

        // Test DNS resolution through Tor
        let tor_dns_works = self.test_tor_dns().await;
        
        // Test direct DNS (should fail if properly configured)
        let direct_dns_works = self.test_direct_dns().await;

        let result = LeakTestResult {
            tor_dns_working: tor_dns_works,
            direct_dns_blocked: !direct_dns_works,
            leak_detected: direct_dns_works && !tor_dns_works,
        };

        if result.leak_detected {
            log::warn!("DNS leak detected!");
        } else {
            log::info!("No DNS leaks detected");
        }

        Ok(result)
    }

    /// Test IPv6 leaks
    pub async fn test_ipv6_leak(&self) -> TorrerResult<bool> {
        log::info!("Testing for IPv6 leaks...");

        // Try to create IPv6 socket (should fail if IPv6 is disabled)
        let ipv6_available = self.test_ipv6_connectivity().await;

        if ipv6_available {
            log::warn!("IPv6 connectivity detected - potential leak");
        } else {
            log::info!("IPv6 properly disabled");
        }

        Ok(ipv6_available)
    }

    async fn test_tor_dns(&self) -> bool {
        use tokio::time::timeout;
        use tokio::process::Command;

        let result = timeout(
            Duration::from_secs(5),
            Command::new("dig")
                .args(&["@127.0.0.1", "-p", "5353", "example.com", "+short"])
                .output(),
        )
        .await;

        match result {
            Ok(Ok(output)) => output.status.success(),
            _ => false,
        }
    }

    async fn test_direct_dns(&self) -> bool {
        use tokio::time::timeout;
        use tokio::process::Command;

        // Try to query Google DNS directly (should be blocked)
        let result = timeout(
            Duration::from_secs(2),
            Command::new("dig")
                .args(&["@8.8.8.8", "example.com", "+short"])
                .output(),
        )
        .await;

        match result {
            Ok(Ok(output)) => output.status.success(),
            _ => false,
        }
    }

    async fn test_ipv6_connectivity(&self) -> bool {
        use tokio::time::timeout;
        use tokio::net::TcpStream;

        // Try to connect to IPv6 address
        let result = timeout(
            Duration::from_secs(2),
            TcpStream::connect("[2001:4860:4860::8888]:80"),
        )
        .await;

        result.is_ok()
    }
}

impl Default for LeakDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Leak test result
#[derive(Debug, Clone)]
pub struct LeakTestResult {
    pub tor_dns_working: bool,
    pub direct_dns_blocked: bool,
    pub leak_detected: bool,
}

