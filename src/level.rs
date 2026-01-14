//! Log level types and conversions.
//!
//! This module provides the [`LogLevel`] enum for type-safe log level configuration.

use log::LevelFilter;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Log level for filtering messages.
///
/// Represents the minimum severity level for log messages to be displayed.
/// Levels are ordered from most verbose (Trace) to least verbose (Fatal).
///
/// # Examples
///
/// ```
/// use twyg::LogLevel;
///
/// let level = LogLevel::Debug;
/// assert_eq!(level.to_string(), "debug");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    /// The most verbose level, for detailed trace information.
    Trace,
    /// Debug information, useful during development.
    Debug,
    /// Informational messages about normal operation.
    Info,
    /// Warning messages for potentially problematic situations.
    Warn,
    /// Error messages for error conditions.
    Error,
    /// Fatal/critical errors (mapped to Error in log crate).
    Fatal,
}

impl LogLevel {
    /// Returns a slice of all log levels in order from most to least verbose.
    pub const fn all() -> &'static [LogLevel] {
        &[
            LogLevel::Trace,
            LogLevel::Debug,
            LogLevel::Info,
            LogLevel::Warn,
            LogLevel::Error,
            LogLevel::Fatal,
        ]
    }

    /// Returns the string representation in lowercase.
    pub const fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Trace => "trace",
            LogLevel::Debug => "debug",
            LogLevel::Info => "info",
            LogLevel::Warn => "warn",
            LogLevel::Error => "error",
            LogLevel::Fatal => "fatal",
        }
    }
}

impl Default for LogLevel {
    /// Returns the default log level: [`LogLevel::Error`].
    fn default() -> Self {
        Self::Error
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for LogLevel {
    type Err = ParseLogLevelError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "trace" => Ok(LogLevel::Trace),
            "debug" => Ok(LogLevel::Debug),
            "info" => Ok(LogLevel::Info),
            "warn" | "warning" => Ok(LogLevel::Warn),
            "error" | "err" => Ok(LogLevel::Error),
            "fatal" => Ok(LogLevel::Fatal),
            _ => Err(ParseLogLevelError {
                invalid_input: s.to_string(),
            }),
        }
    }
}

/// Error returned when parsing a log level from a string fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseLogLevelError {
    invalid_input: String,
}

impl fmt::Display for ParseLogLevelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "invalid log level '{}', expected one of: trace, debug, info, warn, error, fatal",
            self.invalid_input
        )
    }
}

impl std::error::Error for ParseLogLevelError {}

/// Convert LogLevel to log crate's LevelFilter.
///
/// Note: Fatal is mapped to Error as the log crate doesn't have a Fatal level.
impl From<LogLevel> for LevelFilter {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Trace => LevelFilter::Trace,
            LogLevel::Debug => LevelFilter::Debug,
            LogLevel::Info => LevelFilter::Info,
            LogLevel::Warn => LevelFilter::Warn,
            LogLevel::Error | LogLevel::Fatal => LevelFilter::Error,
        }
    }
}

// Backwards compatibility helpers (deprecated)
#[deprecated(since = "0.6.0", note = "Use LogLevel::Trace instead")]
pub fn trace() -> Option<String> {
    Some(String::from("trace"))
}

#[deprecated(since = "0.6.0", note = "Use LogLevel::Debug instead")]
pub fn debug() -> Option<String> {
    Some(String::from("debug"))
}

#[deprecated(since = "0.6.0", note = "Use LogLevel::Info instead")]
pub fn info() -> Option<String> {
    Some(String::from("info"))
}

#[deprecated(since = "0.6.0", note = "Use LogLevel::Warn instead")]
pub fn warn() -> Option<String> {
    Some(String::from("warn"))
}

#[deprecated(since = "0.6.0", note = "Use LogLevel::Error instead")]
pub fn error() -> Option<String> {
    Some(String::from("error"))
}

