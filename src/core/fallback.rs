use crate::error::{TorrerError, TorrerResult};
use crate::tor::TorClient;
use crate::bridge::BridgeManager;
use tokio::time::{timeout, Duration, sleep};
use std::time::Instant;

const TOR_CHECK_TIMEOUT: u64 = 30; // 30 seconds
const BRIDGE_CONNECTION_TIMEOUT: u64 = 60; // 60 seconds
const MAX_RETRIES: u32 = 4; // 4 retries with exponential backoff

/// Automatic fallback mechanism
pub struct FallbackManager {
    bridge_manager: BridgeManager,
    fallback_active: bool,
    retry_count: u32,
    last_fallback_attempt: Option<Instant>,
}

impl FallbackManager {
    /// Create a new FallbackManager
    pub fn new() -> TorrerResult<Self> {
        Ok(Self {
            bridge_manager: BridgeManager::new()?,
            fallback_active: false,
            retry_count: 0,
            last_fallback_attempt: None,
        })
    }

    /// Check if Tor connection is working (with 30s timeout)
    pub async fn check_tor_connection(&self) -> TorrerResult<bool> {
        log::debug!("Checking Tor connection (timeout: {}s)...", TOR_CHECK_TIMEOUT);
        
        let check_future = async {
            let mut client = TorClient::new();
            
            client.connect().await?;
            client.authenticate().await?;
            let status = client.get_status().await?;
            
            Ok::<bool, TorrerError>(status.circuit_established)
        };

        match timeout(Duration::from_secs(TOR_CHECK_TIMEOUT), check_future).await {
            Ok(Ok(true)) => {
                log::info!("Tor connection is working");
                Ok(true)
            }
            Ok(Ok(false)) => {
                log::warn!("Tor connection check failed: circuit not established");
                Ok(false)
            }
            Ok(Err(e)) => {
                log::warn!("Tor connection check error: {}", e);
                Ok(false)
            }
            Err(_) => {
                log::warn!("Tor connection check timed out after {}s", TOR_CHECK_TIMEOUT);
                Ok(false)
            }
        }
    }

    /// Attempt fallback to bridges (with 60s timeout per bridge)
    pub async fn attempt_fallback(&mut self) -> TorrerResult<bool> {
        log::info!("Tor connection failed, attempting fallback to bridges");
        self.last_fallback_attempt = Some(Instant::now());

        let bridges = self.bridge_manager.list_bridges()?;
        
        if bridges.is_empty() {
            log::warn!("No bridges configured for fallback");
            return Ok(false);
        }

        log::info!("Found {} bridges, testing connectivity...", bridges.len());

        // Try each bridge with timeout
        for (index, bridge) in bridges.iter().enumerate() {
            log::info!("Testing bridge {}/{}: {}:{}", index + 1, bridges.len(), bridge.address, bridge.port);
            
            let test_future = self.bridge_manager.test_bridge(bridge);
            
            match timeout(Duration::from_secs(BRIDGE_CONNECTION_TIMEOUT), test_future).await {
                Ok(Ok(true)) => {
                    log::info!("✓ Bridge {}:{} is available and reachable", bridge.address, bridge.port);
                    self.fallback_active = true;
                    self.retry_count = 0; // Reset retry count on success
                    return Ok(true);
                }
                Ok(Ok(false)) => {
                    log::warn!("✗ Bridge {}:{} is not reachable", bridge.address, bridge.port);
                }
                Ok(Err(e)) => {
                    log::warn!("✗ Bridge {}:{} test error: {}", bridge.address, bridge.port, e);
                }
                Err(_) => {
                    log::warn!("✗ Bridge {}:{} connection timeout ({}s)", bridge.address, bridge.port, BRIDGE_CONNECTION_TIMEOUT);
                }
            }
        }

        log::error!("All {} bridges failed, fallback unsuccessful", bridges.len());
        Ok(false)
    }

    /// Attempt fallback with exponential backoff retry
    pub async fn attempt_fallback_with_retry(&mut self) -> TorrerResult<bool> {
        let mut retry_delay = 1u64; // Start with 1 second
        
        for attempt in 1..=MAX_RETRIES {
            log::info!("Fallback attempt {}/{}", attempt, MAX_RETRIES);
            
            match self.attempt_fallback().await {
                Ok(true) => {
                    log::info!("✓ Fallback successful on attempt {}", attempt);
                    return Ok(true);
                }
                Ok(false) => {
                    if attempt < MAX_RETRIES {
                        log::info!("Fallback failed, retrying in {}s (exponential backoff)...", retry_delay);
                        sleep(Duration::from_secs(retry_delay)).await;
                        retry_delay *= 2; // Exponential backoff: 1s, 2s, 4s, 8s
                        self.retry_count = attempt;
                    } else {
                        log::error!("Fallback failed after {} attempts", MAX_RETRIES);
                        self.retry_count = MAX_RETRIES;
                    }
                }
                Err(e) => {
                    log::error!("Fallback error on attempt {}: {}", attempt, e);
                    if attempt < MAX_RETRIES {
                        sleep(Duration::from_secs(retry_delay)).await;
                        retry_delay *= 2;
                    }
                }
            }
        }

        Ok(false)
    }

    /// Get retry count
    pub fn get_retry_count(&self) -> u32 {
        self.retry_count
    }

    /// Get time since last fallback attempt
    pub fn time_since_last_attempt(&self) -> Option<Duration> {
        self.last_fallback_attempt.map(|instant| instant.elapsed())
    }

    /// Check if fallback is active
    pub fn is_fallback_active(&self) -> bool {
        self.fallback_active
    }

    /// Reset fallback state
    pub fn reset(&mut self) {
        self.fallback_active = false;
        self.retry_count = 0;
        self.last_fallback_attempt = None;
        log::info!("Fallback state reset");
    }
}

impl Default for FallbackManager {
    fn default() -> Self {
        Self::new().expect("Failed to create FallbackManager")
    }
}

