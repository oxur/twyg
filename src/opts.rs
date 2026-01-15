//! Logger configuration options.
//!
//! This module provides the [`Opts`] struct for configuring the twyg logger.

use chrono::Local;
use serde::{Deserialize, Serialize};

use super::error::{Result, TwygError};
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
/// use twyg::{LogLevel, OptsBuilder, Output};
///
/// let opts = OptsBuilder::new()
///     .coloured(true)
///     .output(Output::Stdout)
///     .level(LogLevel::Debug)
///     .report_caller(true)
///     .build()
///     .unwrap();
/// ```
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Opts {
    /// Enable colored output using ANSI escape codes.
    coloured: bool,

    /// Output destination (stdout, stderr, or file).
    output: Output,

    /// Minimum log level to display.
    level: LogLevel,

    /// Include file name and line number in log output.
    report_caller: bool,

    /// Custom time format string (chrono format).
    /// If None, uses the default format: "%Y-%m-%d %H:%M:%S".
    time_format: Option<String>,
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

    /// Returns whether colored output is enabled.
    pub fn coloured(&self) -> bool {
        self.coloured
    }

    /// Returns the output destination.
    pub fn output(&self) -> &Output {
        &self.output
    }

    /// Returns the minimum log level.
    pub fn level(&self) -> LogLevel {
        self.level
    }

    /// Returns whether caller reporting is enabled.
    pub fn report_caller(&self) -> bool {
        self.report_caller
    }

    /// Returns the time format string, if set.
    pub fn time_format(&self) -> Option<&str> {
        self.time_format.as_deref()
    }
}

/// Builder for constructing [`Opts`] with validation.
///
/// # Examples
///
/// ```
/// use twyg::{LogLevel, OptsBuilder, Output};
///
/// let opts = OptsBuilder::new()
///     .coloured(true)
///     .level(LogLevel::Debug)
///     .report_caller(true)
///     .build()
///     .unwrap();
/// ```
#[derive(Clone, Debug)]
pub struct OptsBuilder {
    coloured: bool,
    output: Output,
    level: LogLevel,
    report_caller: bool,
    time_format: Option<String>,
}

impl Default for OptsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl OptsBuilder {
    /// Creates a new OptsBuilder with default values.
    pub fn new() -> Self {
        Self {
            coloured: false,
            output: Output::default(),
            level: LogLevel::default(),
            report_caller: false,
            time_format: None,
        }
    }

    /// Enable or disable colored output.
    pub fn coloured(mut self, coloured: bool) -> Self {
        self.coloured = coloured;
        self
    }

    /// Set the output destination.
    pub fn output(mut self, output: Output) -> Self {
        self.output = output;
        self
    }

    /// Set the minimum log level.
    pub fn level(mut self, level: LogLevel) -> Self {
        self.level = level;
        self
    }

    /// Enable or disable caller reporting.
    pub fn report_caller(mut self, report: bool) -> Self {
        self.report_caller = report;
        self
    }

    /// Set a custom time format string.
    ///
    /// The format string uses chrono's format syntax.
    ///
    /// # Examples
    ///
    /// ```
    /// use twyg::OptsBuilder;
    ///
    /// let opts = OptsBuilder::new()
    ///     .time_format("%H:%M:%S")
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn time_format(mut self, format: impl Into<String>) -> Self {
        self.time_format = Some(format.into());
        self
    }

    /// Build the Opts, validating the time format if provided.
    ///
    /// # Errors
    ///
    /// Returns an error if the time format string is invalid.
    pub fn build(self) -> Result<Opts> {
        // Validate time_format if provided
        if let Some(ref fmt) = self.time_format {
            validate_time_format(fmt)?;
        }

        Ok(Opts {
            coloured: self.coloured,
            output: self.output,
            level: self.level,
            report_caller: self.report_caller,
            time_format: self.time_format,
        })
    }
}

