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
use owo_colors::{OwoColorize, Stream};
use serde::{Deserialize, Serialize};

use super::error::Result;
use super::level::LogLevel;
use super::opts::Opts;
use super::output::Output;

const DEFAULT_TS_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

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
    time_format: String,
    report_caller: bool,
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
        let time_format = opts
            .time_format()
            .unwrap_or(DEFAULT_TS_FORMAT)
            .to_string();
        let report_caller = opts.report_caller();

        TwygLogger {
            output: Arc::new(Mutex::new(output)),
            config: LoggerConfig {
                stream,
                max_level,
                time_format,
                report_caller,
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
        let timestamp = Local::now().format(&self.config.time_format);
        let level = colour_level(record.level(), self.config.stream);
        let target = record.target();
        let message = record.args();

        // Extract key-value pairs for structured logging
        let mut kv_collector = KeyValueCollector::new();
        let _ = record.key_values().visit(&mut kv_collector);

        // Use write!() for zero-copy formatting (fern pattern)
        if self.config.report_caller {
            write!(
                writer,
                "{} {} [{} {}] {} {}{}",
                timestamp.if_supports_color(self.config.stream, |x| x.green()),
                level,
                format_args!(
                    "{}:{}",
                    opt_str_or_placeholder(record.file()),
                    opt_u32_or_placeholder(record.line()),
                )
                .if_supports_color(self.config.stream, |x| x.bright_yellow()),
                target.if_supports_color(self.config.stream, |x| x.bright_yellow()),
                "▶".if_supports_color(self.config.stream, |x| x.cyan()),
                message.if_supports_color(self.config.stream, |x| x.green()),
                kv_collector.format_pairs(self.config.stream)
            )?;
        } else {
            write!(
                writer,
                "{} {} [{}] {} {}{}",
                timestamp.if_supports_color(self.config.stream, |x| x.green()),
                level,
                target.if_supports_color(self.config.stream, |x| x.bright_yellow()),
                "▶".if_supports_color(self.config.stream, |x| x.cyan()),
                message.if_supports_color(self.config.stream, |x| x.green()),
                kv_collector.format_pairs(self.config.stream)
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

    fn format_pairs(&self, stream: Stream) -> String {
        if self.pairs.is_empty() {
            return String::new();
        }

        let formatted = self
            .pairs
            .iter()
            .map(|(k, v)| {
                format!(
                    "{}={}",
                    k.if_supports_color(stream, |x| x.bright_yellow()),
                    format!("{{{}}}", v).if_supports_color(stream, |x| x.cyan())
                )
            })
            .collect::<Vec<_>>()
            .join(", ");

        format!(": {}", formatted)
    }
}

impl<'kvs> VisitSource<'kvs> for KeyValueCollector {
    fn visit_pair(&mut self, key: Key<'kvs>, value: Value<'kvs>) -> std::result::Result<(), log::kv::Error> {
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

fn colour_level(level: Level, stream: Stream) -> String {
    let s_level = level.to_string();
    match level {
        Level::Error => s_level.if_supports_color(stream, |x| x.red()).to_string(),
        Level::Warn => s_level
            .if_supports_color(stream, |x| x.bright_yellow())
            .to_string(),
        Level::Info => s_level
            .if_supports_color(stream, |x| x.bright_green())
            .to_string(),
        Level::Debug => s_level.if_supports_color(stream, |x| x.cyan()).to_string(),
        Level::Trace => s_level
            .if_supports_color(stream, |x| x.bright_blue())
            .to_string(),
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
        log::set_boxed_logger(Box::new(logger))
            .map_err(|_| super::error::TwygError::InitError)?;
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
    fn test_colour_level_error() {
        let colored = colour_level(Level::Error, Stream::Stdout);
        assert!(colored.contains("ERROR") || colored.contains("error"));
    }

    #[test]
    fn test_colour_level_warn() {
        let colored = colour_level(Level::Warn, Stream::Stdout);
        assert!(colored.contains("WARN") || colored.contains("warn"));
    }

    #[test]
    fn test_colour_level_info() {
        let colored = colour_level(Level::Info, Stream::Stdout);
        assert!(colored.contains("INFO") || colored.contains("info"));
    }

    #[test]
    fn test_colour_level_debug() {
        let colored = colour_level(Level::Debug, Stream::Stdout);
        assert!(colored.contains("DEBUG") || colored.contains("debug"));
    }

    #[test]
    fn test_colour_level_trace() {
        let colored = colour_level(Level::Trace, Stream::Stdout);
        assert!(colored.contains("TRACE") || colored.contains("trace"));
    }

    #[test]
    fn test_colour_level_with_stderr() {
        let error = colour_level(Level::Error, Stream::Stderr);
        assert!(error.contains("ERROR") || error.contains("error"));

        let warn = colour_level(Level::Warn, Stream::Stderr);
        assert!(warn.contains("WARN") || warn.contains("warn"));

        let info = colour_level(Level::Info, Stream::Stderr);
        assert!(info.contains("INFO") || info.contains("info"));

        let debug = colour_level(Level::Debug, Stream::Stderr);
        assert!(debug.contains("DEBUG") || debug.contains("debug"));

        let trace = colour_level(Level::Trace, Stream::Stderr);
        assert!(trace.contains("TRACE") || trace.contains("trace"));
    }

    #[test]
    fn test_kv_collector_empty() {
        let collector = KeyValueCollector::new();
        assert!(collector.is_empty());
        assert_eq!(collector.format_pairs(Stream::Stdout), "");
    }

    #[test]
    fn test_kv_collector_format_pairs() {
        let mut collector = KeyValueCollector::new();
        collector.pairs.push(("user".to_string(), "alice".to_string()));
        collector.pairs.push(("action".to_string(), "login".to_string()));

        let formatted = collector.format_pairs(Stream::Stdout);
        assert!(formatted.contains("user="));
        assert!(formatted.contains("{alice}"));
        assert!(formatted.contains("action="));
        assert!(formatted.contains("{login}"));
        assert!(formatted.starts_with(": "));
    }

    #[test]
    fn test_kv_collector_single_pair() {
        let mut collector = KeyValueCollector::new();
        collector.pairs.push(("key".to_string(), "value".to_string()));

        let formatted = collector.format_pairs(Stream::Stdout);
        assert!(formatted.contains("key={value}"));
        assert!(formatted.starts_with(": "));
    }
}
