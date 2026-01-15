//! Error types for the twyg logging library.

use std::io;

use thiserror::Error;

/// Custom error type for twyg operations.
#[derive(Debug, Error)]
pub enum TwygError {
    /// Invalid time format string provided.
    #[error("invalid time format '{format}': {source}")]
    InvalidTimeFormat {
        format: String,
        #[source]
        source: io::Error,
    },

    /// Failed to initialize the logger.
    #[error("failed to initialize logger: {0}")]
    InitError(#[from] log::SetLoggerError),

    /// Failed to open or write to log file.
    #[error("failed to open log file: {0}")]
    FileError(#[from] io::Error),

    /// Configuration error.
    #[error("configuration error: {0}")]
    ConfigError(String),
}

/// Result type alias using TwygError.
pub type Result<T> = std::result::Result<T, TwygError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = TwygError::ConfigError("test error".to_string());
        assert_eq!(err.to_string(), "configuration error: test error");
    }

    #[test]
    fn test_file_error() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let err = TwygError::FileError(io_err);
        assert!(err.to_string().contains("failed to open log file"));
    }

    #[test]
    fn test_invalid_time_format() {
        let io_err = io::Error::new(io::ErrorKind::InvalidInput, "bad format");
        let err = TwygError::InvalidTimeFormat {
            format: "%Z".to_string(),
            source: io_err,
        };
        assert!(err.to_string().contains("invalid time format"));
        assert!(err.to_string().contains("%Z"));
    }
}
