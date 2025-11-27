use std::path::PathBuf;
use std::fs;

use crate::error::{TorrerError, TorrerResult};
use crate::config::{Configuration, validate_config};

const CONFIG_DIR: &str = "/etc/torrer";
const CONFIG_FILE: &str = "config.toml";

/// Configuration manager
pub struct ConfigManager {
    config_path: PathBuf,
}

impl ConfigManager {
    /// Create a new ConfigManager
    pub fn new() -> TorrerResult<Self> {
        let config_dir = PathBuf::from(CONFIG_DIR);
        
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir).map_err(|e| {
                TorrerError::Config(format!("Failed to create config directory: {}", e))
            })?;
        }

        Ok(Self {
            config_path: config_dir.join(CONFIG_FILE),
        })
    }

    /// Load configuration
    pub fn load(&self) -> TorrerResult<Configuration> {
        if !self.config_path.exists() {
            log::info!("Config file not found, using defaults");
            return Ok(Configuration::default());
        }

        let content = fs::read_to_string(&self.config_path).map_err(|e| {
            TorrerError::Config(format!("Failed to read config file: {}", e))
        })?;

        let config: Configuration = toml::from_str(&content).map_err(|e| {
            TorrerError::Config(format!("Failed to parse config file: {}", e))
        })?;

        // Validate configuration
        validate_config(&config)?;

        Ok(config)
    }

    /// Export configuration to file
    pub fn export(&self, export_path: &str) -> TorrerResult<()> {
        let config = self.load()?;
        let content = toml::to_string_pretty(&config).map_err(|e| {
            TorrerError::Config(format!("Failed to serialize config: {}", e))
        })?;

        fs::write(export_path, content).map_err(|e| {
            TorrerError::Config(format!("Failed to write export file: {}", e))
        })?;

        log::info!("Configuration exported to {}", export_path);
        Ok(())
    }

    /// Import configuration from file with backup
    pub fn import(&self, import_path: &str) -> TorrerResult<()> {
        // Backup existing configuration before import
        let backup_path = self.backup_config()?;
        log::info!("Backed up existing configuration to: {:?}", backup_path);

        // Read import file
        let content = fs::read_to_string(import_path).map_err(|e| {
            TorrerError::Config(format!("Failed to read import file: {} - {}", import_path, e))
        })?;

        // Parse imported configuration
        let imported_config: Configuration = toml::from_str(&content).map_err(|e| {
            // Restore backup on parse error
            let _ = self.restore_backup(&backup_path);
            TorrerError::Config(format!("Failed to parse import file: {} - {}", import_path, e))
        })?;

        // Validate imported configuration
        validate_config(&imported_config).map_err(|e| {
            // Restore backup on validation error
            let _ = self.restore_backup(&backup_path);
            TorrerError::Config(format!("Invalid configuration in import file: {}", e))
        })?;

        // Save imported configuration
        match self.save(&imported_config) {
            Ok(_) => {
                log::info!("Configuration imported successfully from {}", import_path);
                Ok(())
            }
            Err(e) => {
                // Restore backup on save error
                let _ = self.restore_backup(&backup_path);
                Err(e)
            }
        }
    }

    /// Import configuration with partial merge (merge with existing config)
    pub fn import_partial(&self, import_path: &str) -> TorrerResult<()> {
        // Backup existing configuration
        let backup_path = self.backup_config()?;
        log::info!("Backed up existing configuration to: {:?}", backup_path);

        // Load existing configuration
        let mut existing_config = self.load().unwrap_or_default();

        // Read import file
        let content = fs::read_to_string(import_path).map_err(|e| {
            TorrerError::Config(format!("Failed to read import file: {} - {}", import_path, e))
        })?;

        // Parse imported configuration (may be partial)
        let imported_config: Configuration = toml::from_str(&content).map_err(|e| {
            TorrerError::Config(format!("Failed to parse import file: {} - {}", import_path, e))
        })?;

        // Merge configurations (imported values override existing)
        existing_config.tor_control_port = imported_config.tor_control_port;
        existing_config.tor_transport_port = imported_config.tor_transport_port;
        existing_config.tor_dns_port = imported_config.tor_dns_port;
        existing_config.ipv6_enabled = imported_config.ipv6_enabled;
        existing_config.auto_fallback = imported_config.auto_fallback;
        if imported_config.country_code.is_some() {
            existing_config.country_code = imported_config.country_code;
        }

        // Validate merged configuration
        validate_config(&existing_config).map_err(|e| {
            let _ = self.restore_backup(&backup_path);
            TorrerError::Config(format!("Invalid merged configuration: {}", e))
        })?;

        // Save merged configuration
        match self.save(&existing_config) {
            Ok(_) => {
                log::info!("Configuration partially imported and merged from {}", import_path);
                Ok(())
            }
            Err(e) => {
                let _ = self.restore_backup(&backup_path);
                Err(e)
            }
        }
    }

    /// Backup current configuration
    fn backup_config(&self) -> TorrerResult<PathBuf> {
        if !self.config_path.exists() {
            // No config to backup
            return Ok(self.config_path.clone());
        }

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let backup_path = self.config_path.with_extension(&format!("toml.backup.{}", timestamp));
        
        fs::copy(&self.config_path, &backup_path).map_err(|e| {
            TorrerError::Config(format!("Failed to backup configuration: {}", e))
        })?;

        log::debug!("Configuration backed up to: {:?}", backup_path);
        Ok(backup_path)
    }

    /// Restore configuration from backup
    fn restore_backup(&self, backup_path: &PathBuf) -> TorrerResult<()> {
        if !backup_path.exists() {
            return Err(TorrerError::Config(
                format!("Backup file not found: {:?}", backup_path)
            ));
        }

        fs::copy(backup_path, &self.config_path).map_err(|e| {
            TorrerError::Config(format!("Failed to restore backup: {}", e))
        })?;

        log::info!("Configuration restored from backup: {:?}", backup_path);
        Ok(())
    }

    /// Save configuration
    pub fn save(&self, config: &Configuration) -> TorrerResult<()> {
        // Validate before saving
        validate_config(config)?;

        let content = toml::to_string_pretty(config).map_err(|e| {
            TorrerError::Config(format!("Failed to serialize config: {}", e))
        })?;

        fs::write(&self.config_path, content).map_err(|e| {
            TorrerError::Config(format!("Failed to write config file: {}", e))
        })?;

        log::info!("Configuration saved to {:?}", self.config_path);
        Ok(())
    }

    /// Run interactive configuration wizard
    pub fn interactive_config(&self) -> TorrerResult<Configuration> {
        use std::io::{self, Write};

        println!("=== Torrer Configuration Wizard ===");
        println!();
        println!("This wizard will help you configure Torrer.");
        println!("Press Enter to use default values (shown in brackets).");
        println!();

        let mut config = self.load().unwrap_or_default();

        // Show current configuration
        println!("Current configuration:");
        println!("  Tor Control Port: {}", config.tor_control_port);
        println!("  Tor Transport Port: {}", config.tor_transport_port);
        println!("  Tor DNS Port: {}", config.tor_dns_port);
        println!("  IPv6 Enabled: {}", config.ipv6_enabled);
        println!("  Auto Fallback: {}", config.auto_fallback);
        if let Some(ref country) = config.country_code {
            println!("  Exit Country: {}", country);
        } else {
            println!("  Exit Country: Any");
        }
        println!();

        // Tor Control Port
        print!("Tor Control Port [{}]: ", config.tor_control_port);
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let port_input = input.trim();
        if !port_input.is_empty() {
            match port_input.parse::<u16>() {
                Ok(port) if port > 0 && port <= 65535 => {
                    config.tor_control_port = port;
                }
                _ => {
                    println!("Invalid port, keeping default: {}", config.tor_control_port);
                }
            }
        }

        // Tor Transport Port
        print!("Tor Transport Port [{}]: ", config.tor_transport_port);
        io::stdout().flush()?;
        input.clear();
        io::stdin().read_line(&mut input)?;
        let port_input = input.trim();
        if !port_input.is_empty() {
            match port_input.parse::<u16>() {
                Ok(port) if port > 0 && port <= 65535 => {
                    config.tor_transport_port = port;
                }
                _ => {
                    println!("Invalid port, keeping default: {}", config.tor_transport_port);
                }
            }
        }

        // Tor DNS Port
        print!("Tor DNS Port [{}]: ", config.tor_dns_port);
        io::stdout().flush()?;
        input.clear();
        io::stdin().read_line(&mut input)?;
        let port_input = input.trim();
        if !port_input.is_empty() {
            match port_input.parse::<u16>() {
                Ok(port) if port > 0 && port <= 65535 => {
                    config.tor_dns_port = port;
                }
                _ => {
                    println!("Invalid port, keeping default: {}", config.tor_dns_port);
                }
            }
        }

        // IPv6 configuration
        print!("Enable IPv6? [{}] (y/N): ", if config.ipv6_enabled { "Y" } else { "N" });
        io::stdout().flush()?;
        input.clear();
        io::stdin().read_line(&mut input)?;
        let ipv6_input = input.trim().to_lowercase();
        if !ipv6_input.is_empty() {
            config.ipv6_enabled = ipv6_input == "y" || ipv6_input == "yes";
        }

        // Auto-fallback configuration
        print!("Enable automatic fallback to bridges? [{}] (Y/n): ", if config.auto_fallback { "Y" } else { "N" });
        io::stdout().flush()?;
        input.clear();
        io::stdin().read_line(&mut input)?;
        let fallback_input = input.trim().to_lowercase();
        if !fallback_input.is_empty() {
            config.auto_fallback = fallback_input != "n" && fallback_input != "no";
        }

        // Country code
        let current_country = config.country_code.as_deref().unwrap_or("Any");
        print!("Exit node country code [{}] (optional, e.g., CA, US, DE, or empty for any): ", current_country);
        io::stdout().flush()?;
        input.clear();
        io::stdin().read_line(&mut input)?;
        let country = input.trim();
        if country.is_empty() {
            config.country_code = None;
        } else if country.len() == 2 && country.chars().all(|c| c.is_ascii_alphabetic()) {
            config.country_code = Some(country.to_uppercase());
        } else {
            println!("Invalid country code format (must be 2 letters), keeping: {}", current_country);
        }

        // Validate configuration
        println!();
        println!("Validating configuration...");
        match validate_config(&config) {
            Ok(_) => {
                println!("✓ Configuration is valid");
            }
            Err(e) => {
                println!("✗ Configuration validation failed: {}", e);
                return Err(e);
            }
        }

        // Save configuration
        println!();
        print!("Save configuration? (Y/n): ");
        io::stdout().flush()?;
        input.clear();
        io::stdin().read_line(&mut input)?;
        let save_input = input.trim().to_lowercase();
        
        if save_input.is_empty() || save_input != "n" && save_input != "no" {
            self.save(&config)?;
            println!("✓ Configuration saved to {:?}", self.config_path);
        } else {
            println!("Configuration not saved.");
        }

        Ok(config)
    }
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self::new().expect("Failed to create ConfigManager")
    }
}

