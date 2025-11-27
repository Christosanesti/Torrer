use crate::error::TorrerResult;

/// Validate system configuration and setup
pub async fn validate() -> TorrerResult<()> {
    println!("Validating Torrer installation...");
    println!();

    let mut all_ok = true;

    // Check Tor daemon
    println!("Checking Tor daemon...");
    if check_tor_daemon().await {
        println!("  ✓ Tor daemon is running");
    } else {
        println!("  ✗ Tor daemon is not running");
        println!("    Run: sudo systemctl start tor");
        all_ok = false;
    }

    // Check iptables
    println!("Checking iptables...");
    if check_iptables() {
        println!("  ✓ iptables is available");
    } else {
        println!("  ✗ iptables is not available");
        all_ok = false;
    }

    // Check configuration
    println!("Checking configuration...");
    match check_configuration() {
        Ok(_) => println!("  ✓ Configuration is valid"),
        Err(e) => {
            println!("  ✗ Configuration error: {}", e);
            all_ok = false;
        }
    }

    // Check bridges
    println!("Checking bridges...");
    match check_bridges() {
        Ok(count) => {
            if count > 0 {
                println!("  ✓ {} bridge(s) configured", count);
            } else {
                println!("  ⚠ No bridges configured (optional)");
            }
        }
        Err(e) => {
            println!("  ✗ Bridge check failed: {}", e);
            all_ok = false;
        }
    }

    println!();
    if all_ok {
        println!("✓ All checks passed!");
        Ok(())
    } else {
        println!("✗ Some checks failed");
        Err(crate::error::TorrerError::Config("Validation failed".to_string()))
    }
}

async fn check_tor_daemon() -> bool {
    use std::process::Command;
    Command::new("systemctl")
        .args(&["is-active", "--quiet", "tor"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn check_iptables() -> bool {
    use std::process::Command;
    Command::new("iptables")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn check_configuration() -> TorrerResult<()> {
    use crate::config::ConfigManager;
    let config_manager = ConfigManager::new()?;
    let _config = config_manager.load()?;
    Ok(())
}

fn check_bridges() -> TorrerResult<usize> {
    use crate::bridge::BridgeManager;
    let bridge_manager = BridgeManager::new()?;
    let bridges = bridge_manager.list_bridges()?;
    Ok(bridges.len())
}

