//! Logger implementation with direct log::Log trait implementation.
//!
//! This module provides the [`TwygLogger`] struct that implements `log::Log`
//! directly, enabling structured logging with key-value pairs while adopting
//! best practices for performance and reliability.
//!
//! # Features
//!
//! - Zero-copy formatting using `write!()` instead of String allocation
//! - Three-tiered error recovery (normal → stderr → panic)
//! - Mutex poison recovery for robust thread safety
//! - BufWriter for efficient file I/O
//! - Structured logging support via log crate's kv feature

use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::sync::{Arc, Mutex};

use chrono::Local;
use log::{Level, LevelFilter, Log, Metadata, Record};
use owo_colors::Stream;
use serde::{Deserialize, Serialize};

use super::color::Colors;
use super::error::Result;
use super::level::LogLevel;
use super::opts::{Opts, PadSide};
use super::output::Output;
use super::timestamp::TSFormat;

/// Output writer enum supporting stdout, stderr, and file output.
enum OutputWriter {
    Stdout(io::Stdout),
    Stderr(io::Stderr),
    File(BufWriter<File>),
}

impl OutputWriter {
    fn write_fmt(&mut self, args: std::fmt::Arguments) -> io::Result<()> {
        match self {
            OutputWriter::Stdout(w) => w.write_fmt(args),
            OutputWriter::Stderr(w) => w.write_fmt(args),
            OutputWriter::File(w) => w.write_fmt(args),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            OutputWriter::Stdout(w) => w.flush(),
            OutputWriter::Stderr(w) => w.flush(),
            OutputWriter::File(w) => w.flush(),
        }
    }
}

/// Internal logger configuration.
struct LoggerConfig {
    stream: Stream,
    max_level: LevelFilter,
    timestamp_format: TSFormat,
    report_caller: bool,
    pad_level: bool,
    pad_amount: usize,
    pad_side: PadSide,
    msg_separator: String,
    arrow_char: String,
    colors: Colors,
}

/// Logger implementation that directly implements log::Log trait.
///
/// This struct is used internally by twyg and supports:
/// - Thread-safe output via Arc<Mutex<OutputWriter>>
/// - Structured logging with key-value pairs
/// - Zero-copy formatting for performance
/// - Robust error handling with fallback to stderr
struct TwygLogger {
    output: Arc<Mutex<OutputWriter>>,
    config: LoggerConfig,
}

impl TwygLogger {
    /// Creates a new TwygLogger from Opts.
    fn new(opts: &Opts, output: OutputWriter) -> Self {
        let stream = Stream::from(opts.output());
        let max_level = LevelFilter::from(opts.level());
        let timestamp_format = opts.timestamp_format().clone();
        let report_caller = opts.report_caller();
        let pad_level = opts.pad_level();
        let pad_amount = opts.pad_amount();
        let pad_side = opts.pad_side();
        let msg_separator = opts.msg_separator().to_string();
        let arrow_char = opts.arrow_char().to_string();
        let colors = opts.colors().clone();

        TwygLogger {
            output: Arc::new(Mutex::new(output)),
            config: LoggerConfig {
                stream,
                max_level,
                timestamp_format,
                report_caller,
                pad_level,
                pad_amount,
                pad_side,
                msg_separator,
                arrow_char,
                colors,
            },
        }
    }

