//! Logger configuration options.
//!
//! This module provides the [`Opts`] struct for configuring the twyg logger.

use serde::{Deserialize, Serialize};

use super::level::LogLevel;
use super::output::Output;

const DEFAULT_TS_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

/// Logger configuration options.
///
/// Configure all aspects of the twyg logger including output destination,
/// log level, colors, and formatting.
///
/// # Examples
///
/// ```
/// use twyg::{Opts, LogLevel, Output};
///
/// let opts = Opts {
///     coloured: true,
///     output: Output::Stdout,
///     level: LogLevel::Debug,
///     report_caller: true,
///     time_format: None,
/// };
/// ```
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Opts {
    /// Enable colored output using ANSI escape codes.
    pub coloured: bool,

    /// Output destination (stdout, stderr, or file).
    pub output: Output,

    /// Minimum log level to display.
    pub level: LogLevel,

    /// Include file name and line number in log output.
    pub report_caller: bool,

    /// Custom time format string (chrono format).
    /// If None, uses the default format: "%Y-%m-%d %H:%M:%S".
    pub time_format: Option<String>,
}

impl Opts {
    /// Creates a new Opts with default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use twyg::Opts;
    ///
    /// let opts = Opts::new();
    /// ```
    pub fn new() -> Opts {
        Opts {
            coloured: false,
            output: Output::default(),
            level: LogLevel::default(),
            report_caller: false,
            time_format: Some(DEFAULT_TS_FORMAT.to_string()),
        }
    }
}

// Backwards compatibility helpers (deprecated)
pub mod compat {
    use super::*;
    use crate::out;

    /// Returns the default file output (deprecated).
    #[deprecated(since = "0.6.0", note = "Use Output::default() instead")]
    pub fn default_file() -> Option<String> {
        Some(out::STDOUT.to_string())
    }

    /// Returns the default log level (deprecated).
    #[deprecated(since = "0.6.0", note = "Use LogLevel::default() instead")]
    pub fn default_level() -> Option<String> {
        Some("error".to_string())
    }

    /// Returns the default timestamp format (deprecated).
    #[deprecated(since = "0.6.0", note = "Use Opts::new() or set time_format directly")]
    pub fn default_ts_format() -> Option<String> {
        Some(DEFAULT_TS_FORMAT.to_string())
    }
}

// Re-export deprecated functions at module level for backwards compatibility
#[allow(deprecated)]
#[deprecated(since = "0.6.0", note = "Use Output::default() instead")]
pub use compat::default_file;

#[allow(deprecated)]
#[deprecated(since = "0.6.0", note = "Use LogLevel::default() instead")]
pub use compat::default_level;

#[allow(deprecated)]
#[deprecated(since = "0.6.0", note = "Use Opts::new() or set time_format directly")]
pub use compat::default_ts_format;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_opts() {
        let opts = Opts::default();
        assert!(!opts.coloured);
        assert_eq!(opts.output, Output::Stdout);
        assert_eq!(opts.level, LogLevel::Error);
        assert!(!opts.report_caller);
        assert!(opts.time_format.is_none());
    }

    #[test]
    fn test_new_opts_sets_defaults() {
        let opts = Opts::new();
        assert!(!opts.coloured);
        assert_eq!(opts.output, Output::Stdout);
        assert_eq!(opts.level, LogLevel::Error);
        assert!(!opts.report_caller);
        assert_eq!(opts.time_format, Some("%Y-%m-%d %H:%M:%S".to_string()));
    }

    #[test]
    fn test_opts_clone() {
        let opts1 = Opts::new();
        let opts2 = opts1.clone();
        assert_eq!(opts1.output, opts2.output);
        assert_eq!(opts1.level, opts2.level);
        assert_eq!(opts1.coloured, opts2.coloured);
    }

    #[test]
    fn test_opts_debug() {
        let opts = Opts::new();
        let debug_str = format!("{:?}", opts);
        assert!(debug_str.contains("Opts"));
    }

    #[test]
    fn test_opts_serialize_deserialize() {
        let opts = Opts {
            coloured: true,
            output: Output::Stderr,
            level: LogLevel::Debug,
            report_caller: true,
            time_format: Some("%H:%M:%S".to_string()),
        };

        let serialized = serde_json::to_string(&opts).unwrap();
        let deserialized: Opts = serde_json::from_str(&serialized).unwrap();

        assert_eq!(opts.coloured, deserialized.coloured);
        assert_eq!(opts.output, deserialized.output);
        assert_eq!(opts.level, deserialized.level);
        assert_eq!(opts.report_caller, deserialized.report_caller);
        assert_eq!(opts.time_format, deserialized.time_format);
    }

    #[test]
    fn test_opts_with_custom_values() {
        let opts = Opts {
            coloured: true,
            output: Output::file("/tmp/test.log"),
            level: LogLevel::Trace,
            report_caller: true,
            time_format: Some("%Y-%m-%d".to_string()),
        };

        assert!(opts.coloured);
        assert_eq!(opts.output, Output::file("/tmp/test.log"));
        assert_eq!(opts.level, LogLevel::Trace);
        assert!(opts.report_caller);
        assert_eq!(opts.time_format, Some("%Y-%m-%d".to_string()));
    }

    #[test]
    fn test_opts_with_different_outputs() {
        let stdout_opts = Opts {
            output: Output::Stdout,
            ..Default::default()
        };
        assert_eq!(stdout_opts.output, Output::Stdout);

        let stderr_opts = Opts {
            output: Output::Stderr,
            ..Default::default()
        };
        assert_eq!(stderr_opts.output, Output::Stderr);

        let file_opts = Opts {
            output: Output::file("/var/log/app.log"),
            ..Default::default()
        };
        assert_eq!(file_opts.output, Output::file("/var/log/app.log"));
    }

    // Test deprecated functions still work
    #[test]
    #[allow(deprecated)]
    fn test_deprecated_default_file() {
        let file = default_file();
        assert_eq!(file, Some("stdout".to_string()));
    }

    #[test]
    #[allow(deprecated)]
    fn test_deprecated_default_level() {
        let level = default_level();
        assert_eq!(level, Some("error".to_string()));
    }

    #[test]
    #[allow(deprecated)]
    fn test_deprecated_default_ts_format() {
        let format = default_ts_format();
        assert_eq!(format, Some("%Y-%m-%d %H:%M:%S".to_string()));
    }
}