#[deprecated(since = "0.6.0", note = "Use LogLevel::Error instead")]
pub fn fatal() -> Option<String> {
    Some(String::from("fatal"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level_as_str() {
        assert_eq!(LogLevel::Trace.as_str(), "trace");
        assert_eq!(LogLevel::Debug.as_str(), "debug");
        assert_eq!(LogLevel::Info.as_str(), "info");
        assert_eq!(LogLevel::Warn.as_str(), "warn");
        assert_eq!(LogLevel::Error.as_str(), "error");
        assert_eq!(LogLevel::Fatal.as_str(), "fatal");
    }

    #[test]
    fn test_log_level_display() {
        assert_eq!(LogLevel::Trace.to_string(), "trace");
        assert_eq!(LogLevel::Debug.to_string(), "debug");
        assert_eq!(LogLevel::Info.to_string(), "info");
        assert_eq!(LogLevel::Warn.to_string(), "warn");
        assert_eq!(LogLevel::Error.to_string(), "error");
        assert_eq!(LogLevel::Fatal.to_string(), "fatal");
    }

    #[test]
    fn test_log_level_from_str() {
        assert_eq!("trace".parse::<LogLevel>().unwrap(), LogLevel::Trace);
        assert_eq!("debug".parse::<LogLevel>().unwrap(), LogLevel::Debug);
        assert_eq!("info".parse::<LogLevel>().unwrap(), LogLevel::Info);
        assert_eq!("warn".parse::<LogLevel>().unwrap(), LogLevel::Warn);
        assert_eq!("warning".parse::<LogLevel>().unwrap(), LogLevel::Warn);
        assert_eq!("error".parse::<LogLevel>().unwrap(), LogLevel::Error);
        assert_eq!("err".parse::<LogLevel>().unwrap(), LogLevel::Error);
        assert_eq!("fatal".parse::<LogLevel>().unwrap(), LogLevel::Fatal);
    }

    #[test]
    fn test_log_level_from_str_case_insensitive() {
        assert_eq!("TRACE".parse::<LogLevel>().unwrap(), LogLevel::Trace);
        assert_eq!("Debug".parse::<LogLevel>().unwrap(), LogLevel::Debug);
        assert_eq!("INFO".parse::<LogLevel>().unwrap(), LogLevel::Info);
    }

    #[test]
    fn test_log_level_from_str_invalid() {
        let result = "invalid".parse::<LogLevel>();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("invalid log level"));
        assert!(err.to_string().contains("invalid"));
    }

    #[test]
    fn test_log_level_default() {
        assert_eq!(LogLevel::default(), LogLevel::Error);
    }

    #[test]
    fn test_log_level_to_filter() {
        assert_eq!(LevelFilter::from(LogLevel::Trace), LevelFilter::Trace);
        assert_eq!(LevelFilter::from(LogLevel::Debug), LevelFilter::Debug);
        assert_eq!(LevelFilter::from(LogLevel::Info), LevelFilter::Info);
        assert_eq!(LevelFilter::from(LogLevel::Warn), LevelFilter::Warn);
        assert_eq!(LevelFilter::from(LogLevel::Error), LevelFilter::Error);
        assert_eq!(LevelFilter::from(LogLevel::Fatal), LevelFilter::Error);
    }

    #[test]
    fn test_log_level_all() {
        let all = LogLevel::all();
        assert_eq!(all.len(), 6);
        assert_eq!(all[0], LogLevel::Trace);
        assert_eq!(all[5], LogLevel::Fatal);
    }

    #[test]
    fn test_log_level_clone() {
        let level = LogLevel::Debug;
        let cloned = level;
        assert_eq!(level, cloned);
    }

    #[test]
    fn test_log_level_eq() {
        assert_eq!(LogLevel::Debug, LogLevel::Debug);
        assert_ne!(LogLevel::Debug, LogLevel::Info);
    }

    #[test]
    fn test_log_level_serialize_deserialize() {
        let level = LogLevel::Debug;
        let serialized = serde_json::to_string(&level).unwrap();
        assert_eq!(serialized, r#""debug""#);

        let deserialized: LogLevel = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, LogLevel::Debug);
    }

    #[test]
    fn test_log_level_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(LogLevel::Debug);
        set.insert(LogLevel::Debug); // Duplicate
        set.insert(LogLevel::Info);

        assert_eq!(set.len(), 2); // Should have 2 unique levels
        assert!(set.contains(&LogLevel::Debug));
        assert!(set.contains(&LogLevel::Info));
        assert!(!set.contains(&LogLevel::Warn));
    }

    // Test deprecated functions still work
    #[test]
    #[allow(deprecated)]
    fn test_deprecated_functions() {
        assert_eq!(trace().unwrap(), "trace");
        assert_eq!(debug().unwrap(), "debug");
        assert_eq!(info().unwrap(), "info");
        assert_eq!(warn().unwrap(), "warn");
        assert_eq!(error().unwrap(), "error");
        assert_eq!(fatal().unwrap(), "fatal");
    }
}
