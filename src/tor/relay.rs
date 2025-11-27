use crate::error::{TorrerError, TorrerResult};
use crate::tor::TorClient;
use serde::{Serialize, Deserialize};

/// Tor relay information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelayInfo {
    pub fingerprint: String,
    pub nickname: Option<String>,
    pub address: Option<String>,
    pub country: Option<String>,
    pub is_exit: bool,
    pub is_guard: bool,
}

/// Relay manager
pub struct RelayManager;

impl RelayManager {
    /// Get relay information
    pub async fn get_relay_info(client: &mut TorClient, fingerprint: &str) -> TorrerResult<RelayInfo> {
        let command = format!("GETINFO ns/id/{}\r\n", fingerprint);
        let response = client.send_command(&command).await?;

        // Parse relay information from response
        // This is a simplified parser - actual implementation would be more robust
        let relay = Self::parse_relay_info(&response, fingerprint)?;

        Ok(relay)
    }

    /// Get exit relay information
    pub async fn get_exit_relay(client: &mut TorClient) -> TorrerResult<Option<RelayInfo>> {
        // Get circuit status to find exit relay
        let command = "GETINFO circuit-status\r\n";
        let response = client.send_command(command).await?;

        // Parse circuit status to find exit relay fingerprint
        // Format: "250+circuit-status=\n<circuit info>\n250 OK"
        if let Some(circuit_start) = response.find("circuit-status=") {
            let circuit_data = &response[circuit_start + 15..];
            
            // Look for BUILD_FLAGS or EXTENDED events with exit relay
            if let Some(extend_pos) = circuit_data.find("EXTENDED") {
                // Extract fingerprint from EXTENDED line
                // Format: "EXTENDED <fingerprint>"
                let extend_line = &circuit_data[extend_pos..];
                if let Some(fingerprint_start) = extend_line.find(' ') {
                    let fingerprint_end = extend_line[fingerprint_start + 1..]
                        .find(|c: char| c == ' ' || c == '\n' || c == '\r')
                        .unwrap_or(40);
                    let fingerprint = &extend_line[fingerprint_start + 1..fingerprint_start + 1 + fingerprint_end];
                    
                    if fingerprint.len() == 40 {
                        // Get full relay info
                        return Self::get_relay_info(client, fingerprint).await.map(Some);
                    }
                }
            }
        }
        
        // Fallback: try to get from circuit established status
        let status_cmd = "GETINFO status/circuit-established\r\n";
        let status_response = client.send_command(status_cmd).await?;
        
        if status_response.contains("circuit-established=1") {
            // Circuit is established, try to get exit node from circuit info
            // This is a simplified approach - full implementation would parse circuit details
            log::debug!("Circuit is established but exit relay details not available");
        }
        
        Ok(None)
    }

    fn parse_relay_info(response: &str, fingerprint: &str) -> TorrerResult<RelayInfo> {
        // Parse relay information from Tor GETINFO ns/id response
        // Format: "250+ns/id/<fingerprint>=\n<descriptor>\n250 OK"
        
        let mut relay = RelayInfo {
            fingerprint: fingerprint.to_string(),
            nickname: None,
            address: None,
            country: None,
            is_exit: false,
            is_guard: false,
        };
        
        // Extract nickname (r line)
        if let Some(nickname_start) = response.find("r ") {
            if let Some(nickname_end) = response[nickname_start..].find(' ') {
                let nickname_line = &response[nickname_start + 2..nickname_start + nickname_end];
                let parts: Vec<&str> = nickname_line.split_whitespace().collect();
                if !parts.is_empty() {
                    relay.nickname = Some(parts[0].to_string());
                }
                if parts.len() > 1 {
                    relay.address = Some(parts[1].to_string());
                }
            }
        }
        
        // Extract country (a line or from address)
        if let Some(addr_line) = response.find("a ") {
            if let Some(addr_end) = response[addr_line..].find('\n') {
                let addr = response[addr_line + 2..addr_line + addr_end].trim();
                if !addr.is_empty() {
                    relay.address = Some(addr.to_string());
                }
            }
        }
        
        // Check for exit flag (s line)
        if let Some(s_line) = response.find("s ") {
            let flags = &response[s_line + 2..];
            if let Some(flags_end) = flags.find('\n') {
                let flags_str = &flags[..flags_end];
                relay.is_exit = flags_str.contains("Exit");
                relay.is_guard = flags_str.contains("Guard");
            }
        }
        
        // Extract country from address or descriptor
        if relay.address.is_none() {
            // Try to extract from fingerprint or other fields
            if let Some(country_match) = response.find("country=") {
                if let Some(country_end) = response[country_match + 8..].find(|c: char| c == ' ' || c == '\n') {
                    relay.country = Some(response[country_match + 8..country_match + 8 + country_end].to_string());
                }
            }
        }
        
        Ok(relay)
    }
}

