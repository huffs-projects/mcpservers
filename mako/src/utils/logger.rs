use log::{debug, error, info, warn};
use std::time::SystemTime;

/// Structured logging per endpoint for agent reasoning
pub struct EndpointLogger {
    endpoint_name: String,
    start_time: SystemTime,
}

impl EndpointLogger {
    pub fn new(endpoint_name: &str) -> Self {
        let logger = Self {
            endpoint_name: endpoint_name.to_string(),
            start_time: SystemTime::now(),
        };
        info!("[{}] Starting request", logger.endpoint_name);
        logger
    }

    pub fn log_info(&self, message: &str) {
        info!("[{}] {}", self.endpoint_name, message);
    }

    pub fn log_debug(&self, message: &str) {
        debug!("[{}] {}", self.endpoint_name, message);
    }

    pub fn log_warning(&self, message: &str) {
        warn!("[{}] {}", self.endpoint_name, message);
    }

    pub fn log_error(&self, message: &str) {
        error!("[{}] {}", self.endpoint_name, message);
    }

    pub fn log_success(&self, message: &str) {
        let elapsed = self.start_time.elapsed().unwrap_or_default();
        info!(
            "[{}] ✓ {} (took {:?})",
            self.endpoint_name, message, elapsed
        );
    }
}

impl Drop for EndpointLogger {
    fn drop(&mut self) {
        let elapsed = self.start_time.elapsed().unwrap_or_default();
        debug!("[{}] Request completed in {:?}", self.endpoint_name, elapsed);
    }
}

