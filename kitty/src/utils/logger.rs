use log::{Level, Metadata, Record};

pub fn structured_log(level: Level, message: &str, context: &[(&str, &str)]) {
    let mut log_entry = format!("[{}] {}", level, message);
    if !context.is_empty() {
        log_entry.push_str(" | ");
        log_entry.push_str(
            &context
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join(" "),
        );
    }
    eprintln!("{}", log_entry);
}

pub struct StructuredLogger;

impl log::Log for StructuredLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            structured_log(
                record.level(),
                &record.args().to_string(),
                &[],
            );
        }
    }

    fn flush(&self) {}
}

