use clap::{Parser, Subcommand};

mod cli;
mod core;
mod config;
mod security;
mod tor;
mod iptables;
mod bridge;
mod logging;
mod error;
mod utils;
#[cfg(feature = "gui")]
mod gui;

use cli::commands::logs;

use error::TorrerResult;
use logging::logger::init_logger;
use core::TorrerEngine;

#[derive(Parser)]
#[command(name = "torrer")]
#[command(about = "System-wide Tor routing for Ubuntu")]
#[command(version = get_version())]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[derive(Subcommand)]
enum Commands {
    /// Start Tor routing
    Start,
    /// Stop Tor routing
    Stop,
    /// Show routing status
    Status,
    /// Restart Tor routing
    Restart,
    /// Configure Torrer
    Config,
    /// Add a bridge
    AddBridge {
        /// Bridge address (IP:PORT)
        bridge: String,
    },
    /// List bridges
    ListBridges,
    /// Test bridge connectivity
    TestBridge {
        /// Bridge address (IP:PORT)
        bridge: String,
    },
    /// Remove a bridge
    RemoveBridge {
        /// Bridge address
        address: String,
        /// Bridge port
        port: u16,
    },
    /// Collect bridges automatically
    CollectBridges {
        /// Test bridges before caching
        #[arg(short, long)]
        test: bool,
    },
    /// View logs
    Logs {
        /// Follow log output
        #[arg(short, long)]
        follow: bool,
        /// Number of lines to show (tail)
        #[arg(short, long, default_value = "0")]
        tail: usize,
        /// Filter by log level (DEBUG, INFO, WARN, ERROR)
        #[arg(short, long)]
        level: Option<String>,
        /// Output format (text, json)
        #[arg(short, long, default_value = "text")]
        format: String,
    },
    /// Export configuration
    Export {
        /// Export file path
        path: String,
    },
    /// Import configuration
    Import {
        /// Import file path
        path: String,
        /// Merge with existing configuration (partial import)
        #[arg(short, long)]
        partial: bool,
    },
    /// Show statistics
    Stats {
        /// Output format (text, json, csv)
        #[arg(short, long)]
        format: Option<String>,
        /// Export to file
        #[arg(short, long)]
        export: Option<String>,
        /// Monitor in real-time (interval in seconds)
        #[arg(short, long)]
        monitor: Option<u64>,
    },
    /// Set exit node country
    SetCountry {
        /// Country code (e.g., CA, US, DE)
        country: String,
    },
    /// Randomize MAC addresses
    RandomizeMac {
        /// Network interface (optional, randomizes all if not specified)
        interface: Option<String>,
    },
    /// Validate installation and configuration
    Validate,
    /// Perform health check
    Health,
    /// Generate shell completion
    Completion {
        /// Shell type (bash, zsh, fish)
        shell: String,
    },
    /// Run diagnostic tests
    Test,
    /// Collect diagnostic information
    Diagnostics {
        /// Output format (text, json)
        #[arg(short, long, default_value = "text")]
        format: String,
    },
    /// Clean temporary files and caches
    Clean {
        /// What to clean (logs, cache, backups, all)
        what: String,
    },
    /// Show system information
    Info,
    /// Test for DNS and IPv6 leaks
    LeakTest,
    /// List active Tor circuits
    Circuits,
    /// Request new Tor circuit
    NewCircuit,
    /// Show application state
    State,
    /// Save state to file
    SaveState {
        /// State file path
        path: String,
    },
    /// Load state from file
    LoadState {
        /// State file path
        path: String,
    },
    /// Check for updates
    CheckUpdate,
    /// Update Torrer
    Update,
    /// Create a backup
    Backup,
    /// List backups
    ListBackups,
    /// Restore from backup
    RestoreBackup {
        /// Backup file path
        path: String,
    },
    /// Clean old backups
    CleanBackups,
    /// Show help information
    Help {
        /// Command to show help for
        command: Option<String>,
    },
    /// Monitor events
    Monitor,
    /// Show recent events
    Events {
        /// Number of events to show
        #[arg(short, long, default_value = "10")]
        count: usize,
    },
    /// Check firewall status
    CheckFirewall,
    /// Configure firewall for Tor
    ConfigureFirewall,
    /// List scheduled tasks
    ListTasks,
    /// Add scheduled task
    AddTask {
        /// Task name
        name: String,
        /// Interval in seconds
        interval: u64,
    },
    /// Remove scheduled task
    RemoveTask {
        /// Task name
        name: String,
    },
    /// Calculate file checksum
    Checksum {
        /// File path
        path: String,
    },
    /// Verify file checksum
    VerifyChecksum {
        /// File path
        path: String,
        /// Expected checksum
        checksum: String,
    },
    /// Migrate configuration
    Migrate,
    /// List persisted data
    ListData,
    /// Delete persisted data
    DeleteData {
        /// Data key
        key: String,
    },
    /// Get relay information
    RelayInfo {
        /// Relay fingerprint
        fingerprint: String,
    },
    /// Get current exit relay
    ExitRelay,
    /// Install systemd service
    InstallService,
    /// Show service status
    ServiceStatus,
}