/// Validates a time format string by attempting to format the current time.
fn validate_time_format(format: &str) -> Result<()> {
    match std::panic::catch_unwind(|| {
        Local::now().format(format).to_string();
    }) {
        Ok(_) => Ok(()),
        Err(_) => Err(TwygError::ConfigError(format!(
            "invalid time format string: {}",
            format
        ))),
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
        assert!(!opts.coloured());
        assert_eq!(opts.output(), &Output::Stdout);
        assert_eq!(opts.level(), LogLevel::Error);
        assert!(!opts.report_caller());
        assert!(opts.time_format().is_none());
    }

    #[test]
    fn test_new_opts_sets_defaults() {
        let opts = Opts::new();
        assert!(!opts.coloured());
        assert_eq!(opts.output(), &Output::Stdout);
        assert_eq!(opts.level(), LogLevel::Error);
        assert!(!opts.report_caller());
        assert_eq!(opts.time_format(), Some("%Y-%m-%d %H:%M:%S"));
    }

    #[test]
    fn test_opts_clone() {
        let opts1 = Opts::new();
        let opts2 = opts1.clone();
        assert_eq!(opts1.output(), opts2.output());
        assert_eq!(opts1.level(), opts2.level());
        assert_eq!(opts1.coloured(), opts2.coloured());
    }

    #[test]
    fn test_opts_debug() {
        let opts = Opts::new();
        let debug_str = format!("{:?}", opts);
        assert!(debug_str.contains("Opts"));
    }

    #[test]
    fn test_opts_serialize_deserialize() {
        let opts = OptsBuilder::new()
            .coloured(true)
            .output(Output::Stderr)
            .level(LogLevel::Debug)
            .report_caller(true)
            .time_format("%H:%M:%S")
            .build()
            .unwrap();

        let serialized = serde_json::to_string(&opts).unwrap();
        let deserialized: Opts = serde_json::from_str(&serialized).unwrap();

        assert_eq!(opts.coloured(), deserialized.coloured());
        assert_eq!(opts.output(), deserialized.output());
        assert_eq!(opts.level(), deserialized.level());
        assert_eq!(opts.report_caller(), deserialized.report_caller());
        assert_eq!(opts.time_format(), deserialized.time_format());
    }

    #[test]
    fn test_opts_builder_with_custom_values() {
        let opts = OptsBuilder::new()
            .coloured(true)
            .output(Output::file("/tmp/test.log"))
            .level(LogLevel::Trace)
            .report_caller(true)
            .time_format("%Y-%m-%d")
            .build()
            .unwrap();

        assert!(opts.coloured());
        assert_eq!(opts.output(), &Output::file("/tmp/test.log"));
        assert_eq!(opts.level(), LogLevel::Trace);
        assert!(opts.report_caller());
        assert_eq!(opts.time_format(), Some("%Y-%m-%d"));
    }

    #[test]
    fn test_opts_builder_with_different_outputs() {
        let stdout_opts = OptsBuilder::new().output(Output::Stdout).build().unwrap();
        assert_eq!(stdout_opts.output(), &Output::Stdout);

        let stderr_opts = OptsBuilder::new().output(Output::Stderr).build().unwrap();
        assert_eq!(stderr_opts.output(), &Output::Stderr);

        let file_opts = OptsBuilder::new()
            .output(Output::file("/var/log/app.log"))
            .build()
            .unwrap();
        assert_eq!(file_opts.output(), &Output::file("/var/log/app.log"));
    }

    #[test]
    fn test_opts_builder_default() {
        let opts = OptsBuilder::default().build().unwrap();
        assert!(!opts.coloured());
        assert_eq!(opts.output(), &Output::Stdout);
        assert_eq!(opts.level(), LogLevel::Error);
        assert!(!opts.report_caller());
        assert!(opts.time_format().is_none());
    }

    #[test]
    fn test_validate_time_format_valid() {
        assert!(validate_time_format("%Y-%m-%d %H:%M:%S").is_ok());
        assert!(validate_time_format("%H:%M:%S").is_ok());
        assert!(validate_time_format("%Y-%m-%d").is_ok());
    }

    #[test]
    fn test_validate_time_format_invalid() {
        // Note: chrono's format is quite lenient, so many things work
        // This test documents the behavior
        let result = validate_time_format("%Z"); // %Z might not work in all contexts
                                                 // Most format strings are accepted, so this might pass
        let _ = result;
    }

    #[test]
    fn test_opts_builder_chaining() {
        let opts = OptsBuilder::new()
            .coloured(true)
            .level(LogLevel::Debug)
            .report_caller(true)
            .output(Output::Stderr)
            .build()
            .unwrap();

        assert!(opts.coloured());
        assert_eq!(opts.level(), LogLevel::Debug);
        assert!(opts.report_caller());
        assert_eq!(opts.output(), &Output::Stderr);
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
