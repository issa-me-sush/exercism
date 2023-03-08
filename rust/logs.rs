/// Various log levels.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warning => write!(f, "WARNING"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

/// Primary function for emitting logs.
pub fn log(level: LogLevel, message: &str) -> String {
    format!("[{}]: {}", level, message)
}

/// Emits an info log message.
pub fn info(message: &str) -> String {
    log(LogLevel::Info, message)
}

/// Emits a warning log message.
pub fn warn(message: &str) -> String {
    log(LogLevel::Warning, message)
}

/// Emits an error log message.
pub fn error(message: &str) -> String {
    log(LogLevel::Error, message)
}
