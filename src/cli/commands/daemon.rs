use crate::error::TorrerResult;
use crate::core::DaemonManager;

/// Install systemd service
pub fn install_service() -> TorrerResult<()> {
    println!("Installing systemd service...");
    
    match DaemonManager::install_service() {
        Ok(_) => {
            println!("✓ Service installed");
            println!("Enable with: sudo systemctl enable torrer");
            Ok(())
        }
        Err(e) => {
            println!("✗ Installation failed: {}", e);
            Err(e)
        }
    }
}

/// Show service status
pub fn service_status() -> TorrerResult<()> {
    println!("=== Systemd Service Status ===");
    println!();

    if DaemonManager::is_service_installed() {
        println!("Service: Installed");
    } else {
        println!("Service: Not installed");
        return Ok(());
    }

    if DaemonManager::is_service_enabled() {
        println!("Enabled: Yes");
    } else {
        println!("Enabled: No");
    }

    // Check if running
    use std::process::Command;
    let output = Command::new("systemctl")
        .args(&["is-active", "torrer"])
        .output();

    if let Ok(output) = output {
        let status_str = String::from_utf8_lossy(&output.stdout);
        let status = status_str.trim();
        println!("Status: {}", status);
    }

    Ok(())
}

