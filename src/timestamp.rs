//! Timestamp format configuration.
//!
//! This module provides timestamp format presets matching zylog's TSFormat enum.

use serde::{Deserialize, Serialize};

/// Timestamp format presets.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TSFormat {
    /// RFC3339: "2006-01-02T15:04:05Z07:00"
    RFC3339,

    /// Standard: "2006-01-02 15:04:05"
    Standard,

    /// Simple: "20060102.150405"
    Simple,

    /// Time only: "15:04:05"
    TimeOnly,

    /// Custom chrono format string
    Custom(String),
}

impl Default for TSFormat {
    fn default() -> Self {
        Self::Standard
    }
}

impl TSFormat {
    /// Convert to chrono format string
    pub fn to_format_string(&self) -> &str {
        match self {
            Self::RFC3339 => "%Y-%m-%dT%H:%M:%S%z",
            Self::Standard => "%Y-%m-%d %H:%M:%S",
            Self::Simple => "%Y%m%d.%H%M%S",
            Self::TimeOnly => "%H:%M:%S",
            Self::Custom(s) => s.as_str(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tsformat_to_format_string() {
        assert_eq!(TSFormat::RFC3339.to_format_string(), "%Y-%m-%dT%H:%M:%S%z");
        assert_eq!(TSFormat::Standard.to_format_string(), "%Y-%m-%d %H:%M:%S");
        assert_eq!(TSFormat::Simple.to_format_string(), "%Y%m%d.%H%M%S");
        assert_eq!(TSFormat::TimeOnly.to_format_string(), "%H:%M:%S");
    }

    #[test]
    fn test_tsformat_custom() {
        let custom = TSFormat::Custom("%H:%M".to_string());
        assert_eq!(custom.to_format_string(), "%H:%M");
    }

    #[test]
    fn test_tsformat_default() {
        let default = TSFormat::default();
        assert_eq!(default, TSFormat::Standard);
    }
}
