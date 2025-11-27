use crate::error::TorrerResult;
use crate::utils::Crypto;
use std::fs;

/// Calculate file checksum
pub fn calculate_checksum(path: &str) -> TorrerResult<()> {
    let data = fs::read(path).map_err(|e| {
        crate::error::TorrerError::Config(format!("Failed to read file: {}", e))
    })?;

    let checksum = Crypto::sha256(&data);
    println!("SHA256: {}", checksum);
    println!("File: {}", path);

    Ok(())
}

/// Verify file checksum
pub fn verify_checksum(path: &str, expected: &str) -> TorrerResult<()> {
    let data = fs::read(path).map_err(|e| {
        crate::error::TorrerError::Config(format!("Failed to read file: {}", e))
    })?;

    if Crypto::verify_checksum(&data, expected) {
        println!("✓ Checksum verified");
        Ok(())
    } else {
        println!("✗ Checksum mismatch");
        Err(crate::error::TorrerError::Config("Checksum verification failed".to_string()))
    }
}

