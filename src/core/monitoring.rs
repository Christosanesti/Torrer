use std::time::{Duration, Instant};
use crate::error::TorrerResult;
use crate::tor::TorClient;

/// Advanced monitoring and statistics
pub struct Monitoring {
    start_time: Option<Instant>,
    bytes_sent: u64,
    bytes_received: u64,
    connection_attempts: u32,
    successful_connections: u32,
}

impl Monitoring {
    /// Create a new Monitoring instance
    pub fn new() -> Self {
        Self {
            start_time: None,
            bytes_sent: 0,
            bytes_received: 0,
            connection_attempts: 0,
            successful_connections: 0,
        }
    }

    /// Start monitoring
    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
        log::info!("Monitoring started");
    }

    /// Stop monitoring
    pub fn stop(&mut self) {
        self.start_time = None;
        log::info!("Monitoring stopped");
    }

    /// Get uptime
    pub fn uptime(&self) -> Option<Duration> {
        self.start_time.map(|start| start.elapsed())
    }

    /// Record connection attempt
    pub fn record_connection_attempt(&mut self) {
        self.connection_attempts += 1;
    }

    /// Record successful connection
    pub fn record_successful_connection(&mut self) {
        self.successful_connections += 1;
    }

    /// Record bytes sent
    pub fn record_bytes_sent(&mut self, bytes: u64) {
        self.bytes_sent += bytes;
    }

    /// Record bytes received
    pub fn record_bytes_received(&mut self, bytes: u64) {
        self.bytes_received += bytes;
    }

    /// Get statistics
    pub fn get_stats(&self) -> Statistics {
        Statistics {
            uptime: self.uptime(),
            bytes_sent: self.bytes_sent,
            bytes_received: self.bytes_received,
            connection_attempts: self.connection_attempts,
            successful_connections: self.successful_connections,
            success_rate: if self.connection_attempts > 0 {
                (self.successful_connections as f64 / self.connection_attempts as f64) * 100.0
            } else {
                0.0
            },
        }
    }

    /// Get detailed connection status
    pub async fn get_connection_status(&self, client: &mut TorClient) -> TorrerResult<ConnectionStatus> {
        let status = client.get_status().await?;
        
        Ok(ConnectionStatus {
            is_connected: status.is_connected,
            circuit_established: status.circuit_established,
            circuit_info: status.circuit_info,
        })
    }
}

impl Default for Monitoring {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics
#[derive(Debug, Clone)]
pub struct Statistics {
    pub uptime: Option<Duration>,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub connection_attempts: u32,
    pub successful_connections: u32,
    pub success_rate: f64,
}

/// Connection status
#[derive(Debug, Clone)]
pub struct ConnectionStatus {
    pub is_connected: bool,
    pub circuit_established: bool,
    pub circuit_info: Option<crate::tor::client::CircuitInfo>,
}

