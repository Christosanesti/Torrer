use std::time::Duration;
use crate::error::TorrerResult;

/// Retry configuration
pub struct RetryConfig {
    pub max_attempts: u32,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(30),
            multiplier: 2.0,
        }
    }
}

/// Retry a function with exponential backoff
pub async fn retry_with_backoff<F, Fut, T>(config: RetryConfig, mut f: F) -> TorrerResult<T>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = TorrerResult<T>>,
{
    let mut delay = config.initial_delay;
    let mut last_error = None;

    for attempt in 1..=config.max_attempts {
        match f().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                last_error = Some(e);
                
                if attempt < config.max_attempts {
                    log::debug!("Attempt {} failed, retrying in {:?}...", attempt, delay);
                    tokio::time::sleep(delay).await;
                    
                    // Exponential backoff
                    delay = Duration::from_secs_f64(
                        (delay.as_secs_f64() * config.multiplier).min(config.max_delay.as_secs_f64())
                    );
                }
            }
        }
    }

    Err(last_error.unwrap_or_else(|| {
        crate::error::TorrerError::Tor("All retry attempts failed".to_string())
    }))
}

/// Retry a function with fixed delay
pub async fn retry_fixed<F, Fut, T>(
    max_attempts: u32,
    delay: Duration,
    mut f: F,
) -> TorrerResult<T>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = TorrerResult<T>>,
{
    let mut last_error = None;

    for attempt in 1..=max_attempts {
        match f().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                last_error = Some(e);
                
                if attempt < max_attempts {
                    log::debug!("Attempt {} failed, retrying in {:?}...", attempt, delay);
                    tokio::time::sleep(delay).await;
                }
            }
        }
    }

    Err(last_error.unwrap_or_else(|| {
        crate::error::TorrerError::Tor("All retry attempts failed".to_string())
    }))
}

