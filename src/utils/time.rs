use std::time::{SystemTime, UNIX_EPOCH, Duration};

/// Get current Unix timestamp
pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// Get current timestamp as milliseconds
pub fn current_timestamp_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

/// Format timestamp as ISO 8601 string
pub fn format_timestamp(timestamp: u64) -> String {
    use chrono::{DateTime, Utc, NaiveDateTime};
    
    if let Some(dt) = NaiveDateTime::from_timestamp_opt(timestamp as i64, 0) {
        let utc_dt = DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc);
        utc_dt.format("%Y-%m-%d %H:%M:%S UTC").to_string()
    } else {
        "Invalid timestamp".to_string()
    }
}

/// Calculate elapsed time
pub fn elapsed_since(timestamp: u64) -> Duration {
    let now = current_timestamp();
    if now > timestamp {
        Duration::from_secs(now - timestamp)
    } else {
        Duration::from_secs(0)
    }
}

