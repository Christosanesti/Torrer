// Unit tests for retry utilities

#[cfg(test)]
mod tests {
    use torrer::utils::retry::{RetryConfig, retry_fixed};
    use std::time::Duration;
    use torrer::error::TorrerResult;

    #[tokio::test]
    async fn test_retry_success() {
        let mut attempts = 0;
        let result = retry_fixed(
            3,
            Duration::from_millis(10),
            || async {
                attempts += 1;
                if attempts == 2 {
                    Ok(42)
                } else {
                    Err(torrer::error::TorrerError::Tor("Test error".to_string()))
                }
            },
        )
        .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
        assert_eq!(attempts, 2);
    }

    #[tokio::test]
    async fn test_retry_failure() {
        let result = retry_fixed(
            3,
            Duration::from_millis(10),
            || async {
                Err(torrer::error::TorrerError::Tor("Always fails".to_string()))
            },
        )
        .await;

        assert!(result.is_err());
    }
}

