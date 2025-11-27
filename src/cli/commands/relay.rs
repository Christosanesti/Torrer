use crate::error::TorrerResult;
use crate::tor::{TorClient, RelayManager};

/// Get relay information
pub async fn get_relay_info(fingerprint: &str) -> TorrerResult<()> {
    println!("Fetching relay information for: {}", fingerprint);

    let mut client = TorClient::new();
    client.connect().await?;
    client.authenticate().await?;

    match RelayManager::get_relay_info(&mut client, fingerprint).await {
        Ok(relay) => {
            println!("Relay Information:");
            println!("  Fingerprint: {}", relay.fingerprint);
            if let Some(ref nickname) = relay.nickname {
                println!("  Nickname: {}", nickname);
            }
            if let Some(ref address) = relay.address {
                println!("  Address: {}", address);
            }
            if let Some(ref country) = relay.country {
                println!("  Country: {}", country);
            }
            println!("  Is Exit: {}", relay.is_exit);
            println!("  Is Guard: {}", relay.is_guard);
            Ok(())
        }
        Err(e) => {
            println!("Failed to get relay info: {}", e);
            Err(e)
        }
    }
}

/// Get current exit relay
pub async fn get_exit_relay() -> TorrerResult<()> {
    println!("Fetching current exit relay information...");

    let mut client = TorClient::new();
    client.connect().await?;
    client.authenticate().await?;

    match RelayManager::get_exit_relay(&mut client).await {
        Ok(Some(relay)) => {
            println!("Current Exit Relay:");
            println!("  Fingerprint: {}", relay.fingerprint);
            if let Some(ref nickname) = relay.nickname {
                println!("  Nickname: {}", nickname);
            }
            if let Some(ref country) = relay.country {
                println!("  Country: {}", country);
            }
            Ok(())
        }
        Ok(None) => {
            println!("No exit relay information available");
            Ok(())
        }
        Err(e) => {
            println!("Failed to get exit relay: {}", e);
            Err(e)
        }
    }
}

