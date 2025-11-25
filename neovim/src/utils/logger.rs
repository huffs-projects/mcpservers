use serde_json::json;

/// Structured JSON logger for agent reasoning
pub struct StructuredLogger {
    enabled: bool,
}

impl StructuredLogger {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }

    pub fn log(&self, level: &str, message: &str, metadata: Option<serde_json::Value>) {
        if !self.enabled {
            return;
        }

        let log_entry = json!({
            "level": level,
            "message": message,
            "timestamp": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            "metadata": metadata
        });

        eprintln!("{}", serde_json::to_string_pretty(&log_entry).unwrap_or_default());
    }

    pub fn info(&self, message: &str) {
        self.log("info", message, None);
    }

    pub fn warn(&self, message: &str) {
        self.log("warn", message, None);
    }

    pub fn error(&self, message: &str) {
        self.log("error", message, None);
    }

    pub fn debug(&self, message: &str, metadata: serde_json::Value) {
        self.log("debug", message, Some(metadata));
    }
}

impl Default for StructuredLogger {
    fn default() -> Self {
        Self::new(true)
    }
}

