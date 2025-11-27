use crate::error::TorrerResult;

/// Run diagnostic tests
pub async fn run_tests() -> TorrerResult<()> {
    println!("Running Torrer diagnostic tests...");
    println!();

    let mut passed = 0;
    let mut failed = 0;

    // Test Tor daemon
    print!("Testing Tor daemon... ");
    if test_tor_daemon().await {
        println!("✓ PASSED");
        passed += 1;
    } else {
        println!("✗ FAILED");
        failed += 1;
    }

    // Test Tor control port
    print!("Testing Tor control port... ");
    if test_tor_control().await {
        println!("✓ PASSED");
        passed += 1;
    } else {
        println!("✗ FAILED");
        failed += 1;
    }

    // Test iptables
    print!("Testing iptables... ");
    if test_iptables() {
        println!("✓ PASSED");
        passed += 1;
    } else {
        println!("✗ FAILED");
        failed += 1;
    }

    // Test DNS
    print!("Testing DNS configuration... ");
    if test_dns().await {
        println!("✓ PASSED");
        passed += 1;
    } else {
        println!("✗ FAILED");
        failed += 1;
    }

    // Test configuration
    print!("Testing configuration... ");
    match test_configuration() {
        Ok(_) => {
            println!("✓ PASSED");
            passed += 1;
        }
        Err(_) => {
            println!("✗ FAILED");
            failed += 1;
        }
    }

    println!();
    println!("Results: {} passed, {} failed", passed, failed);

    if failed == 0 {
        println!("✓ All tests passed!");
        Ok(())
    } else {
        println!("✗ Some tests failed");
        Err(crate::error::TorrerError::Config("Tests failed".to_string()))
    }
}

async fn test_tor_daemon() -> bool {
    use std::process::Command;
    Command::new("systemctl")
        .args(&["is-active", "--quiet", "tor"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

async fn test_tor_control() -> bool {
    use crate::tor::TorClient;
    let mut client = TorClient::new();
    client.connect().await.is_ok()
}

fn test_iptables() -> bool {
    use std::process::Command;
    Command::new("iptables")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

async fn test_dns() -> bool {
    use std::process::Command;
    use tokio::time::{timeout, Duration};

    let result = timeout(
        Duration::from_secs(5),
        tokio::process::Command::new("dig")
            .args(&["@127.0.0.1", "-p", "5353", "example.com", "+short"])
            .output(),
    )
    .await;

    match result {
        Ok(Ok(output)) => output.status.success(),
        _ => false,
    }
}

fn test_configuration() -> TorrerResult<()> {
    use crate::config::ConfigManager;
    let config_manager = ConfigManager::new()?;
    let _config = config_manager.load()?;
    Ok(())
}

