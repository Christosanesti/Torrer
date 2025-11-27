use std::path::PathBuf;
use std::fs;
use std::io::Write;

use crate::error::{TorrerError, TorrerResult};
use crate::bridge::Bridge;

const BRIDGE_CONFIG_DIR: &str = "/etc/tor/torrer-bridges";
const BRIDGE_CONFIG_FILE: &str = "bridges.conf";

/// Bridge manager
pub struct BridgeManager {
    config_path: PathBuf,
}

impl BridgeManager {
    /// Create a new BridgeManager
    pub fn new() -> TorrerResult<Self> {
        let config_dir = PathBuf::from(BRIDGE_CONFIG_DIR);
        
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir).map_err(|e| {
                TorrerError::Bridge(format!("Failed to create bridge config directory: {}", e))
            })?;
        }

        Ok(Self {
            config_path: config_dir.join(BRIDGE_CONFIG_FILE),
        })
    }

    /// Add a bridge
    pub fn add_bridge(&self, bridge: Bridge) -> TorrerResult<()> {
        log::info!("Adding bridge: {}:{}", bridge.address, bridge.port);

        // Validate bridge format
        bridge.validate().map_err(|e| {
            TorrerError::Bridge(format!("Invalid bridge format: {}", e))
        })?;

        // Read existing bridges
        let mut bridges = self.list_bridges()?;

        // Check if bridge already exists
        if bridges.iter().any(|b| b.address == bridge.address && b.port == bridge.port) {
            return Err(TorrerError::Bridge(
                format!("Bridge {}:{} already exists", bridge.address, bridge.port)
            ));
        }

        bridges.push(bridge);
        self.save_bridges(&bridges)?;

        log::info!("Bridge added successfully");
        Ok(())
    }

    /// List all bridges
    pub fn list_bridges(&self) -> TorrerResult<Vec<Bridge>> {
        if !self.config_path.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&self.config_path).map_err(|e| {
            TorrerError::Bridge(format!("Failed to read bridge config: {}", e))
        })?;

        let bridges: Vec<Bridge> = content
            .lines()
            .filter_map(|line| {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    None
                } else {
                    Bridge::from_str(line).ok()
                }
            })
            .collect();

        Ok(bridges)
    }

    /// Remove a bridge
    pub fn remove_bridge(&self, address: &str, port: u16) -> TorrerResult<()> {
        let mut bridges = self.list_bridges()?;
        let initial_len = bridges.len();
        
        bridges.retain(|b| !(b.address == address && b.port == port));

        if bridges.len() == initial_len {
            return Err(TorrerError::Bridge("Bridge not found".to_string()));
        }

        self.save_bridges(&bridges)?;
        log::info!("Bridge removed successfully");
        Ok(())
    }

    /// Test bridge connectivity
    pub async fn test_bridge(&self, bridge: &Bridge) -> TorrerResult<bool> {
        log::info!("Testing bridge {}:{}", bridge.address, bridge.port);
        
        // Simple connectivity test - try to connect to bridge
        use tokio::net::TcpStream;
        use tokio::time::{timeout, Duration};

        let addr = format!("{}:{}", bridge.address, bridge.port);
        let result = timeout(Duration::from_secs(5), TcpStream::connect(&addr)).await;

        match result {
            Ok(Ok(_)) => {
                log::info!("Bridge {}:{} is reachable", bridge.address, bridge.port);
                Ok(true)
            }
            Ok(Err(e)) => {
                log::warn!("Bridge {}:{} is not reachable: {}", bridge.address, bridge.port, e);
                Ok(false)
            }
            Err(_) => {
                log::warn!("Bridge {}:{} connection timeout", bridge.address, bridge.port);
                Ok(false)
            }
        }
    }

    /// Save bridges to config file
    fn save_bridges(&self, bridges: &[Bridge]) -> TorrerResult<()> {
        let mut file = fs::File::create(&self.config_path).map_err(|e| {
            TorrerError::Bridge(format!("Failed to create bridge config file: {}", e))
        })?;

        writeln!(file, "# Torrer Bridge Configuration").map_err(|e| {
            TorrerError::Bridge(format!("Failed to write bridge config: {}", e))
        })?;

        for bridge in bridges {
            writeln!(file, "{}", bridge.to_tor_config()).map_err(|e| {
                TorrerError::Bridge(format!("Failed to write bridge config: {}", e))
            })?;
        }

        Ok(())
    }

    /// Get bridges for Tor configuration
    pub fn get_tor_bridges(&self) -> TorrerResult<Vec<String>> {
        let bridges = self.list_bridges()?;
        Ok(bridges.iter().map(|b| b.to_tor_config()).collect())
    }

    /// Read bridges from Tor config file
    pub fn read_from_tor_config(&self, torrc_path: &str) -> TorrerResult<Vec<Bridge>> {
        use std::path::PathBuf;
        let torrc = PathBuf::from(torrc_path);
        
        if !torrc.exists() {
            log::warn!("Tor config file not found: {}", torrc_path);
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&torrc).map_err(|e| {
            TorrerError::Bridge(format!("Failed to read Tor config file: {}", e))
        })?;

        let bridges: Vec<Bridge> = content
            .lines()
            .filter_map(|line| {
                let line = line.trim();
                if line.to_uppercase().starts_with("BRIDGE ") {
                    Bridge::from_str(line).ok()
                } else {
                    None
                }
            })
            .collect();

        log::info!("Read {} bridges from Tor config file", bridges.len());
        Ok(bridges)
    }

    /// Apply bridges to Tor configuration (write to separate bridge file)
    pub fn apply_to_tor(&self) -> TorrerResult<()> {
        let bridges = self.list_bridges()?;
        
        if bridges.is_empty() {
            log::info!("No bridges to apply");
            return Ok(());
        }

        // Write to Torrer bridge config file (separate from main torrc)
        // Tor can include this file via: Include /etc/tor/torrer-bridges/bridges.conf
        self.save_bridges(&bridges)?;
        
        log::info!("Applied {} bridges to Tor configuration", bridges.len());
        log::info!("Bridge config file: {:?}", self.config_path);
        log::info!("To use these bridges, add to /etc/tor/torrc:");
        log::info!("  Include {}", self.config_path.to_string_lossy());
        
        Ok(())
    }
}

impl Default for BridgeManager {
    fn default() -> Self {
        Self::new().expect("Failed to create BridgeManager")
    }
}

