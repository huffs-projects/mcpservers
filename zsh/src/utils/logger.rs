use serde_json::Value;
use tracing::{debug, error, info, warn};

/// Logger that integrates with the tracing framework.
/// 
/// All logging is done through the tracing macros which provide structured logging
/// with automatic span creation and context propagation.
pub struct Logger;

impl Logger {
    /// Log an operation with details using tracing.
    /// 
    /// # Arguments
    /// 
    /// * `operation` - The operation name
    /// * `details` - JSON value containing operation details
    pub fn log_operation(operation: &str, details: &Value) {
        debug!(operation = operation, details = %details, "Operation executed");
    }
    
    /// Log an error using tracing.
    /// 
    /// # Arguments
    /// 
    /// * `operation` - The operation name
    /// * `error` - Error message
    pub fn log_error(operation: &str, error: &str) {
        error!(operation = operation, error = error, "Operation failed");
    }
    
    /// Log an info message using tracing.
    /// 
    /// # Arguments
    /// 
    /// * `operation` - The operation name
    /// * `message` - Info message
    pub fn log_info(operation: &str, message: &str) {
        info!(operation = operation, message = message, "Operation info");
    }
    
    /// Log a warning using tracing.
    /// 
    /// # Arguments
    /// 
    /// * `operation` - The operation name
    /// * `message` - Warning message
    pub fn log_warn(operation: &str, message: &str) {
        warn!(operation = operation, message = message, "Operation warning");
    }
}