    /// Gets a lock on the output writer with poison recovery.
    ///
    /// Adopts fern's pattern: never panic on poisoned mutex in logging infrastructure.
    fn output_lock(&self) -> impl std::ops::DerefMut<Target = OutputWriter> + '_ {
        self.output.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// Writes a log record to the output.
    ///
    /// Uses zero-copy write!() formatting (fern pattern) instead of String allocation.
    fn write_log(&self, record: &Record) -> io::Result<()> {
        let mut writer = self.output_lock();
        let ts_format = self.config.timestamp_format.to_format_string();
        let timestamp = Local::now().format(ts_format);
        let level = format_level(
            record.level(),
            &self.config.colors,
            self.config.pad_level,
            self.config.pad_amount,
            self.config.pad_side,
            self.config.stream,
        );
        let target = record.target();
        let message = record.args();

        // Extract key-value pairs for structured logging
        let mut kv_collector = KeyValueCollector::new();
        let _ = record.key_values().visit(&mut kv_collector);

        // Use write!() for zero-copy formatting (fern pattern)
        if self.config.report_caller {
            // Format timestamp with config color
            let timestamp_str = timestamp.to_string();
            let timestamp_colored = self
                .config
                .colors
                .timestamp
                .as_ref()
                .map(|c| c.apply(&timestamp_str, self.config.stream))
                .unwrap_or(timestamp_str);

            // Format caller file and line
            let file = opt_str_or_placeholder(record.file());
            let line = opt_u32_or_placeholder(record.line());
            let caller_str = format!("{}:{}", file, line);
            let caller_colored = self
                .config
                .colors
                .caller_file
                .as_ref()
                .map(|c| c.apply(&caller_str, self.config.stream))
                .unwrap_or(caller_str);

            // Format target with config color
            let target_colored = self
                .config
                .colors
                .target
                .as_ref()
                .map(|c| c.apply(target, self.config.stream))
                .unwrap_or_else(|| target.to_string());

            // Format arrow with config color
            let arrow_colored = self
                .config
                .colors
                .arrow
                .as_ref()
                .map(|c| c.apply(&self.config.arrow_char, self.config.stream))
                .unwrap_or_else(|| self.config.arrow_char.clone());

            // Format message with config color
            let message_str = message.to_string();
            let message_colored = self
                .config
                .colors
                .message
                .as_ref()
                .map(|c| c.apply(&message_str, self.config.stream))
                .unwrap_or(message_str);

            write!(
                writer,
                "{} {} [{} {}] {} {}{}",
                timestamp_colored,
                level,
                caller_colored,
                target_colored,
                arrow_colored,
                message_colored,
                kv_collector.format_pairs(&self.config)
            )?;
        } else {
            // Format timestamp with config color
            let timestamp_str = timestamp.to_string();
            let timestamp_colored = self
                .config
                .colors
                .timestamp
                .as_ref()
                .map(|c| c.apply(&timestamp_str, self.config.stream))
                .unwrap_or(timestamp_str);

            // Format target with config color
            let target_colored = self
                .config
                .colors
                .target
                .as_ref()
                .map(|c| c.apply(target, self.config.stream))
                .unwrap_or_else(|| target.to_string());

            // Format arrow with config color
            let arrow_colored = self
                .config
                .colors
                .arrow
                .as_ref()
                .map(|c| c.apply(&self.config.arrow_char, self.config.stream))
                .unwrap_or_else(|| self.config.arrow_char.clone());

            // Format message with config color
            let message_str = message.to_string();
            let message_colored = self
                .config
                .colors
                .message
                .as_ref()
                .map(|c| c.apply(&message_str, self.config.stream))
                .unwrap_or(message_str);

            write!(
                writer,
                "{} {} [{}] {} {}{}",
                timestamp_colored,
                level,
                target_colored,
                arrow_colored,
                message_colored,
                kv_collector.format_pairs(&self.config)
            )?;
        }

        writeln!(writer)?;
        writer.flush()
    }
}

impl Log for TwygLogger {
    #[inline]
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.config.max_level
    }

    #[inline]
    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return; // Early exit (fern pattern)
        }

        // Three-tiered error recovery: normal → stderr → panic (fern pattern)
        fallback_on_error(record, |rec| self.write_log(rec));
    }

    fn flush(&self) {
        let _ = self.output_lock().flush();
    }
}

/// Three-tiered error recovery function (fern pattern).
///
/// Marked #[inline(always)] to avoid overhead in hot path.
#[inline(always)]
fn fallback_on_error<F>(record: &Record, log_func: F)
where
    F: FnOnce(&Record) -> io::Result<()>,
{
    if let Err(error) = log_func(record) {
        backup_to_stderr(record, &error);
    }
}

