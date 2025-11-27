use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};

/// Rate limiter for API calls and operations
pub struct RateLimiter {
    limits: Arc<Mutex<HashMap<String, Vec<Instant>>>>,
    max_requests: u32,
    window: Duration,
}

impl RateLimiter {
    /// Create a new rate limiter
    pub fn new(max_requests: u32, window: Duration) -> Self {
        Self {
            limits: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window,
        }
    }

    /// Check if request is allowed
    pub fn check(&self, key: &str) -> bool {
        let mut limits = self.limits.lock().unwrap();
        let now = Instant::now();
        
        // Get or create entry
        let requests = limits.entry(key.to_string()).or_insert_with(Vec::new);
        
        // Remove old requests outside window
        requests.retain(|&time| now.duration_since(time) < self.window);
        
        // Check if under limit
        if requests.len() < self.max_requests as usize {
            requests.push(now);
            true
        } else {
            false
        }
    }

    /// Wait until request is allowed
    pub async fn wait(&self, key: &str) {
        while !self.check(key) {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    /// Reset rate limit for a key
    pub fn reset(&self, key: &str) {
        let mut limits = self.limits.lock().unwrap();
        limits.remove(key);
    }

    /// Reset all rate limits
    pub fn reset_all(&self) {
        let mut limits = self.limits.lock().unwrap();
        limits.clear();
    }
}

