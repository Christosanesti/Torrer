use std::time::Duration;
use tokio::time::timeout;

/// Async utility functions
pub struct AsyncUtils;

impl AsyncUtils {
    /// Run with timeout
    pub async fn with_timeout<F, T>(
        duration: Duration,
        future: F,
    ) -> Result<T, tokio::time::error::Elapsed>
    where
        F: std::future::Future<Output = T>,
    {
        timeout(duration, future).await
    }

    /// Retry with timeout
    pub async fn retry_with_timeout<F, Fut, T>(
        max_attempts: u32,
        timeout_duration: Duration,
        delay: Duration,
        mut f: F,
    ) -> Option<T>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Option<T>>,
    {
        for attempt in 1..=max_attempts {
            match Self::with_timeout(timeout_duration, f()).await {
                Ok(Some(result)) => return Some(result),
                Ok(None) => {
                    if attempt < max_attempts {
                        tokio::time::sleep(delay).await;
                    }
                }
                Err(_) => {
                    if attempt < max_attempts {
                        tokio::time::sleep(delay).await;
                    }
                }
            }
        }
        None
    }

    /// Parallel execution
    pub async fn parallel<Fut, T>(futures: Vec<Fut>) -> Vec<T>
    where
        Fut: std::future::Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        let handles: Vec<_> = futures
            .into_iter()
            .map(|fut| tokio::spawn(fut))
            .collect();

        let mut results = Vec::new();
        for handle in handles {
            if let Ok(result) = handle.await {
                results.push(result);
            }
        }
        results
    }
}