#[tokio::main]
async fn main() -> TorrerResult<()> {
    // Initialize logging
    init_logger();
    
    let cli = Cli::parse();
    
    let mut engine = TorrerEngine::new()?;
    
    match cli.command {
        Commands::Start => {
            engine.start().await?;
            println!("✓ Tor routing started successfully");
            Ok(())
        }
        Commands::Stop => {
            match engine.stop().await {
                Ok(_) => {
                    println!("✓ Tor routing stopped successfully");
                    println!("  - Tor routing rules removed");
                    println!("  - DNS configuration restored");
                    println!("  - iptables rules restored");
                    Ok(())
                }
                Err(e) => {
                    println!("⚠ Tor routing stopped with warnings:");
                    println!("  {}", e);
                    println!();
                    println!("Some cleanup operations may have failed.");
                    println!("Run 'torrer status' to verify current state.");
                    Err(e)
                }
            }
        }
        Commands::Status => {
            let status = engine.status().await?;
            
            println!("=== Torrer Status ===");
            println!();
            
            // Routing status
            println!("Routing Status: {}", if status.is_running { "ACTIVE" } else { "INACTIVE" });
            
            if status.is_running {
                println!();
                println!("Connection Details:");
                println!("  Tor Connected: {}", if status.tor_connected { "Yes ✓" } else { "No ✗" });
                println!("  Circuit Established: {}", if status.circuit_established { "Yes ✓" } else { "No ✗" });
                
                // Determine routing method
                let routing_method = if status.tor_connected {
                    "Tor"
                } else {
                    "Unknown"
                };
                println!("  Routing Method: {}", routing_method);
                
                // Get additional state information
                let state_manager = crate::core::state::StateManager::new();
                let state = state_manager.get_state();
                if let Some(start_time) = state.start_time {
                    use crate::utils::{format_timestamp, elapsed_since, format_duration};
                    println!();
                    println!("Runtime Information:");
                    println!("  Started: {}", format_timestamp(start_time));
                    let elapsed = elapsed_since(start_time);
                    println!("  Uptime: {}", format_duration(elapsed));
                    println!("  Connections: {}", state.connection_count);
                    if state.fallback_count > 0 {
                        println!("  Fallbacks: {}", state.fallback_count);
                    }
                    if let Some(ref country) = state.current_country {
                        println!("  Exit Country: {}", country);
                    }
                }
                
                // Health check summary
                println!();
                println!("Health Check:");
                if let Ok(health) = crate::core::health::HealthChecker::check_all().await {
                    println!("  Tor Daemon: {}", if health.tor_daemon { "Running ✓" } else { "Stopped ✗" });
                    println!("  Tor Control: {}", if health.tor_control { "Connected ✓" } else { "Disconnected ✗" });
                    println!("  Tor Circuit: {}", if health.tor_circuit { "Established ✓" } else { "Not Established ✗" });
                    println!("  iptables: {}", if health.iptables { "Available ✓" } else { "Unavailable ✗" });
                    println!("  DNS: {}", if health.dns { "Configured ✓" } else { "Not Configured ✗" });
                    
                    let health_score = health.score();
                    println!();
                    println!("Overall Health: {}%", health_score);
                    if health_score == 100 {
                        println!("  Status: All systems operational ✓");
                    } else if health_score >= 80 {
                        println!("  Status: Mostly operational ⚠");
                    } else {
                        println!("  Status: Issues detected ✗");
                    }
                }
            } else {
                println!();
                println!("To start Tor routing, use:");
                println!("  sudo torrer start");
            }
            
            Ok(())
        }
        Commands::Restart => {
            println!("Restarting Tor routing...");
            println!();
            
            match engine.restart().await {
                Ok(_) => {
                    println!("✓ Tor routing restarted successfully");
                    println!("  - Routing stopped");
                    println!("  - Routing started");
                    
                    // Show status
                    let status = engine.status().await?;
                    if status.is_running {
                        println!("  - Status: ACTIVE");
                        if status.circuit_established {
                            println!("  - Circuit: Established");
                        } else {
                            println!("  - Circuit: Establishing...");
                        }
                    }
                    Ok(())
                }
                Err(e) => {
                    println!("✗ Tor routing restart failed: {}", e);
                    println!();
                    println!("Current status:");
                    match engine.status().await {
                        Ok(status) => {
                            println!("  - Routing: {}", if status.is_running { "ACTIVE" } else { "INACTIVE" });
                            println!("  - Tor connected: {}", if status.tor_connected { "Yes" } else { "No" });
                        }
                        Err(_) => {
                            println!("  - Status: Unknown");
                        }
                    }
                    Err(e)
                }
            }
        }
        Commands::Config => {
            use crate::config::ConfigManager;
            let config_manager = ConfigManager::new()?;
            config_manager.interactive_config()?;
            Ok(())
        }
        Commands::AddBridge { bridge } => {
            use crate::bridge::{BridgeManager, Bridge};
            let bridge_manager = BridgeManager::new()?;
            
            // Parse and validate bridge
            let bridge = Bridge::from_str(&bridge)
                .map_err(|e| error::TorrerError::Bridge(format!("Invalid bridge format: {}", e)))?;
            
            // Additional validation
            bridge.validate()
                .map_err(|e| error::TorrerError::Bridge(format!("Bridge validation failed: {}", e)))?;
            
            println!("Adding bridge {}:{}...", bridge.address, bridge.port);
            bridge_manager.add_bridge(bridge.clone())?;
            println!("✓ Bridge {}:{} added successfully", bridge.address, bridge.port);
            
            // Show Tor config format
            println!("  Tor config format: {}", bridge.to_tor_config());
            Ok(())
        }
        Commands::ListBridges => {
            use crate::bridge::BridgeManager;
            let bridge_manager = BridgeManager::new()?;
            let bridges = bridge_manager.list_bridges()?;
            
            if bridges.is_empty() {
                println!("No bridges configured");
                println!();
                println!("To add a bridge, use:");
                println!("  torrer add-bridge <address>:<port>");
                println!();
                println!("Example:");
                println!("  torrer add-bridge 1.2.3.4:443");
            } else {
                println!("Configured bridges ({}):", bridges.len());
                println!();
                for (index, bridge) in bridges.iter().enumerate() {
                    print!("  {}. {}:{}", index + 1, bridge.address, bridge.port);
                    if let Some(ref fp) = bridge.fingerprint {
                        print!(" [{}]", fp);
                    }
                    if let Some(ref transport) = bridge.transport {
                        print!(" ({})", transport);
                    }
                    println!();
                }
                println!();
                println!("To test a bridge, use:");
                println!("  torrer test-bridge <address>:<port>");
            }
            Ok(())
        }
        Commands::TestBridge { bridge } => {
            use crate::bridge::{BridgeManager, Bridge};
            let bridge_manager = BridgeManager::new()?;
            
            // Parse and validate bridge
            let bridge = Bridge::from_str(&bridge)
                .map_err(|e| error::TorrerError::Bridge(format!("Invalid bridge format: {}", e)))?;
            
            println!("Testing bridge {}:{}...", bridge.address, bridge.port);
            print!("Connecting... ");
            use std::io::Write;
            std::io::stdout().flush().unwrap();
            
            let is_reachable = bridge_manager.test_bridge(&bridge).await?;
            
            if is_reachable {
                println!("\r✓ Bridge {}:{} is reachable", bridge.address, bridge.port);
                println!("  The bridge appears to be working and can be used for routing.");
            } else {
                println!("\r✗ Bridge {}:{} is not reachable", bridge.address, bridge.port);
                println!("  The bridge may be down, blocked, or unreachable.");
                println!("  Check your network connection and firewall settings.");
            }
            Ok(())
        }
        Commands::RemoveBridge { address, port } => {
            use crate::bridge::BridgeManager;
            let bridge_manager = BridgeManager::new()?;
            
            println!("Removing bridge {}:{}...", address, port);
            bridge_manager.remove_bridge(&address, port)?;
            println!("✓ Bridge {}:{} removed successfully", address, port);
            Ok(())
        }
        Commands::CollectBridges { test } => {
            use crate::bridge::collector::BridgeCollector;
            println!("Collecting bridges...");
            let mut collector = BridgeCollector::new()?;
            
            if test {
                println!("Testing bridges before caching...");
                let successful = collector.collect_and_test().await?;
                println!("✓ Collected and tested bridges: {} successful", successful);
            } else {
                collector.collect_and_cache().await?;
                println!("✓ Bridges collected and cached");
            }
            
            // Show prioritized bridges
            let prioritized = collector.get_prioritized_bridges();
            if !prioritized.is_empty() {
                println!("\nPrioritized bridges (by success rate):");
                for (key, score) in prioritized.iter().take(10) {
                    println!("  {}: {:.2}%", key, score);
                }
            }
            
            Ok(())
        }
        Commands::Logs { follow, tail, level, format } => {
            logs::view_logs(follow, tail, level.as_deref(), &format)?;
            Ok(())
        }
        Commands::Export { path } => {
            use crate::config::ConfigManager;
            let config_manager = ConfigManager::new()?;
            config_manager.export(&path)?;
            println!("✓ Configuration exported to {}", path);
            Ok(())
        }
        Commands::Import { path, partial } => {
            use crate::config::ConfigManager;
            let config_manager = ConfigManager::new()?;
            if partial {
                config_manager.import_partial(&path)?;
                println!("✓ Configuration partially imported and merged from {}", path);
            } else {
                config_manager.import(&path)?;
                println!("✓ Configuration imported from {}", path);
            }
            Ok(())
        }
        Commands::Stats { format, export, monitor } => {
            use cli::commands::stats;
            if let Some(interval) = monitor {
                stats::monitor_stats(interval).await?;
            } else if let Some(export_path) = export {
                let format_str = format.as_deref().unwrap_or("json");
                stats::export_stats(&export_path, format_str).await?;
            } else {
                stats::show_stats(format.as_deref()).await?;
            }
            Ok(())
        }
        Commands::SetCountry { country } => {
            use crate::tor::{TorClient, CountrySelector};
            let mut client = TorClient::new();
            client.connect().await?;
            client.authenticate().await?;
            let selector = CountrySelector::new(Some(country.clone()));
            
            // Validate country code(s) before setting
            let validated = if country.contains(',') {
                let codes = CountrySelector::validate_country_codes(&country)?;
                selector.set_exit_country(&mut client, &country).await?;
                println!("✓ Exit node countries set to: {}", codes.join(", "));
                codes.join(",")
            } else {
                CountrySelector::validate_country_code(&country)?;
                selector.set_exit_country(&mut client, &country).await?;
                println!("✓ Exit node country set to: {}", country.to_uppercase());
                country.to_uppercase()
            };
            
            // Update configuration if possible
            if let Ok(mut config_manager) = crate::config::ConfigManager::new() {
                let mut config = config_manager.load().unwrap_or_default();
                config.country_code = Some(validated);
                if let Err(e) = config_manager.save(&config) {
                    log::warn!("Failed to save country to config: {}", e);
                }
            }
            
            Ok(())
        }
        Commands::RandomizeMac { interface } => {
            use crate::security::MacManager;
            let mac_manager = MacManager::new();
            if let Some(iface) = interface {
                mac_manager.randomize_mac(&iface)?;
                println!("✓ MAC address randomized for interface: {}", iface);
            } else {
                mac_manager.randomize_all()?;
                println!("✓ MAC addresses randomized for all interfaces");
            }
            Ok(())
        }
        Commands::Validate => {
            use cli::commands::validate;
            validate::validate().await?;
            Ok(())
        }
        Commands::Health => {
            use crate::core::HealthChecker;
            let status = HealthChecker::check_all().await?;
            println!("Torrer Health Check:");
            println!("  Tor daemon: {}", if status.tor_daemon { "✓" } else { "✗" });
            println!("  Tor control: {}", if status.tor_control { "✓" } else { "✗" });
            println!("  Tor circuit: {}", if status.tor_circuit { "✓" } else { "✗" });
            println!("  iptables: {}", if status.iptables { "✓" } else { "✗" });
            println!("  DNS: {}", if status.dns { "✓" } else { "✗" });
            println!("  Health score: {}/100", status.score());
            if status.is_healthy() {
                println!("  Status: ✓ All systems healthy");
            } else {
                println!("  Status: ✗ Some systems unhealthy");
            }
            Ok(())
        }
        Commands::Completion { shell } => {
            use cli::commands::completion;
            completion::generate_completion(&shell)?;
            Ok(())
        }
        Commands::Test => {
            use cli::commands::test;
            test::run_tests().await?;
            Ok(())
        }
        Commands::Diagnostics { format } => {
            use crate::core::Diagnostics;
            let info = Diagnostics::collect_all().await?;
            match format.as_str() {
                "json" => {
                    println!("{}", info.to_json().unwrap_or_else(|_| "Error serializing".to_string()));
                }
                _ => {
                    print!("{}", info.to_text());
                }
            }
            Ok(())
        }
        Commands::Clean { what } => {
            use cli::commands::clean;
            clean::clean(&what)?;
            Ok(())
        }
        Commands::Info => {
            use cli::commands::info;
            info::show_info().await?;
            Ok(())
        }
        Commands::LeakTest => {
            use cli::commands::leak_test;
            leak_test::run_leak_tests().await?;
            Ok(())
        }
        Commands::Circuits => {
            use cli::commands::circuits;
            circuits::list_circuits().await?;
            Ok(())
        }
        Commands::NewCircuit => {
            use cli::commands::circuits;
            circuits::new_circuit().await?;
            Ok(())
        }
        Commands::State => {
            use cli::commands::state;
            state::show_state()?;
            Ok(())
        }
        Commands::SaveState { path } => {
            use cli::commands::state;
            state::save_state(&path)?;
            Ok(())
        }
        Commands::LoadState { path } => {
            use cli::commands::state;
            state::load_state(&path)?;
            Ok(())
        }
        Commands::CheckUpdate => {
            use cli::commands::update;
            update::check_updates().await?;
            Ok(())
        }
        Commands::Update => {
            use cli::commands::update;
            update::update().await?;
            Ok(())
        }
        Commands::Backup => {
            use cli::commands::backup;
            backup::create_backup()?;
            Ok(())
        }
        Commands::ListBackups => {
            use cli::commands::backup;
            backup::list_backups()?;
            Ok(())
        }
        Commands::RestoreBackup { path } => {
            use cli::commands::backup;
            backup::restore_backup(&path)?;
            Ok(())
        }
        Commands::CleanBackups => {
            use cli::commands::backup;
            backup::clean_backups()?;
            Ok(())
        }
        Commands::Help { command } => {
            use cli::commands::help;
            help::show_help(command.as_deref())?;
            Ok(())
        }
        Commands::Monitor => {
            use cli::commands::events;
            events::monitor_events()?;
            Ok(())
        }
        Commands::Events { count } => {
            use cli::commands::events;
            events::show_events(count)?;
            Ok(())
        }
        Commands::CheckFirewall => {
            use cli::commands::firewall;
            firewall::check_firewall()?;
            Ok(())
        }
        Commands::ConfigureFirewall => {
            use cli::commands::firewall;
            firewall::configure_firewall()?;
            Ok(())
        }
        Commands::ListTasks => {
            use cli::commands::schedule;
            schedule::list_tasks()?;
            Ok(())
        }
        Commands::AddTask { name, interval } => {
            use cli::commands::schedule;
            schedule::add_task(&name, interval)?;
            Ok(())
        }
        Commands::RemoveTask { name } => {
            use cli::commands::schedule;
            schedule::remove_task(&name)?;
            Ok(())
        }
        Commands::Checksum { path } => {
            use cli::commands::checksum;
            checksum::calculate_checksum(&path)?;
            Ok(())
        }
        Commands::VerifyChecksum { path, checksum } => {
            use cli::commands::checksum;
            checksum::verify_checksum(&path, &checksum)?;
            Ok(())
        }
        Commands::Migrate => {
            use cli::commands::migrate;
            migrate::migrate_config()?;
            Ok(())
        }
        Commands::ListData => {
            use cli::commands::persist;
            persist::list_data()?;
            Ok(())
        }
        Commands::DeleteData { key } => {
            use cli::commands::persist;
            persist::delete_data(&key)?;
            Ok(())
        }
        Commands::RelayInfo { fingerprint } => {
            use cli::commands::relay;
            relay::get_relay_info(&fingerprint).await?;
            Ok(())
        }
        Commands::ExitRelay => {
            use cli::commands::relay;
            relay::get_exit_relay().await?;
            Ok(())
        }
        Commands::InstallService => {
            use cli::commands::daemon;
            daemon::install_service()?;
            Ok(())
        }
        Commands::ServiceStatus => {
            use cli::commands::daemon;
            daemon::service_status()?;
            Ok(())
        }
    }
}

