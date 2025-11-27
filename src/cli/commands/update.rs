use crate::error::{TorrerError, TorrerResult};
use std::process::Command;

const GITHUB_API_URL: &str = "https://api.github.com/repos/yourusername/torrer/releases/latest";
const USER_AGENT: &str = "Torrer/0.1.0";

/// Check for updates
pub async fn check_updates() -> TorrerResult<()> {
    println!("Checking for Torrer updates...");
    println!();

    let current_version = env!("CARGO_PKG_VERSION");
    println!("Current version: {}", current_version);

    // Check GitHub for latest release
    println!("Checking latest version from GitHub...");
    
    match check_github_releases().await {
        Ok(latest_version) => {
            println!("Latest version available: {}", latest_version);
            
            if compare_versions(current_version, &latest_version) {
                println!("✓ You are running the latest version");
            } else {
                println!("⚠ Update available!");
                println!("  Current: {}", current_version);
                println!("  Latest:  {}", latest_version);
                println!();
                println!("To update, run: torrer update");
            }
        }
        Err(e) => {
            log::warn!("Failed to check for updates: {}", e);
            println!("⚠ Could not check for updates");
            println!("  Error: {}", e);
            println!();
            println!("To update manually, pull latest changes and run: sudo ./install.sh");
        }
    }
    
    println!("✓ Update check complete");

    Ok(())
}

/// Update Torrer
pub async fn update() -> TorrerResult<()> {
    println!("Updating Torrer...");
    println!();

    // Check if we're in a git repository
    let is_git_repo = Command::new("git")
        .args(&["rev-parse", "--git-dir"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if !is_git_repo {
        return Err(crate::error::TorrerError::Config(
            "Not a git repository. Cannot update automatically.".to_string()
        ));
    }

    println!("Pulling latest changes...");
    let output = Command::new("git")
        .args(&["pull"])
        .output()
        .map_err(|e| {
            crate::error::TorrerError::Config(format!("Failed to pull updates: {}", e))
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(crate::error::TorrerError::Config(
            format!("Git pull failed: {}", stderr)
        ));
    }

    println!("✓ Updates pulled successfully");
    println!();
    println!("Rebuilding Torrer...");
    println!("Run: cargo build --release && sudo ./install.sh");

    Ok(())
}

/// Check GitHub for latest release version
async fn check_github_releases() -> TorrerResult<String> {
    let client = reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| TorrerError::Config(format!("Failed to create HTTP client: {}", e)))?;

    let response = client
        .get(GITHUB_API_URL)
        .send()
        .await
        .map_err(|e| TorrerError::Config(format!("Failed to fetch release info: {}", e)))?;

    if !response.status().is_success() {
        return Err(TorrerError::Config(format!(
            "GitHub API returned error: {}",
            response.status()
        )));
    }

    let json: serde_json::Value = response
        .json()
        .await
        .map_err(|e| TorrerError::Config(format!("Failed to parse response: {}", e)))?;

    // Extract version from tag_name (remove 'v' prefix if present)
    let version = json
        .get("tag_name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| TorrerError::Config("No tag_name in release response".to_string()))?
        .trim_start_matches('v')
        .to_string();

    Ok(version)
}

/// Compare two version strings
/// Returns true if current >= latest (no update needed)
fn compare_versions(current: &str, latest: &str) -> bool {
    // Simple version comparison (semver-like)
    // This is a basic implementation - for production, use a proper semver library
    let current_parts: Vec<u32> = current
        .split('.')
        .filter_map(|s| s.parse().ok())
        .collect();
    let latest_parts: Vec<u32> = latest
        .split('.')
        .filter_map(|s| s.parse().ok())
        .collect();

    // Compare major, minor, patch
    for i in 0..3 {
        let current_val = current_parts.get(i).copied().unwrap_or(0);
        let latest_val = latest_parts.get(i).copied().unwrap_or(0);

        if current_val > latest_val {
            return true; // Current is newer
        } else if current_val < latest_val {
            return false; // Latest is newer
        }
    }

    true // Versions are equal
}

