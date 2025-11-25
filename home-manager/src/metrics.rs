use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;

#[derive(Clone)]
pub struct Metrics {
    request_count: Arc<AtomicU64>,
    error_count: Arc<AtomicU64>,
    cache_hits: Arc<AtomicU64>,
    cache_misses: Arc<AtomicU64>,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            request_count: Arc::new(AtomicU64::new(0)),
            error_count: Arc::new(AtomicU64::new(0)),
            cache_hits: Arc::new(AtomicU64::new(0)),
            cache_misses: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn record_request(&self) {
        self.request_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_error(&self) {
        self.error_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_cache_hit(&self) {
        self.cache_hits.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_cache_miss(&self) {
        self.cache_misses.fetch_add(1, Ordering::Relaxed);
    }

    pub fn get_stats(&self) -> MetricsStats {
        MetricsStats {
            request_count: self.request_count.load(Ordering::Relaxed),
            error_count: self.error_count.load(Ordering::Relaxed),
            cache_hits: self.cache_hits.load(Ordering::Relaxed),
            cache_misses: self.cache_misses.load(Ordering::Relaxed),
        }
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct MetricsStats {
    pub request_count: u64,
    pub error_count: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
}

impl MetricsStats {
    pub fn cache_hit_rate(&self) -> f64 {
        let total = self.cache_hits + self.cache_misses;
        if total == 0 {
            return 0.0;
        }
        self.cache_hits as f64 / total as f64
    }

    pub fn error_rate(&self) -> f64 {
        if self.request_count == 0 {
            return 0.0;
        }
        self.error_count as f64 / self.request_count as f64
    }
}

pub struct RequestTimer {
    start: Instant,
    metrics: Metrics,
}

// Global metrics instance for cache tracking
static GLOBAL_METRICS: std::sync::OnceLock<Metrics> = std::sync::OnceLock::new();

pub fn get_global_metrics() -> &'static Metrics {
    GLOBAL_METRICS.get_or_init(|| Metrics::new())
}

pub fn set_global_metrics(metrics: Metrics) {
    let _ = GLOBAL_METRICS.set(metrics);
}

impl RequestTimer {
    pub fn start(metrics: &Metrics) -> Self {
        metrics.record_request();
        Self {
            start: Instant::now(),
            metrics: metrics.clone(),
        }
    }

    pub fn finish(self) -> Duration {
        self.start.elapsed()
    }
}

impl Drop for RequestTimer {
    fn drop(&mut self) {
        let duration = self.start.elapsed();
        if duration.as_secs() > 5 {
            tracing::warn!("Slow request: {:?}", duration);
        }
    }
}

use std::time::Duration;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics() {
        let metrics = Metrics::new();
        metrics.record_request();
        metrics.record_request();
        metrics.record_error();
        metrics.record_cache_hit();
        metrics.record_cache_miss();

        let stats = metrics.get_stats();
        assert_eq!(stats.request_count, 2);
        assert_eq!(stats.error_count, 1);
        assert_eq!(stats.cache_hits, 1);
        assert_eq!(stats.cache_misses, 1);
        assert_eq!(stats.cache_hit_rate(), 0.5);
        assert_eq!(stats.error_rate(), 0.5);
    }
}