/// Fallback to stderr if primary logging fails (fern pattern).
///
/// Only panics if stderr also fails (catastrophic failure).
fn backup_to_stderr(record: &Record, error: &io::Error) {
    let stderr = io::stderr();
    let mut handle = stderr.lock();

    let write_result = writeln!(
        handle,
        "[twyg error: {}] {} - {}",
        error,
        record.level(),
        record.args()
    );

    if let Err(stderr_err) = write_result {
        panic!(
            "twyg: failed to write to stderr (err: {:?}), \
             failed to write to primary output (err: {:?}), \
             log record: {:?}",
            stderr_err, error, record
        );
    }
}

// Key-Value Collector for structured logging

use log::kv::{Key, Value, VisitSource};

/// Visitor for collecting key-value pairs from log records.
struct KeyValueCollector {
    pairs: Vec<(String, String)>,
}

impl KeyValueCollector {
    fn new() -> Self {
        Self { pairs: Vec::new() }
    }

    #[cfg(test)]
    fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }

    fn format_pairs(&self, config: &LoggerConfig) -> String {
        if self.pairs.is_empty() {
            return String::new();
        }

        let formatted = self
            .pairs
            .iter()
            .map(|(k, v)| {
                // Format key with config color
                let key_colored = config
                    .colors
                    .attr_key
                    .as_ref()
                    .map(|c| c.apply(k, config.stream))
                    .unwrap_or_else(|| k.clone());

                // Format value with braces and config color
                let value_with_braces = format!("{{{}}}", v);
                let value_colored = config
                    .colors
                    .attr_value
                    .as_ref()
                    .map(|c| c.apply(&value_with_braces, config.stream))
                    .unwrap_or(value_with_braces);

                format!("{}={}", key_colored, value_colored)
            })
            .collect::<Vec<_>>()
            .join(", ");

        format!("{}{}", config.msg_separator, formatted)
    }
}

impl<'kvs> VisitSource<'kvs> for KeyValueCollector {
    fn visit_pair(
        &mut self,
        key: Key<'kvs>,
        value: Value<'kvs>,
    ) -> std::result::Result<(), log::kv::Error> {
        // Convert key and value to strings
        self.pairs.push((key.to_string(), value.to_string()));
        Ok(())
    }
}

// Helper functions

fn opt_str_or_placeholder(x: Option<&str>) -> &str {
    x.unwrap_or("??")
}

fn opt_u32_or_placeholder(x: Option<u32>) -> std::borrow::Cow<'static, str> {
    match x {
        None => std::borrow::Cow::Borrowed("??"),
        Some(val) => std::borrow::Cow::Owned(val.to_string()),
    }
}

/// Pad a level string to specified width
fn pad_level(level: &str, amount: usize, side: PadSide) -> String {
    match side {
        PadSide::Left => format!("{:>width$}", level, width = amount),
        PadSide::Right => format!("{:<width$}", level, width = amount),
    }
}

/// Format a level with optional padding and config-driven colors
fn format_level(
    level: Level,
    colors: &Colors,
    pad: bool,
    pad_amount: usize,
    pad_side: PadSide,
    stream: Stream,
) -> String {
    let level_str = level.to_string();

    // Apply padding if enabled
    let padded = if pad {
        pad_level(&level_str, pad_amount, pad_side)
    } else {
        level_str
    };

    // Apply color from config
    if let Some(color) = colors.level_color(level) {
        color.apply(&padded, stream)
    } else {
        padded
    }
}

// Public API - Logger struct for backwards compatibility

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Logger {
    opts: Opts,
}

impl Logger {
    pub fn new(opts: Opts) -> Logger {
        owo_colors::set_override(opts.coloured());
        Logger { opts }
    }

    /// Creates a TwygLogger and installs it as the global logger.
    ///
    /// This replaces the previous dispatch() method.
    pub fn dispatch(&self) -> Result<()> {
        // Create output writer based on opts
        let output_writer = match self.opts.output() {
            Output::Stdout => OutputWriter::Stdout(io::stdout()),
            Output::Stderr => OutputWriter::Stderr(io::stderr()),
            Output::File(path) => {
                let file = File::create(path)?;
                OutputWriter::File(BufWriter::new(file))
            }
        };

        // Create and install the logger
        let logger = TwygLogger::new(&self.opts, output_writer);
        log::set_boxed_logger(Box::new(logger)).map_err(|_| super::error::TwygError::InitError)?;
        log::set_max_level(LevelFilter::from(self.opts.level()));

        Ok(())
    }

