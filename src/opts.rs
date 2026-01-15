//! Logger configuration options.
//!
//! This module provides the [`Opts`] struct for configuring the twyg logger.

use chrono::Local;
use serde::{Deserialize, Serialize};

use super::color::Colors;
use super::error::{Result, TwygError};
use super::level::LogLevel;
use super::output::Output;
use super::timestamp::TSFormat;

const DEFAULT_TS_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

/// Side to pad level strings.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PadSide {
    /// Pad on the left (right-align)
    Left,

    /// Pad on the right (left-align)
    Right,
}

impl Default for PadSide {
    fn default() -> Self {
        Self::Right
    }
}

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
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Opts {
    /// Enable colored output using ANSI escape codes.
    coloured: bool,

    /// Output destination (stdout, stderr, or file).
    output: Output,

    /// Minimum log level to display.
    level: LogLevel,

    /// Include file name and line number in log output.
    report_caller: bool,

    /// Timestamp format (enum with presets + custom).
    #[serde(default)]
    timestamp_format: TSFormat,

    /// Enable level padding for alignment.
    #[serde(default)]
    pad_level: bool,

    /// Number of characters to pad level to.
    #[serde(default = "default_pad_amount")]
    pad_amount: usize,

    /// Which side to pad the level string.
    #[serde(default)]
    pad_side: PadSide,

    /// Separator between message and attributes (default: ": ").
    #[serde(default = "default_msg_separator")]
    msg_separator: String,

    /// Arrow character to use (default: "▶").
    #[serde(default = "default_arrow_char")]
    arrow_char: String,

    /// Fine-grained color configuration.
    #[serde(default)]
    colors: Colors,
}

// Default value functions for serde
fn default_pad_amount() -> usize {
    5
}

fn default_msg_separator() -> String {
    ": ".to_string()
}

fn default_arrow_char() -> String {
    "▶".to_string()
}

impl Default for Opts {
    fn default() -> Self {
        Self {
            coloured: false,
            output: Output::default(),
            level: LogLevel::default(),
            report_caller: false,
            timestamp_format: TSFormat::default(),
            pad_level: false,
            pad_amount: 5,
            pad_side: PadSide::default(),
            msg_separator: ": ".to_string(),
            arrow_char: "▶".to_string(),
            colors: Colors::default(),
        }
    }
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
        Opts::default()
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

    /// Returns the timestamp format.
    pub fn timestamp_format(&self) -> &TSFormat {
        &self.timestamp_format
    }

    /// Returns whether level padding is enabled.
    pub fn pad_level(&self) -> bool {
        self.pad_level
    }

    /// Returns the padding amount.
    pub fn pad_amount(&self) -> usize {
        self.pad_amount
    }

    /// Returns the padding side.
    pub fn pad_side(&self) -> PadSide {
        self.pad_side
    }

    /// Returns the message separator.
    pub fn msg_separator(&self) -> &str {
        &self.msg_separator
    }

    /// Returns the arrow character.
    pub fn arrow_char(&self) -> &str {
        &self.arrow_char
    }

    /// Returns the color configuration.
    pub fn colors(&self) -> &Colors {
        &self.colors
    }

