use std::sync::Arc;
use tracing::{info, warn, error};

pub struct Logger;

impl Logger {
    pub fn init_stdio() {
        // #region agent log
        use std::io::Write;
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open("/Users/huffmullen/mcp/flakes/.cursor/debug.log") {
            let _ = writeln!(f, r#"{{"id":"log_logger_001","timestamp":{},"location":"logger.rs:7","message":"init_stdio called","data":{{"rust_log":{:?}}},"sessionId":"debug-session","runId":"run1","hypothesisId":"B"}}"#, 
                std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(),
                std::env::var("RUST_LOG")
            );
        }
        // #endregion
        // For stdio mode, send all logs to stderr and disable ANSI
        tracing_subscriber::fmt()
            .with_writer(std::io::stderr)
            .with_ansi(false) // Disable ANSI colors for cleaner output
            .with_target(false) // Don't show target/module names
            .with_thread_ids(false) // Don't show thread IDs
            .with_file(false) // Don't show file names
            .with_line_number(false) // Don't show line numbers
            .with_env_filter(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("warn")),
            )
            .init();
        // #region agent log
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open("/Users/huffmullen/mcp/flakes/.cursor/debug.log") {
            let _ = writeln!(f, r#"{{"id":"log_logger_002","timestamp":{},"location":"logger.rs:20","message":"init_stdio completed","data":{{}},"sessionId":"debug-session","runId":"run1","hypothesisId":"B"}}"#, 
                std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis()
            );
        }
        // #endregion
    }

    pub fn init() {
        tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
            )
            .init();
    }

    pub fn info(message: &str) {
        info!("{}", message);
    }

    pub fn warn(message: &str) {
        warn!("{}", message);
    }

    pub fn error(message: &str) {
        error!("{}", message);
    }
}

