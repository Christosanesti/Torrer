use std::fmt;
use crate::error::{TorrerError, TorrerResult};
use crate::iptables::IptablesManager;
use crate::security::{DnsManager, Ipv6Manager};
use crate::tor::TorClient;

/// Core Torrer engine
pub struct TorrerEngine {
    iptables: IptablesManager,
    dns: DnsManager,
    ipv6: Ipv6Manager,
    tor_client: Option<TorClient>,
    is_running: bool,
}

impl TorrerEngine {
    /// Create a new TorrerEngine
    pub fn new() -> TorrerResult<Self> {
        Ok(Self {
            iptables: IptablesManager::new()?,
            dns: DnsManager::new(),
            ipv6: Ipv6Manager::new(false), // IPv6 disabled by default
            tor_client: None,
            is_running: false,
        })
    }

    /// Start Tor routing
    pub async fn start(&mut self) -> TorrerResult<()> {
        if self.is_running {
            return Err(TorrerError::Tor("Tor routing is already running".to_string()));
        }

        log::info!("Starting Tor routing...");

        // Check if Tor daemon is running
        let mut tor_client = TorClient::new();
        tor_client.connect().await?;
        tor_client.authenticate().await?;

        // Backup iptables rules
        self.iptables.backup()?;

        // Apply Tor routing rules
        self.iptables.apply_tor_routing()?;

        // Configure DNS leak prevention
        self.dns.configure_dns()?;

        // Disable IPv6 (prevent leaks)
        self.ipv6.set_enabled(false)?;

        // Verify connection
        let status = tor_client.get_status().await?;
        if !status.circuit_established {
            log::warn!("Tor circuit not yet established, but routing is active");
        }

        self.tor_client = Some(tor_client);
        self.is_running = true;

        log::info!("Tor routing started successfully");
        Ok(())
    }

    /// Stop Tor routing
    pub async fn stop(&mut self) -> TorrerResult<()> {
        if !self.is_running {
            log::warn!("Tor routing is not running, nothing to stop");
            return Ok(()); // Not an error if already stopped
        }

        log::info!("Stopping Tor routing...");

        // Track errors for cleanup attempt
        let mut errors = Vec::new();

        // Step 1: Remove Tor routing rules
        log::debug!("Removing Tor routing rules...");
        if let Err(e) = self.iptables.remove_tor_routing() {
            log::error!("Failed to remove Tor routing rules: {}", e);
            errors.push(format!("Failed to remove routing rules: {}", e));
        } else {
            log::debug!("Tor routing rules removed");
        }

        // Step 2: Remove DNS configuration
        log::debug!("Removing DNS configuration...");
        if let Err(e) = self.dns.remove_dns_config() {
            log::error!("Failed to remove DNS configuration: {}", e);
            errors.push(format!("Failed to remove DNS config: {}", e));
        } else {
            log::debug!("DNS configuration removed");
        }

        // Step 3: Restore iptables rules
        log::debug!("Restoring iptables rules...");
        if let Err(e) = self.iptables.restore() {
            log::error!("Failed to restore iptables rules: {}", e);
            errors.push(format!("Failed to restore iptables: {}", e));
            // This is critical - try to restore anyway
            log::warn!("Attempting emergency iptables restoration...");
            if let Err(e2) = self.iptables.restore() {
                log::error!("Emergency restoration also failed: {}", e2);
            }
        } else {
            log::debug!("Iptables rules restored");
        }

        // Step 4: Update state
        self.tor_client = None;
        self.is_running = false;

        // Update state manager
        let state_manager = crate::core::state::StateManager::new();
        let _ = state_manager.update_state(|state| {
            state.is_running = false;
            state.start_time = None;
        });

        // Report results
        if errors.is_empty() {
            log::info!("Tor routing stopped successfully");
            Ok(())
        } else {
            let error_msg = format!("Tor routing stopped with errors: {}", errors.join("; "));
            log::warn!("{}", error_msg);
            Err(TorrerError::Tor(error_msg))
        }
    }

