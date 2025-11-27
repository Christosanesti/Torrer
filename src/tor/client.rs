use std::time::Duration;
use std::fmt;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::timeout;

use crate::error::{TorrerError, TorrerResult};

const DEFAULT_CONTROL_PORT: u16 = 9051;
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);
const AUTH_COOKIE_PATH: &str = "/var/run/tor/control.authcookie";

/// Tor control port client
pub struct TorClient {
    stream: Option<TcpStream>,
    control_port: u16,
}

impl TorClient {
    /// Create a new TorClient instance
    pub fn new() -> Self {
        Self {
            stream: None,
            control_port: DEFAULT_CONTROL_PORT,
        }
    }

    /// Create a TorClient with custom control port
    pub fn with_port(port: u16) -> Self {
        Self {
            stream: None,
            control_port: port,
        }
    }

    /// Connect to Tor control port
    pub async fn connect(&mut self) -> TorrerResult<()> {
        let addr = format!("127.0.0.1:{}", self.control_port);
        
        log::info!("Connecting to Tor control port at {}", addr);
        
        let stream_result = timeout(DEFAULT_TIMEOUT, TcpStream::connect(&addr)).await;
        
        match stream_result {
            Ok(Ok(stream)) => {
                self.stream = Some(stream);
                log::info!("Connected to Tor control port");
                Ok(())
            }
            Ok(Err(e)) => {
                log::error!("Failed to connect to Tor: {}", e);
                Err(TorrerError::Tor(format!(
                    "Failed to connect to Tor control port: {}. Is Tor running?",
                    e
                )))
            }
            Err(_) => {
                log::error!("Connection timeout to Tor control port");
                Err(TorrerError::Tor(
                    "Connection timeout. Is Tor running?".to_string(),
                ))
            }
        }
    }

    /// Authenticate with Tor control port
    pub async fn authenticate(&mut self) -> TorrerResult<()> {
        if self.stream.is_none() {
            return Err(TorrerError::Tor("Not connected to Tor".to_string()));
        }

        log::info!("Authenticating with Tor control port");

        // Try cookie authentication first
        if let Ok(cookie) = std::fs::read(AUTH_COOKIE_PATH) {
            log::debug!("Using cookie authentication");
            let hex_cookie = hex::encode(cookie);
            let auth_cmd = format!("AUTHENTICATE {}\r\n", hex_cookie);
            
            match self.send_raw_command(&auth_cmd).await {
                Ok(response) => {
                    if response.contains("250") {
                        log::info!("Authenticated with cookie");
                        return Ok(());
                    } else {
                        log::warn!("Cookie authentication failed: {}", response);
                    }
                }
                Err(e) => {
                    log::warn!("Cookie authentication failed: {}", e);
                }
            }
        }

        // Fallback: try null authentication (if configured)
        log::debug!("Trying null authentication");
        let auth_cmd = "AUTHENTICATE\r\n";
        
        if let Err(e) = self.send_raw_command(auth_cmd).await {
            log::error!("Authentication failed: {}", e);
            return Err(TorrerError::Tor(format!(
                "Authentication failed. Please check Tor configuration."
            )));
        }

        log::info!("Authenticated successfully");
        Ok(())
    }

    /// Get Tor connection status
    pub async fn get_status(&mut self) -> TorrerResult<TorStatus> {
        if self.stream.is_none() {
            return Err(TorrerError::Tor("Not connected to Tor".to_string()));
        }

        // Check if circuit is established
        let circuit_cmd = "GETINFO status/circuit-established\r\n";
        let response = self.send_raw_command(circuit_cmd).await?;
        
        let circuit_established = response.contains("status/circuit-established=1");

        // Get circuit status
        let status_cmd = "GETINFO circuit-status\r\n";
        let circuit_response = self.send_raw_command(status_cmd).await?;

        Ok(TorStatus {
            is_connected: true,
            circuit_established,
            circuit_info: Some(CircuitInfo {
                status: circuit_response,
            }),
        })
    }

    /// Send a raw command to Tor
    async fn send_raw_command(&mut self, command: &str) -> TorrerResult<String> {
        let stream = self.stream.as_mut().ok_or_else(|| {
            TorrerError::Tor("Not connected to Tor".to_string())
        })?;

        log::debug!("Sending command: {}", command.trim());

        // Write command
        stream.write_all(command.as_bytes()).await.map_err(|e| {
            TorrerError::Tor(format!("Failed to write command: {}", e))
        })?;

        // Read response
        let mut buffer = vec![0u8; 4096];
        let n = timeout(DEFAULT_TIMEOUT, stream.read(&mut buffer)).await
            .map_err(|_| TorrerError::Tor("Read timeout".to_string()))?
            .map_err(|e| TorrerError::Tor(format!("Failed to read response: {}", e)))?;

        let response = String::from_utf8_lossy(&buffer[..n]).to_string();
        log::debug!("Received response: {}", response.trim());

        // Check for error responses
        if response.starts_with("515") {
            return Err(TorrerError::Tor("Authentication required".to_string()));
        }

        Ok(response)
    }

    /// Send a command and parse response
    pub async fn send_command(&mut self, command: &str) -> TorrerResult<String> {
        self.send_raw_command(command).await
    }
}

impl Default for TorClient {
    fn default() -> Self {
        Self::new()
    }
}

/// Tor connection status
#[derive(Debug, Clone)]
pub struct TorStatus {
    pub is_connected: bool,
    pub circuit_established: bool,
    pub circuit_info: Option<CircuitInfo>,
}

/// Circuit information
#[derive(Debug, Clone)]
pub struct CircuitInfo {
    pub status: String,
}

impl fmt::Display for TorStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Connected: {}, Circuit Established: {}",
            self.is_connected, self.circuit_established
        )
    }
}

