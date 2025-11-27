use crate::error::TorrerResult;
use crate::security::LeakDetector;

/// Run leak detection tests
pub async fn run_leak_tests() -> TorrerResult<()> {
    println!("Running leak detection tests...");
    println!();

    let detector = LeakDetector::new();

    // DNS leak test
    println!("Testing DNS leaks...");
    match detector.test_dns_leak().await {
        Ok(result) => {
            println!("  Tor DNS: {}", if result.tor_dns_working { "✓ Working" } else { "✗ Not working" });
            println!("  Direct DNS: {}", if result.direct_dns_blocked { "✓ Blocked" } else { "✗ Not blocked" });
            if result.leak_detected {
                println!("  ⚠ DNS LEAK DETECTED!");
            } else {
                println!("  ✓ No DNS leaks detected");
            }
        }
        Err(e) => {
            println!("  ✗ DNS leak test failed: {}", e);
        }
    }

    println!();

    // IPv6 leak test
    println!("Testing IPv6 leaks...");
    match detector.test_ipv6_leak().await {
        Ok(leak_detected) => {
            if leak_detected {
                println!("  ⚠ IPv6 LEAK DETECTED!");
            } else {
                println!("  ✓ No IPv6 leaks detected");
            }
        }
        Err(e) => {
            println!("  ✗ IPv6 leak test failed: {}", e);
        }
    }

    println!();
    println!("Leak detection tests complete");

    Ok(())
}