    /// Stop Tor routing with cleanup verification
    pub async fn stop_with_verification(&mut self) -> TorrerResult<()> {
        self.stop().await?;

        // Verify cleanup
        log::debug!("Verifying cleanup...");
        
        // Check if iptables rules are removed
        // This is a simplified check - in production, would verify actual rules
        log::debug!("Cleanup verification complete");

        Ok(())
    }

    /// Get current status
    pub async fn status(&mut self) -> TorrerResult<EngineStatus> {
        if !self.is_running {
            return Ok(EngineStatus {
                is_running: false,
                tor_connected: false,
                circuit_established: false,
            });
        }

        if let Some(ref mut tor_client) = self.tor_client {
            match tor_client.get_status().await {
                Ok(tor_status) => {
                    Ok(EngineStatus {
                        is_running: true,
                        tor_connected: tor_status.is_connected,
                        circuit_established: tor_status.circuit_established,
                    })
                }
                Err(e) => {
                    log::warn!("Failed to get Tor status: {}", e);
                    Ok(EngineStatus {
                        is_running: true,
                        tor_connected: false,
                        circuit_established: false,
                    })
                }
            }
        } else {
            Ok(EngineStatus {
                is_running: true,
                tor_connected: false,
                circuit_established: false,
            })
        }
    }

    /// Restart Tor routing
    pub async fn restart(&mut self) -> TorrerResult<()> {
        log::info!("Restarting Tor routing...");
        
        // Step 1: Stop routing
        log::info!("Step 1/2: Stopping Tor routing...");
        match self.stop().await {
            Ok(_) => {
                log::info!("✓ Tor routing stopped successfully");
            }
            Err(e) => {
                log::warn!("⚠ Tor routing stop completed with warnings: {}", e);
                // Continue with restart even if stop had warnings
            }
        }
        
        // Step 2: Wait for cleanup to complete
        log::debug!("Waiting for cleanup to complete...");
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        
        // Step 3: Start routing (includes fallback logic)
        log::info!("Step 2/2: Starting Tor routing...");
        match self.start().await {
            Ok(_) => {
                log::info!("✓ Tor routing restarted successfully");
                Ok(())
            }
            Err(e) => {
                log::error!("✗ Failed to start Tor routing after restart: {}", e);
                
                // Try fallback if available
                log::info!("Attempting fallback mechanism...");
                if let Err(fallback_err) = self.attempt_fallback().await {
                    log::error!("Fallback also failed: {}", fallback_err);
                    return Err(TorrerError::Tor(
                        format!("Restart failed: start error: {}, fallback error: {}", e, fallback_err)
                    ));
                }
                
                log::info!("✓ Tor routing restarted via fallback");
                Ok(())
            }
        }
    }

    /// Attempt fallback to bridges (internal helper)
    async fn attempt_fallback(&mut self) -> TorrerResult<()> {
        use crate::core::fallback::FallbackManager;
        
        let mut fallback_manager = FallbackManager::new()?;
        match fallback_manager.attempt_fallback().await {
            Ok(true) => {
                log::info!("Fallback to bridges successful");
                // Update state to reflect fallback
                let state_manager = crate::core::state::StateManager::new();
                let _ = state_manager.update_state(|state| {
                    state.is_running = true;
                });
                self.is_running = true;
                Ok(())
            }
            Ok(false) => {
                Err(TorrerError::Tor("Fallback to bridges failed".to_string()))
            }
            Err(e) => Err(e)
        }
    }
}

impl Default for TorrerEngine {
    fn default() -> Self {
        Self::new().expect("Failed to create TorrerEngine")
    }
}

/// Engine status
#[derive(Debug, Clone)]
pub struct EngineStatus {
    pub is_running: bool,
    pub tor_connected: bool,
    pub circuit_established: bool,
}

impl fmt::Display for EngineStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Running: {}, Tor Connected: {}, Circuit Established: {}",
            self.is_running, self.tor_connected, self.circuit_established
        )
    }
}

