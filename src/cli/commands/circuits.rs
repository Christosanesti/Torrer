use crate::error::TorrerResult;
use crate::tor::{TorClient, CircuitManager};

/// List active Tor circuits
pub async fn list_circuits() -> TorrerResult<()> {
    println!("Active Tor Circuits:");
    println!();

    let mut client = TorClient::new();
    
    // Connect and authenticate
    client.connect().await?;
    client.authenticate().await?;

    // Get circuits
    match CircuitManager::get_circuits(&mut client).await {
        Ok(circuits) => {
            if circuits.is_empty() {
                println!("No active circuits");
            } else {
                for (i, circuit) in circuits.iter().enumerate() {
                    println!("Circuit {}:", i + 1);
                    println!("  ID: {}", circuit.id);
                    println!("  Status: {}", circuit.status);
                    if let Some(ref purpose) = circuit.purpose {
                        println!("  Purpose: {}", purpose);
                    }
                    if let Some(ref flags) = circuit.flags {
                        println!("  Flags: {}", flags);
                    }
                    println!();
                }
            }
        }
        Err(e) => {
            println!("Failed to get circuits: {}", e);
            return Err(e);
        }
    }

    Ok(())
}

/// Request new circuit
pub async fn new_circuit() -> TorrerResult<()> {
    println!("Requesting new Tor circuit...");

    let mut client = TorClient::new();
    
    // Connect and authenticate
    client.connect().await?;
    client.authenticate().await?;

    // Request new circuit
    CircuitManager::new_circuit(&mut client).await?;

    println!("✓ New circuit requested");
    Ok(())
}

