use crate::error::TorrerResult;

/// Display system information
pub async fn show_info() -> TorrerResult<()> {
    println!("=== Torrer System Information ===");
    println!();

    // Version
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    println!();

    // System info
    println!("=== System ===");
    if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            if line.starts_with("PRETTY_NAME=") {
                println!("OS: {}", 
                    line.split('=').nth(1)
                        .unwrap_or("Unknown")
                        .trim_matches('"'));
                break;
            }
        }
    }

    if let Ok(output) = std::process::Command::new("uname")
        .arg("-r")
        .output()
    {
        if let Ok(kernel) = String::from_utf8(output.stdout) {
            println!("Kernel: {}", kernel.trim());
        }
    }

    if let Ok(output) = std::process::Command::new("rustc")
        .arg("--version")
        .output()
    {
        if let Ok(version) = String::from_utf8(output.stdout) {
            println!("Rust: {}", version.trim());
        }
    }

    println!();

    // Tor info
    println!("=== Tor ===");
    let tor_running = std::process::Command::new("systemctl")
        .args(&["is-active", "--quiet", "tor"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);
    
    println!("Tor daemon: {}", if tor_running { "Running" } else { "Stopped" });

    if let Ok(output) = std::process::Command::new("tor")
        .arg("--version")
        .output()
    {
        if let Ok(version) = String::from_utf8(output.stdout) {
            println!("Tor version: {}", version.lines().next().unwrap_or("Unknown"));
        }
    }

    println!();

    // Configuration
    println!("=== Configuration ===");
    use crate::config::ConfigManager;
    if let Ok(config_manager) = ConfigManager::new() {
        if let Ok(config) = config_manager.load() {
            println!("IPv6 enabled: {}", config.ipv6_enabled);
            println!("Auto fallback: {}", config.auto_fallback);
            if let Some(ref country) = config.country_code {
                println!("Exit country: {}", country);
            }
        }
    }

    println!();

    // Bridges
    println!("=== Bridges ===");
    use crate::bridge::BridgeManager;
    if let Ok(bridge_manager) = BridgeManager::new() {
        if let Ok(bridges) = bridge_manager.list_bridges() {
            println!("Configured bridges: {}", bridges.len());
        }
    }

    Ok(())
}

