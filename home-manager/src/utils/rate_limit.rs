use governor::{Quota, RateLimiter};
use std::num::NonZeroU32;
use std::sync::Arc;
use tracing::warn;

pub struct RateLimiterManager {
    limiter: Arc<RateLimiter<governor::state::direct::NotKeyed, governor::state::InMemoryState, governor::clock::DefaultClock>>,
}

impl RateLimiterManager {
    pub fn new(requests_per_second: u32) -> Self {
        let quota = Quota::per_second(NonZeroU32::new(requests_per_second).unwrap_or(NonZeroU32::new(1).unwrap()));
        let limiter = RateLimiter::direct(quota);
        
        Self {
            limiter: Arc::new(limiter),
        }
    }

    pub fn check(&self) -> Result<(), RateLimitError> {
        self.limiter.check()
            .map_err(|_| RateLimitError::RateLimited)
    }

    pub async fn check_until_ready(&self) {
        self.limiter.until_ready().await;
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RateLimitError {
    #[error("Rate limit exceeded")]
    RateLimited,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{sleep, Duration as TokioDuration};

    #[tokio::test]
    async fn test_rate_limiter() {
        let limiter = RateLimiterManager::new(2);
        
        assert!(limiter.check().is_ok());
        assert!(limiter.check().is_ok());
        
        // Third request should be rate limited
        assert!(limiter.check().is_err());
        
        // Wait and try again
        sleep(TokioDuration::from_millis(600)).await;
        assert!(limiter.check().is_ok());
    }
}

