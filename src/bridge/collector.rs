use std::collections::{HashSet, HashMap};
use std::time::{SystemTime, Duration as StdDuration};
use crate::error::{TorrerError, TorrerResult};
use crate::bridge::{Bridge, BridgeManager};
use tokio::time::{sleep, Duration};

const BRIDGE_API_URL: &str = "https://bridges.torproject.org/bridges";
const MOAT_API_URL: &str = "https://bridges.torproject.org/moat/circumvention/bridges";
const SNOWFLAKE_URL: &str = "https://snowflake.torproject.org/";
const USER_AGENT: &str = "Torrer/0.1.0";

/// Bridge metadata for prioritization
#[derive(Debug, Clone)]
struct BridgeMetadata {
    success_count: u32,
    failure_count: u32,
    last_tested: Option<SystemTime>,
    last_success: Option<SystemTime>,
}

impl BridgeMetadata {
    fn new() -> Self {
        Self {
            success_count: 0,
            failure_count: 0,
            last_tested: None,
            last_success: None,
        }
    }

    fn success_rate(&self) -> f64 {
        let total = self.success_count + self.failure_count;
        if total == 0 {
            0.0
        } else {
            (self.success_count as f64 / total as f64) * 100.0
        }
    }

    fn record_success(&mut self) {
        self.success_count += 1;
        self.last_success = Some(SystemTime::now());
        self.last_tested = Some(SystemTime::now());
    }

    fn record_failure(&mut self) {
        self.failure_count += 1;
        self.last_tested = Some(SystemTime::now());
    }
}

/// Automatic bridge and Snowflake collector
pub struct BridgeCollector {
    bridge_manager: BridgeManager,
    cache: HashSet<String>,
    metadata: HashMap<String, BridgeMetadata>,
}

impl BridgeCollector {
    /// Create a new BridgeCollector
    pub fn new() -> TorrerResult<Self> {
        Ok(Self {
            bridge_manager: BridgeManager::new()?,
            cache: HashSet::new(),
            metadata: HashMap::new(),
        })
    }

    /// Collect bridges from Tor Project API
    pub async fn collect_bridges(&mut self) -> TorrerResult<Vec<Bridge>> {
        log::info!("Collecting bridges from Tor Project...");

        let mut all_bridges = Vec::new();

        // Try to fetch from Moat API (doesn't require email authentication)
        match self.fetch_from_moat_api().await {
            Ok(bridges) => {
                log::info!("Fetched {} bridges from Moat API", bridges.len());
                all_bridges.extend(bridges);
            }
            Err(e) => {
                log::warn!("Failed to fetch from Moat API: {}", e);
            }
        }

        // Try to fetch from alternative sources (public bridge lists)
        match self.fetch_from_public_sources().await {
            Ok(bridges) => {
                log::info!("Fetched {} bridges from public sources", bridges.len());
                all_bridges.extend(bridges);
            }
            Err(e) => {
                log::debug!("Failed to fetch from public sources: {}", e);
            }
        }

        // Remove duplicates
        let mut unique_bridges = Vec::new();
        let mut seen = HashSet::new();
        for bridge in all_bridges {
            let key = format!("{}:{}", bridge.address, bridge.port);
            if !seen.contains(&key) {
                seen.insert(key);
                unique_bridges.push(bridge);
            }
        }

        if unique_bridges.is_empty() {
            log::warn!("No bridges collected from APIs");
            log::info!("Use 'torrer add-bridge' to manually add bridges");
        } else {
            log::info!("Collected {} unique bridges", unique_bridges.len());
        }

        Ok(unique_bridges)
    }

    /// Fetch bridges from Tor Project Moat API
    async fn fetch_from_moat_api(&self) -> TorrerResult<Vec<Bridge>> {
        log::debug!("Attempting to fetch bridges from Moat API...");
        
        let client = reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| TorrerError::Bridge(format!("Failed to create HTTP client: {}", e)))?;

        // Moat API requires a JSON request with transport type
        let request_body = serde_json::json!({
            "transport": "obfs4"
        });

