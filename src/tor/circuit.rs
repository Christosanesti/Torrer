use crate::error::{TorrerError, TorrerResult};
use crate::tor::TorClient;

/// Circuit information and management
pub struct CircuitManager;

impl CircuitManager {
    /// Get circuit information
    pub async fn get_circuits(client: &mut TorClient) -> TorrerResult<Vec<CircuitInfo>> {
        // Query circuit status
        let command = "GETINFO circuit-status\r\n";
        let response = client.send_command(command).await?;

        // Parse circuit information
        let circuits = Self::parse_circuit_status(&response)?;

        Ok(circuits)
    }

    /// Request new circuit (NEWNYM)
    pub async fn new_circuit(client: &mut TorClient) -> TorrerResult<()> {
        log::info!("Requesting new Tor circuit...");
        
        let command = "SIGNAL NEWNYM\r\n";
        client.send_command(command).await?;

        log::info!("New circuit requested");
        Ok(())
    }

    /// Parse circuit status from Tor response
    fn parse_circuit_status(response: &str) -> TorrerResult<Vec<CircuitInfo>> {
        let mut circuits = Vec::new();

        // Parse circuit status response
        // Format: "250+circuit-status=\n<circuit lines>\n250 OK"
        // Each circuit line: "ID STATUS PURPOSE FLAGS BUILD_FLAGS TIME_CREATED"
        
        let mut in_circuit_section = false;
        
        for line in response.lines() {
            // Check if we're entering the circuit-status section
            if line.contains("circuit-status=") {
                in_circuit_section = true;
                continue;
            }
            
            // Check if we've left the circuit-status section
            if in_circuit_section && line.starts_with("250") && !line.contains("circuit-status") {
                break;
            }
            
            // Parse circuit lines
            if in_circuit_section && !line.trim().is_empty() && !line.starts_with("250") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let id = parts[0].to_string();
                    let status = parts[1].to_string();
                    
                    // Extract purpose (usually 3rd field)
                    let purpose = if parts.len() > 2 {
                        Some(parts[2].to_string())
                    } else {
                        None
                    };
                    
                    // Extract flags (4th field onwards, space-separated)
                    let flags = if parts.len() > 3 {
                        Some(parts[3..].join(" "))
                    } else {
                        None
                    };
                    
                    circuits.push(CircuitInfo {
                        id,
                        status,
                        purpose,
                        flags,
                    });
                }
            }
        }

        // If no circuits found with detailed parsing, try simple parsing
        if circuits.is_empty() {
            for line in response.lines() {
                if line.contains("circuit-status=") || line.contains("circuit ") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        let id = parts[0].split('=').last()
                            .or_else(|| parts[0].split(' ').last())
                            .unwrap_or("0")
                            .to_string();
                        let status = parts.get(1).copied().unwrap_or("UNKNOWN").to_string();
                        
                        circuits.push(CircuitInfo {
                            id,
                            status,
                            purpose: parts.get(2).map(|s| s.to_string()),
                            flags: if parts.len() > 3 {
                                Some(parts[3..].join(" "))
                            } else {
                                None
                            },
                        });
                    }
                }
            }
        }

        Ok(circuits)
    }
}

/// Circuit information
#[derive(Debug, Clone)]
pub struct CircuitInfo {
    pub id: String,
    pub status: String,
    pub purpose: Option<String>,
    pub flags: Option<String>,
}