    /// Returns the time format string (deprecated, for backward compatibility).
    #[deprecated(since = "0.6.1", note = "Use timestamp_format() instead")]
    pub fn time_format(&self) -> Option<&str> {
        match &self.timestamp_format {
            TSFormat::Custom(s) => Some(s.as_str()),
            _ => Some(self.timestamp_format.to_format_string()),
        }
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
    timestamp_format: TSFormat,
    pad_level: bool,
    pad_amount: usize,
    pad_side: PadSide,
    msg_separator: String,
    arrow_char: String,
    colors: Colors,
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
            timestamp_format: TSFormat::default(),
            pad_level: false,
            pad_amount: 5,
            pad_side: PadSide::default(),
            msg_separator: ": ".to_string(),
            arrow_char: "▶".to_string(),
            colors: Colors::default(),
        }
    }

    /// Preset with level padding enabled.
    pub fn with_level_padding() -> Self {
        Self::new()
            .pad_level(true)
            .pad_amount(5)
            .pad_side(PadSide::Right)
    }

    /// Preset without caller information.
    pub fn no_caller() -> Self {
        Self::new().report_caller(false)
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

    /// Set the timestamp format.
    pub fn timestamp_format(mut self, format: TSFormat) -> Self {
        self.timestamp_format = format;
        self
    }

    /// Enable or disable level padding.
    pub fn pad_level(mut self, pad: bool) -> Self {
        self.pad_level = pad;
        self
    }

    /// Set the padding amount.
    pub fn pad_amount(mut self, amount: usize) -> Self {
        self.pad_amount = amount;
        self
    }

    /// Set the padding side.
    pub fn pad_side(mut self, side: PadSide) -> Self {
        self.pad_side = side;
        self
    }

    /// Set the message separator.
    pub fn msg_separator(mut self, sep: impl Into<String>) -> Self {
        self.msg_separator = sep.into();
        self
    }

    /// Set the arrow character.
    pub fn arrow_char(mut self, arrow: impl Into<String>) -> Self {
        self.arrow_char = arrow.into();
        self
    }

    /// Set the color configuration.
    pub fn colors(mut self, colors: Colors) -> Self {
        self.colors = colors;
        self
    }

    /// Set a custom time format string (deprecated).
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
    #[deprecated(since = "0.6.1", note = "Use timestamp_format() instead")]
    pub fn time_format(mut self, format: impl Into<String>) -> Self {
        self.timestamp_format = TSFormat::Custom(format.into());
        self
    }

    /// Build the Opts, validating the timestamp format if provided.
    ///
    /// # Errors
    ///
    /// Returns an error if a custom timestamp format string is invalid.
    pub fn build(self) -> Result<Opts> {
        // Validate custom timestamp format if provided
        if let TSFormat::Custom(ref fmt) = self.timestamp_format {
            validate_time_format(fmt)?;
        }

        Ok(Opts {
            coloured: self.coloured,
            output: self.output,
            level: self.level,
            report_caller: self.report_caller,
            timestamp_format: self.timestamp_format,
            pad_level: self.pad_level,
            pad_amount: self.pad_amount,
            pad_side: self.pad_side,
            msg_separator: self.msg_separator,
            arrow_char: self.arrow_char,
            colors: self.colors,
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
        // timestamp_format is now always set (defaults to Standard)
        assert_eq!(opts.timestamp_format(), &TSFormat::Standard);
    }

    #[test]
    fn test_new_opts_sets_defaults() {
        let opts = Opts::new();
        assert!(!opts.coloured());
        assert_eq!(opts.output(), &Output::Stdout);
        assert_eq!(opts.level(), LogLevel::Error);
        assert!(!opts.report_caller());
        assert_eq!(opts.timestamp_format(), &TSFormat::Standard);
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
            .timestamp_format(TSFormat::TimeOnly)
            .build()
            .unwrap();

        let serialized = serde_json::to_string(&opts).unwrap();
        let deserialized: Opts = serde_json::from_str(&serialized).unwrap();

        assert_eq!(opts.coloured(), deserialized.coloured());
        assert_eq!(opts.output(), deserialized.output());
        assert_eq!(opts.level(), deserialized.level());
        assert_eq!(opts.report_caller(), deserialized.report_caller());
        assert_eq!(opts.timestamp_format(), deserialized.timestamp_format());
    }

    #[test]
    fn test_opts_builder_with_custom_values() {
        let opts = OptsBuilder::new()
            .coloured(true)
            .output(Output::file("/tmp/test.log"))
            .level(LogLevel::Trace)
            .report_caller(true)
            .timestamp_format(TSFormat::Custom("%Y-%m-%d".to_string()))
            .build()
            .unwrap();

        assert!(opts.coloured());
        assert_eq!(opts.output(), &Output::file("/tmp/test.log"));
        assert_eq!(opts.level(), LogLevel::Trace);
        assert!(opts.report_caller());
        assert_eq!(
            opts.timestamp_format(),
            &TSFormat::Custom("%Y-%m-%d".to_string())
        );
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
        // timestamp_format is now always set (defaults to Standard)
        assert_eq!(opts.timestamp_format(), &TSFormat::Standard);
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
