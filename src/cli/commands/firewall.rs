use crate::error::TorrerResult;
use crate::security::FirewallManager;

/// Check firewall status
pub fn check_firewall() -> TorrerResult<()> {
    let manager = FirewallManager::new();

    println!("=== Firewall Status ===");
    println!();

    if manager.is_active() {
        println!("Firewall: Active");
        
        if let Some(fw_type) = manager.get_firewall_type() {
            match fw_type {
                crate::security::FirewallType::Ufw => {
                    println!("Type: UFW");
                }
                crate::security::FirewallType::Firewalld => {
                    println!("Type: firewalld");
                }
            }
        }
    } else {
        println!("Firewall: Not active or not detected");
    }

    Ok(())
}

/// Configure firewall for Tor
pub fn configure_firewall() -> TorrerResult<()> {
    println!("Configuring firewall for Tor...");
    
    let manager = FirewallManager::new();
    manager.configure_tor()?;
    
    println!("✓ Firewall configured");
    Ok(())
}

