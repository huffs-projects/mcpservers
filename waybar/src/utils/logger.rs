use tracing::{debug, error, info, warn};

pub struct Logger;

impl Logger {
    pub fn init() {
        // MCP servers must log to stderr, not stdout
        tracing_subscriber::fmt()
            .with_writer(std::io::stderr)
            .with_env_filter(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "waybar_mcp=info".into()),
            )
            .init();
    }

    pub fn log_operation(operation: &str, details: &str) {
        info!("Operation: {} - {}", operation, details);
    }

    pub fn log_error(operation: &str, error: &str) {
        error!("Error in {}: {}", operation, error);
    }

    pub fn log_warning(operation: &str, warning: &str) {
        warn!("Warning in {}: {}", operation, warning);
    }

    pub fn log_debug(operation: &str, message: &str) {
        debug!("Debug in {}: {}", operation, message);
    }
}

