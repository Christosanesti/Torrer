// Tor control protocol command builders

/// Build AUTHENTICATE command
pub fn build_authenticate(cookie: Option<&str>) -> String {
    match cookie {
        Some(c) => format!("AUTHENTICATE {}\r\n", c),
        None => "AUTHENTICATE\r\n".to_string(),
    }
}

/// Build GETINFO command
pub fn build_getinfo(key: &str) -> String {
    format!("GETINFO {}\r\n", key)
}

/// Build SIGNAL NEWNYM command
pub fn build_signal_newym() -> String {
    "SIGNAL NEWNYM\r\n".to_string()
}

/// Build SETCONF command
pub fn build_setconf(key: &str, value: &str) -> String {
    format!("SETCONF {}={}\r\n", key, value)
}