        let response = client
            .post(MOAT_API_URL)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| TorrerError::Bridge(format!("Moat API request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(TorrerError::Bridge(format!(
                "Moat API returned error status: {}",
                response.status()
            )));
        }

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| TorrerError::Bridge(format!("Failed to parse Moat API response: {}", e)))?;

        let mut bridges = Vec::new();

        // Parse bridges from Moat API response
        // Format: {"bridges": ["IP:PORT FINGERPRINT", ...]}
        if let Some(bridges_array) = json.get("bridges").and_then(|b| b.as_array()) {
            for bridge_str in bridges_array {
                if let Some(bridge_line) = bridge_str.as_str() {
                    match Bridge::from_str(bridge_line) {
                        Ok(bridge) => {
                            bridges.push(bridge);
                        }
                        Err(e) => {
                            log::debug!("Failed to parse bridge '{}': {}", bridge_line, e);
                        }
                    }
                }
            }
        }

        Ok(bridges)
    }

    /// Fetch bridges from public sources (fallback method)
    async fn fetch_from_public_sources(&self) -> TorrerResult<Vec<Bridge>> {
        log::debug!("Attempting to fetch bridges from public sources...");
        
        // Note: This is a placeholder for public bridge sources
        // In practice, you might fetch from:
        // - Public bridge lists maintained by the community
        // - Bridge databases
        // - Other trusted sources
        
        // For now, return empty - this can be extended with actual public sources
        Ok(Vec::new())
    }

    /// Collect bridges with error handling
    pub async fn collect_bridges_safe(&mut self) -> TorrerResult<Vec<Bridge>> {
        match self.collect_bridges().await {
            Ok(bridges) => Ok(bridges),
            Err(e) => {
                log::error!("Failed to collect bridges: {}", e);
                log::info!("Continuing with existing cached bridges");
                // Return empty list to allow graceful degradation
                Ok(Vec::new())
            }
        }
    }

    /// Collect Snowflake proxies
    pub async fn collect_snowflake(&mut self) -> TorrerResult<Vec<String>> {
        log::info!("Collecting Snowflake proxies...");

        // Snowflake is a pluggable transport that doesn't require bridge addresses
        // Instead, it uses a WebRTC-based proxy system
        // Users configure Snowflake by installing the snowflake-client
        
        // For Torrer, we can provide Snowflake configuration instructions
        // but Snowflake doesn't work the same way as traditional bridges
        
        log::info!("Snowflake is a pluggable transport that uses WebRTC");
        log::info!("To use Snowflake, configure it in your Tor config:");
        log::info!("  UseBridges 1");
        log::info!("  Bridge snowflake");
        log::info!("  ClientTransportPlugin snowflake exec /usr/bin/snowflake-client");
        
        // Return empty - Snowflake doesn't use bridge addresses
        Ok(Vec::new())
    }

    /// Test and cache collected bridges with prioritization
    pub async fn test_and_cache_bridges(&mut self, bridges: Vec<Bridge>) -> TorrerResult<usize> {
        let mut tested_count = 0;
        let mut successful_count = 0;
        
        log::info!("Testing {} collected bridges...", bridges.len());
        
        for bridge in bridges {
            let key = format!("{}:{}", bridge.address, bridge.port);
            tested_count += 1;
            
            // Test bridge connectivity
            match self.bridge_manager.test_bridge(&bridge).await {
                Ok(true) => {
                    // Bridge is reachable
                    if !self.cache.contains(&key) {
                        if let Err(e) = self.bridge_manager.add_bridge(bridge.clone()) {
                            log::warn!("Failed to cache bridge {}:{}: {}", bridge.address, bridge.port, e);
                        } else {
                            self.cache.insert(key.clone());
                            successful_count += 1;
                        }
                    }
                    
                    // Update metadata
                    let metadata = self.metadata.entry(key).or_insert_with(BridgeMetadata::new);
                    metadata.record_success();
                }
                Ok(false) => {
                    // Bridge is not reachable
                    let metadata = self.metadata.entry(key).or_insert_with(BridgeMetadata::new);
                    metadata.record_failure();
                    log::debug!("Bridge {}:{} failed connectivity test", bridge.address, bridge.port);
                }
                Err(e) => {
                    log::warn!("Error testing bridge {}:{}: {}", bridge.address, bridge.port, e);
                    let metadata = self.metadata.entry(key).or_insert_with(BridgeMetadata::new);
                    metadata.record_failure();
                }
            }
        }
        
        log::info!("Bridge testing complete: {} tested, {} successful", tested_count, successful_count);
        Ok(successful_count)
    }

    /// Cache collected bridges (without testing)
    pub async fn cache_bridges(&mut self, bridges: Vec<Bridge>) -> TorrerResult<()> {
        for bridge in bridges {
            let key = format!("{}:{}", bridge.address, bridge.port);
            if !self.cache.contains(&key) {
                if let Err(e) = self.bridge_manager.add_bridge(bridge.clone()) {
                    log::warn!("Failed to cache bridge {}:{}: {}", bridge.address, bridge.port, e);
                } else {
                    self.cache.insert(key);
                }
            }
        }
        Ok(())
    }

    /// Get prioritized bridges (sorted by success rate and recency)
    pub fn get_prioritized_bridges(&self) -> Vec<(String, f64)> {
        let mut prioritized: Vec<(String, f64)> = self.metadata
            .iter()
            .map(|(key, meta)| {
                let success_rate = meta.success_rate();
                let recency_bonus = if let Some(last_success) = meta.last_success {
                    let age = last_success.elapsed().unwrap_or(StdDuration::from_secs(0));
                    // Recent successes get a bonus (decay over 7 days)
                    let days_old = age.as_secs() / 86400;
                    if days_old < 7 {
                        (7.0 - days_old as f64) * 5.0
                    } else {
                        0.0
                    }
                } else {
                    0.0
                };
                (key.clone(), success_rate + recency_bonus)
            })
            .collect();
        
        prioritized.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        prioritized
    }

    /// Collect bridges and test them before caching
    pub async fn collect_and_test(&mut self) -> TorrerResult<usize> {
        let bridges = self.collect_bridges().await?;
        self.test_and_cache_bridges(bridges).await
    }

    /// Collect bridges (async version for cache_bridges)
    pub async fn collect_and_cache(&mut self) -> TorrerResult<()> {
        let bridges = self.collect_bridges().await?;
        self.cache_bridges(bridges).await?;
        Ok(())
    }

    /// Get cached bridges
    pub fn get_cached(&self) -> &HashSet<String> {
        &self.cache
    }

    /// Auto-collect and cache bridges periodically
    pub async fn auto_collect(&mut self, interval_seconds: u64) -> TorrerResult<()> {
        loop {
            match self.collect_bridges().await {
                Ok(bridges) => {
                    if let Err(e) = self.cache_bridges(bridges).await {
                        log::error!("Failed to cache bridges: {}", e);
                    }
                }
                Err(e) => {
                    log::error!("Failed to collect bridges: {}", e);
                }
            }

            sleep(Duration::from_secs(interval_seconds)).await;
        }
    }
}

impl Default for BridgeCollector {
    fn default() -> Self {
        Self::new().expect("Failed to create BridgeCollector")
    }
}

