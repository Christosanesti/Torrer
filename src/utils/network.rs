use std::net::{IpAddr, Ipv4Addr};
use crate::error::{TorrerError, TorrerResult};

/// Validate IP address
pub fn validate_ip(ip: &str) -> TorrerResult<IpAddr> {
    ip.parse::<IpAddr>()
        .map_err(|_| TorrerError::Config(format!("Invalid IP address: {}", ip)))
}

/// Validate port number
pub fn validate_port(port: u16) -> TorrerResult<()> {
    if port == 0 || port > 65535 {
        return Err(TorrerError::Config(
            format!("Invalid port number: {} (must be 1-65535)", port)
        ));
    }
    Ok(())
}

/// Check if port is available
pub async fn is_port_available(port: u16) -> bool {
    use tokio::net::TcpListener;
    TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .is_ok()
}

/// Get local IP address
pub fn get_local_ip() -> Option<Ipv4Addr> {
    use std::net::UdpSocket;
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("8.8.8.8:80").ok()?;
    socket.local_addr().ok()?.ip().to_string().parse().ok()
}

