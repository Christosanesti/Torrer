use crate::error::{TorrerError, TorrerResult};

/// Cryptographic utilities
pub struct Crypto;

impl Crypto {
    /// Generate random bytes
    pub fn random_bytes(count: usize) -> Vec<u8> {
        use rand::RngCore;
        let mut bytes = vec![0u8; count];
        rand::thread_rng().fill_bytes(&mut bytes);
        bytes
    }

    /// Generate random hex string
    pub fn random_hex(count: usize) -> String {
        let bytes = Self::random_bytes(count);
        hex::encode(&bytes)
    }

    /// Hash data using SHA256
    pub fn sha256(data: &[u8]) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    /// Verify file checksum
    pub fn verify_checksum(data: &[u8], expected: &str) -> bool {
        let actual = Self::sha256(data);
        actual == expected
    }
}

