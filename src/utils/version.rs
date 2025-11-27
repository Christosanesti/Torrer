/// Version comparison utilities
pub struct Version;

impl Version {
    /// Compare two version strings
    pub fn compare(v1: &str, v2: &str) -> Option<std::cmp::Ordering> {
        let v1_parts: Vec<u32> = v1.split('.')
            .filter_map(|s| s.parse().ok())
            .collect();
        let v2_parts: Vec<u32> = v2.split('.')
            .filter_map(|s| s.parse().ok())
            .collect();

        if v1_parts.is_empty() || v2_parts.is_empty() {
            return None;
        }

        let max_len = v1_parts.len().max(v2_parts.len());
        
        for i in 0..max_len {
            let v1_val = v1_parts.get(i).copied().unwrap_or(0);
            let v2_val = v2_parts.get(i).copied().unwrap_or(0);
            
            match v1_val.cmp(&v2_val) {
                std::cmp::Ordering::Equal => continue,
                other => return Some(other),
            }
        }

        Some(std::cmp::Ordering::Equal)
    }

    /// Check if version is newer
    pub fn is_newer(new_version: &str, current_version: &str) -> bool {
        Self::compare(new_version, current_version)
            .map(|o| o == std::cmp::Ordering::Greater)
            .unwrap_or(false)
    }

    /// Get current version
    pub fn current() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
}

