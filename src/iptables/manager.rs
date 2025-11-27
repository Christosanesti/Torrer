use std::process::{Command, Stdio};
use std::path::PathBuf;
use std::fs;

use crate::error::{TorrerError, TorrerResult};

const IPTABLES_BACKUP_DIR: &str = "/var/lib/torrer";
const IPTABLES_BACKUP_FILE: &str = "iptables-backup.rules";
const TOR_TRANSPORT_PORT: u16 = 9040;
const TOR_DNS_PORT: u16 = 5353;

/// iptables manager for Tor routing
pub struct IptablesManager {
    backup_path: PathBuf,
}

impl IptablesManager {
    /// Create a new IptablesManager
    pub fn new() -> TorrerResult<Self> {
        let backup_dir = PathBuf::from(IPTABLES_BACKUP_DIR);
        
        // Create backup directory if it doesn't exist
        if !backup_dir.exists() {
            fs::create_dir_all(&backup_dir).map_err(|e| {
                TorrerError::Iptables(format!("Failed to create backup directory: {}", e))
            })?;
        }

        Ok(Self {
            backup_path: backup_dir.join(IPTABLES_BACKUP_FILE),
        })
    }

    /// Backup current iptables rules
    pub fn backup(&self) -> TorrerResult<()> {
        log::info!("Backing up current iptables rules to {:?}", self.backup_path);

        let output = Command::new("iptables-save")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| {
                TorrerError::Iptables(format!("Failed to run iptables-save: {}", e))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(TorrerError::Iptables(format!(
                "iptables-save failed: {}",
                stderr
            )));
        }

        fs::write(&self.backup_path, &output.stdout).map_err(|e| {
            TorrerError::Iptables(format!("Failed to write backup file: {}", e))
        })?;

        log::info!("iptables rules backed up successfully");
        Ok(())
    }

    /// Restore iptables rules from backup
    pub fn restore(&self) -> TorrerResult<()> {
        if !self.backup_path.exists() {
            log::warn!("No backup file found at {:?}, skipping restore", self.backup_path);
            return Ok(());
        }

        log::info!("Restoring iptables rules from {:?}", self.backup_path);

        let backup_content = fs::read_to_string(&self.backup_path).map_err(|e| {
            TorrerError::Iptables(format!("Failed to read backup file: {}", e))
        })?;

        let mut output = Command::new("iptables-restore")
            .stdin(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| {
                TorrerError::Iptables(format!("Failed to run iptables-restore: {}", e))
            })?;

        // Write backup content to stdin
        use std::io::Write;
        if let Some(mut stdin) = output.stdin.take() {
            stdin.write_all(backup_content.as_bytes()).map_err(|e| {
                TorrerError::Iptables(format!("Failed to write to iptables-restore: {}", e))
            })?;
            stdin.flush().map_err(|e| {
                TorrerError::Iptables(format!("Failed to flush iptables-restore: {}", e))
            })?;
            // Drop stdin to close it, signaling EOF to the process
            drop(stdin);
        }

        let status = output.wait().map_err(|e| {
            TorrerError::Iptables(format!("Failed to wait for iptables-restore: {}", e))
        })?;

        if !status.success() {
            return Err(TorrerError::Iptables(format!(
                "iptables-restore failed with exit code: {:?}",
                status.code()
            )));
        }

        log::info!("iptables rules restored successfully");
        Ok(())
    }

    /// Apply Tor routing rules
    pub fn apply_tor_routing(&self) -> TorrerResult<()> {
        log::info!("Applying iptables rules for Tor routing");

        // Flush existing NAT rules (be careful!)
        self.run_iptables(&["-t", "nat", "-F", "OUTPUT"])?;

        // Redirect TCP traffic to Tor TransPort
        self.run_iptables(&[
            "-t", "nat",
            "-A", "OUTPUT",
            "!", "-o", "lo",
            "-p", "tcp",
            "-m", "tcp",
            "--syn",
            "-j", "REDIRECT",
            "--to-ports", &TOR_TRANSPORT_PORT.to_string(),
        ])?;

        // Don't redirect Tor's own traffic (if debian-tor user exists)
        // This may fail on some systems, which is OK
        let _ = self.run_iptables(&[
            "-t", "nat",
            "-A", "OUTPUT",
            "-m", "owner",
            "--uid-owner", "debian-tor",
            "-j", "RETURN",
        ]);

        log::info!("Tor routing rules applied successfully");
        Ok(())
    }

    /// Remove Tor routing rules
    pub fn remove_tor_routing(&self) -> TorrerResult<()> {
        log::info!("Removing Tor routing rules");

        // Try to remove the rules (may fail if they don't exist, which is OK)
        let _ = self.run_iptables(&["-t", "nat", "-D", "OUTPUT", "-p", "tcp", "-m", "tcp", "--syn", "-j", "REDIRECT", "--to-ports", &TOR_TRANSPORT_PORT.to_string()]);
        let _ = self.run_iptables(&["-t", "nat", "-D", "OUTPUT", "-m", "owner", "--uid-owner", "debian-tor", "-j", "RETURN"]);

        log::info!("Tor routing rules removed");
        Ok(())
    }

    /// Run an iptables command
    fn run_iptables(&self, args: &[&str]) -> TorrerResult<()> {
        let output = Command::new("iptables")
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| {
                TorrerError::Iptables(format!("Failed to run iptables: {}", e))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            // Some errors are expected (e.g., rule doesn't exist when removing)
            log::debug!("iptables command failed: {} (args: {:?})", stderr, args);
        }

        Ok(())
    }
}

impl Default for IptablesManager {
    fn default() -> Self {
        Self::new().expect("Failed to create IptablesManager")
    }
}