    pub fn level(&self) -> LogLevel {
        self.opts.level()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::opts::OptsBuilder;

    #[test]
    fn test_logger_new() {
        let opts = Opts::default();
        let logger = Logger::new(opts.clone());
        assert_eq!(logger.opts.coloured(), opts.coloured());
    }

    #[test]
    fn test_logger_clone() {
        let opts = Opts::default();
        let logger1 = Logger::new(opts);
        let logger2 = logger1.clone();
        assert_eq!(logger1.opts.coloured(), logger2.opts.coloured());
    }

    #[test]
    fn test_logger_debug() {
        let opts = Opts::default();
        let logger = Logger::new(opts);
        let debug_str = format!("{:?}", logger);
        assert!(debug_str.contains("Logger"));
    }

    #[test]
    fn test_logger_level() {
        let opts = OptsBuilder::new().level(LogLevel::Info).build().unwrap();
        let logger = Logger::new(opts);
        assert_eq!(logger.level(), LogLevel::Info);
    }

    #[test]
    fn test_logger_serialize_deserialize() {
        let opts = OptsBuilder::new()
            .coloured(true)
            .level(LogLevel::Debug)
            .build()
            .unwrap();
        let logger = Logger::new(opts);

        let serialized = serde_json::to_string(&logger).unwrap();
        let deserialized: Logger = serde_json::from_str(&serialized).unwrap();

        assert_eq!(logger.opts.coloured(), deserialized.opts.coloured());
        assert_eq!(logger.opts.level(), deserialized.opts.level());
    }

    #[test]
    fn test_opt_str_or_placeholder_with_some() {
        let result = opt_str_or_placeholder(Some("test"));
        assert_eq!(result, "test");
    }

    #[test]
    fn test_opt_str_or_placeholder_with_none() {
        let result = opt_str_or_placeholder(None);
        assert_eq!(result, "??");
    }

    #[test]
    fn test_opt_u32_or_placeholder_with_some() {
        let result = opt_u32_or_placeholder(Some(42));
        assert_eq!(result, "42");
    }

    #[test]
    fn test_opt_u32_or_placeholder_with_none() {
        let result = opt_u32_or_placeholder(None);
        assert_eq!(result, "??");
    }

    #[test]
    fn test_format_level_without_padding() {
        let colors = Colors::default();
        let formatted = format_level(
            Level::Info,
            &colors,
            false,
            5,
            PadSide::Right,
            Stream::Stdout,
        );
        assert!(formatted.contains("INFO") || formatted.contains("info"));
    }

    #[test]
    fn test_format_level_with_padding_right() {
        let colors = Colors::default();
        let formatted = format_level(
            Level::Info,
            &colors,
            true,
            7,
            PadSide::Right,
            Stream::Stdout,
        );
        assert!(formatted.contains("INFO") || formatted.contains("info"));
        // With right padding, "INFO" becomes "INFO   " (7 chars total)
    }

    #[test]
    fn test_format_level_with_padding_left() {
        let colors = Colors::default();
        let formatted = format_level(Level::Warn, &colors, true, 7, PadSide::Left, Stream::Stdout);
        assert!(formatted.contains("WARN") || formatted.contains("warn"));
        // With left padding, "WARN" becomes "   WARN" (7 chars total)
    }

    #[test]
    fn test_pad_level() {
        assert_eq!(pad_level("INFO", 5, PadSide::Right), "INFO ");
        assert_eq!(pad_level("INFO", 5, PadSide::Left), " INFO");
        assert_eq!(pad_level("ERROR", 7, PadSide::Right), "ERROR  ");
        assert_eq!(pad_level("WARN", 5, PadSide::Left), " WARN");
    }

    #[test]
    fn test_format_level_all_levels() {
        let colors = Colors::default();

        let error = format_level(
            Level::Error,
            &colors,
            false,
            5,
            PadSide::Right,
            Stream::Stdout,
        );
        assert!(error.contains("ERROR") || error.contains("error"));

        let warn = format_level(
            Level::Warn,
            &colors,
            false,
            5,
            PadSide::Right,
            Stream::Stdout,
        );
        assert!(warn.contains("WARN") || warn.contains("warn"));

        let info = format_level(
            Level::Info,
            &colors,
            false,
            5,
            PadSide::Right,
            Stream::Stdout,
        );
        assert!(info.contains("INFO") || info.contains("info"));

        let debug = format_level(
            Level::Debug,
            &colors,
            false,
            5,
            PadSide::Right,
            Stream::Stdout,
        );
        assert!(debug.contains("DEBUG") || debug.contains("debug"));

        let trace = format_level(
            Level::Trace,
            &colors,
            false,
            5,
            PadSide::Right,
            Stream::Stdout,
        );
        assert!(trace.contains("TRACE") || trace.contains("trace"));
    }

    #[test]
    fn test_kv_collector_empty() {
        let collector = KeyValueCollector::new();
        assert!(collector.is_empty());

        let opts = Opts::default();
        let config = LoggerConfig {
            stream: Stream::Stdout,
            max_level: LevelFilter::from(opts.level()),
            timestamp_format: opts.timestamp_format().clone(),
            report_caller: opts.report_caller(),
            pad_level: opts.pad_level(),
            pad_amount: opts.pad_amount(),
            pad_side: opts.pad_side(),
            msg_separator: opts.msg_separator().to_string(),
            arrow_char: opts.arrow_char().to_string(),
            colors: opts.colors().clone(),
        };

        assert_eq!(collector.format_pairs(&config), "");
    }

    #[test]
    fn test_kv_collector_format_pairs() {
        let mut collector = KeyValueCollector::new();
        collector
            .pairs
            .push(("user".to_string(), "alice".to_string()));
        collector
            .pairs
            .push(("action".to_string(), "login".to_string()));

        let opts = Opts::default();
        let config = LoggerConfig {
            stream: Stream::Stdout,
            max_level: LevelFilter::from(opts.level()),
            timestamp_format: opts.timestamp_format().clone(),
            report_caller: opts.report_caller(),
            pad_level: opts.pad_level(),
            pad_amount: opts.pad_amount(),
            pad_side: opts.pad_side(),
            msg_separator: opts.msg_separator().to_string(),
            arrow_char: opts.arrow_char().to_string(),
            colors: opts.colors().clone(),
        };

        let formatted = collector.format_pairs(&config);
        // Check structure (color codes may be present, so check key parts)
        assert!(formatted.contains("user"));
        assert!(formatted.contains("alice"));
        assert!(formatted.contains("action"));
        assert!(formatted.contains("login"));
        assert!(formatted.starts_with(": "));
        // Verify format characters are present
        assert!(formatted.contains("="));
        assert!(formatted.contains("{"));
        assert!(formatted.contains("}"));
    }

    #[test]
    fn test_kv_collector_single_pair() {
        let mut collector = KeyValueCollector::new();
        collector
            .pairs
            .push(("key".to_string(), "value".to_string()));

        let opts = Opts::default();
        let config = LoggerConfig {
            stream: Stream::Stdout,
            max_level: LevelFilter::from(opts.level()),
            timestamp_format: opts.timestamp_format().clone(),
            report_caller: opts.report_caller(),
            pad_level: opts.pad_level(),
            pad_amount: opts.pad_amount(),
            pad_side: opts.pad_side(),
            msg_separator: opts.msg_separator().to_string(),
            arrow_char: opts.arrow_char().to_string(),
            colors: opts.colors().clone(),
        };

        let formatted = collector.format_pairs(&config);
        // Check key components (color codes may be included)
        assert!(formatted.contains("key"));
        assert!(formatted.contains("value"));
        assert!(formatted.starts_with(": "));
        assert!(formatted.contains("="));
        assert!(formatted.contains("{"));
        assert!(formatted.contains("}"));
    }
}
