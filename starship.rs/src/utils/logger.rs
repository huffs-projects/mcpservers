use log::{debug, error, info, warn};
use std::fmt;

pub struct Logger {
    endpoint: String,
}

impl Logger {
    pub fn new(endpoint: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
        }
    }

    pub fn info(&self, message: impl fmt::Display) {
        info!("[{}] {}", self.endpoint, message);
    }

    #[allow(dead_code)]
    pub fn warn(&self, message: impl fmt::Display) {
        warn!("[{}] {}", self.endpoint, message);
    }

    #[allow(dead_code)]
    pub fn error(&self, message: impl fmt::Display) {
        error!("[{}] {}", self.endpoint, message);
    }

    #[allow(dead_code)]
    pub fn debug(&self, message: impl fmt::Display) {
        debug!("[{}] {}", self.endpoint, message);
    }
}

