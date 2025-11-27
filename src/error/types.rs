use thiserror::Error;

/// Custom error types for Torrer
#[derive(Debug, Error)]
pub enum TorrerError {
    #[error("Tor error: {0}")]
    Tor(String),
    
    #[error("iptables error: {0}")]
    Iptables(String),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Bridge error: {0}")]
    Bridge(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Parse error: {0}")]
    Parse(String),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

/// Result type alias for Torrer operations
pub type TorrerResult<T> = Result<T, TorrerError>;


