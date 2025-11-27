use crate::error::TorrerResult;
use std::process::{Command, Stdio};
use std::io::Write;

/// Daemon management for Torrer
pub struct DaemonManager;

impl DaemonManager {
    /// Create systemd service file
    pub fn create_service_file() -> TorrerResult<String> {
        let service_content = r#"[Unit]
Description=Torrer - System-wide Tor routing
After=network.target tor.service
Requires=tor.service

[Service]
Type=oneshot
RemainAfterExit=yes
ExecStart=/usr/local/bin/torrer start
ExecStop=/usr/local/bin/torrer stop
ExecReload=/usr/local/bin/torrer restart
Restart=on-failure
RestartSec=5

[Install]
WantedBy=multi-user.target
"#;

        Ok(service_content.to_string())
    }

    /// Install systemd service
    pub fn install_service() -> TorrerResult<()> {
        let service_content = Self::create_service_file()?;
        let service_path = "/etc/systemd/system/torrer.service";

        // Write service file
        let mut file = std::fs::File::create(service_path).map_err(|e| {
            crate::error::TorrerError::Config(format!("Failed to create service file: {}", e))
        })?;

        file.write_all(service_content.as_bytes()).map_err(|e| {
            crate::error::TorrerError::Config(format!("Failed to write service file: {}", e))
        })?;

        // Reload systemd
        Command::new("systemctl")
            .args(&["daemon-reload"])
            .output()
            .map_err(|e| {
                crate::error::TorrerError::Config(format!("Failed to reload systemd: {}", e))
            })?;

        log::info!("Systemd service installed");
        Ok(())
    }

    /// Enable service
    pub fn enable_service() -> TorrerResult<()> {
        Command::new("systemctl")
            .args(&["enable", "torrer"])
            .output()
            .map_err(|e| {
                crate::error::TorrerError::Config(format!("Failed to enable service: {}", e))
            })?;

        log::info!("Systemd service enabled");
        Ok(())
    }

    /// Check if service is installed
    pub fn is_service_installed() -> bool {
        std::path::Path::new("/etc/systemd/system/torrer.service").exists()
    }

    /// Check if service is enabled
    pub fn is_service_enabled() -> bool {
        Command::new("systemctl")
            .args(&["is-enabled", "torrer"])
            .output()
            .map(|o| {
                String::from_utf8_lossy(&o.stdout).trim() == "enabled"
            })
            .unwrap_or(false)
    }
}

