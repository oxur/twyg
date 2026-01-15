//! Logger implementation using fern.
//!
//! This module provides the [`Logger`] struct that wraps fern configuration.

use std::fmt::Arguments;

use chrono::Local;
use log::{Level, LevelFilter};
use owo_colors::{OwoColorize, Stream};
use serde::{Deserialize, Serialize};

use super::error::Result;
use super::level::LogLevel;
use super::opts::Opts;
use super::output::Output;

const DEFAULT_TS_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Logger {
    opts: Opts,
}

impl Logger {
    pub fn new(opts: Opts) -> Logger {
        owo_colors::set_override(opts.coloured());
        Logger { opts }
    }

    pub fn dispatch(&self) -> Result<fern::Dispatch> {
        let filter = self.level_to_filter();
        let mut dispatch = if self.opts.report_caller() {
            report_caller_logger(self.format_ts(), filter, self.stream())
        } else {
            logger(self.format_ts(), filter, self.stream())
        };

        dispatch = match self.opts.output() {
            Output::Stdout => dispatch.chain(std::io::stdout()),
            Output::Stderr => dispatch.chain(std::io::stderr()),
            Output::File(path) => dispatch.chain(fern::log_file(path)?),
        };

        Ok(dispatch)
    }

    // Private methods

    fn format_ts(&self) -> String {
        let ts = self.opts.time_format().unwrap_or(DEFAULT_TS_FORMAT);
        Local::now().format(ts).to_string()
    }

    pub fn level(&self) -> LogLevel {
        self.opts.level()
    }

    fn level_to_filter(&self) -> LevelFilter {
        self.opts.level().into()
    }

    fn stream(&self) -> Stream {
        Stream::from(self.opts.output())
    }
}

// Private functions

fn report_caller_logger(date: String, filter: LevelFilter, stream: Stream) -> fern::Dispatch {
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{} {} [{} {}] {}",
                date.if_supports_color(stream, |x| x.green()),
                colour_level(record.level(), stream),
                format_args!(
                    "{}:{}",
                    opt_str_or_placeholder(record.file()),
                    opt_u32_or_placeholder(record.line()),
                )
                .to_string()
                .if_supports_color(stream, |x| x.bright_yellow()),
                record
                    .target()
                    .if_supports_color(stream, |x| x.bright_yellow()),
                format_msg(message, stream).if_supports_color(stream, |x| x.bright_green())
            ))
        })
        .level(filter)
}

fn logger(date: String, filter: LevelFilter, stream: Stream) -> fern::Dispatch {
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{} {} [{}] {}",
                date.if_supports_color(stream, |x| x.green()),
                colour_level(record.level(), stream),
                record
                    .target()
                    .if_supports_color(stream, |x| x.bright_yellow()),
                format_msg(message, stream).if_supports_color(stream, |x| x.bright_green())
            ))
        })
        .level(filter)
}

fn opt_str_or_placeholder(x: Option<&str>) -> &str {
    x.unwrap_or("??")
}

fn opt_u32_or_placeholder(x: Option<u32>) -> std::borrow::Cow<'static, str> {
    match x {
        None => std::borrow::Cow::Borrowed("??"),
        Some(val) => std::borrow::Cow::Owned(val.to_string()),
    }
}

fn format_msg(msg: &Arguments<'_>, stream: Stream) -> String {
    format!("{} {}", "▶".if_supports_color(stream, |x| x.cyan()), msg)
        .if_supports_color(stream, |x| x.green())
        .to_string()
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
    fn test_format_ts() {
        let opts = OptsBuilder::new().time_format("%Y-%m-%d").build().unwrap();
        let logger = Logger::new(opts);
        let ts = logger.format_ts();
        // Should be in YYYY-MM-DD format
        assert!(ts.len() >= 10);
        assert!(ts.contains('-'));
    }

    #[test]
    fn test_format_ts_with_default() {
        let opts = Opts::default();
        let logger = Logger::new(opts);
        let ts = logger.format_ts();
        // Should use default format
        assert!(!ts.is_empty());
    }

    #[test]
    fn test_stream_stdout() {
        let opts = OptsBuilder::new().output(Output::Stdout).build().unwrap();
        let logger = Logger::new(opts);
        let stream = logger.stream();
        match stream {
            Stream::Stdout => {}
            _ => panic!("Expected Stream::Stdout"),
        }
    }

    #[test]
    fn test_stream_stderr() {
        let opts = OptsBuilder::new().output(Output::Stderr).build().unwrap();
        let logger = Logger::new(opts);
        let stream = logger.stream();
        match stream {
            Stream::Stderr => {}
            _ => panic!("Expected Stream::Stderr"),
        }
    }

    #[test]
    fn test_stream_file_uses_stdout() {
        let opts = OptsBuilder::new()
            .output(Output::file("/tmp/test.log"))
            .build()
            .unwrap();
        let logger = Logger::new(opts);
        let stream = logger.stream();
        match stream {
            Stream::Stdout => {}
            _ => panic!("Expected Stream::Stdout for file output"),
        }
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
    fn test_format_msg() {
        let args = format_args!("test message");
        let result = format_msg(&args, Stream::Stdout);
        assert!(result.contains("▶"));
        assert!(result.contains("test message"));
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
    fn test_level_to_filter() {
        let opts = OptsBuilder::new().level(LogLevel::Debug).build().unwrap();
        let logger = Logger::new(opts);
        let filter = logger.level_to_filter();
        assert_eq!(filter, LevelFilter::Debug);
    }

    #[test]
    fn test_level() {
        let opts = OptsBuilder::new().level(LogLevel::Info).build().unwrap();
        let logger = Logger::new(opts);
        assert_eq!(logger.level(), LogLevel::Info);
    }

    #[test]
    fn test_dispatch_with_stdout() {
        let opts = OptsBuilder::new()
            .output(Output::Stdout)
            .level(LogLevel::Debug)
            .build()
            .unwrap();
        let logger = Logger::new(opts);
        let result = logger.dispatch();
        assert!(result.is_ok());
    }

    #[test]
    fn test_dispatch_with_stderr() {
        let opts = OptsBuilder::new()
            .output(Output::Stderr)
            .level(LogLevel::Info)
            .build()
            .unwrap();
        let logger = Logger::new(opts);
        let result = logger.dispatch();
        assert!(result.is_ok());
    }

    #[test]
    fn test_dispatch_with_valid_file() {
        use std::env;
        let temp_file = env::temp_dir().join("twyg-test-dispatch.log");
        let opts = OptsBuilder::new()
            .output(Output::File(temp_file.clone()))
            .level(LogLevel::Trace)
            .build()
            .unwrap();
        let logger = Logger::new(opts);
        let result = logger.dispatch();
        assert!(result.is_ok());
        // Clean up
        let _ = std::fs::remove_file(temp_file);
    }

    #[test]
    fn test_dispatch_with_invalid_file_path() {
        let opts = OptsBuilder::new()
            .output(Output::file("/proc/invalid/nonexistent/path/test.log"))
            .level(LogLevel::Debug)
            .build()
            .unwrap();
        let logger = Logger::new(opts);
        let result = logger.dispatch();
        assert!(result.is_err());
    }

    #[test]
    fn test_dispatch_with_report_caller() {
        let opts = OptsBuilder::new()
            .report_caller(true)
            .level(LogLevel::Trace)
            .build()
            .unwrap();
        let logger = Logger::new(opts);
        let result = logger.dispatch();
        assert!(result.is_ok());
    }

    #[test]
    fn test_dispatch_with_coloured_and_caller() {
        let opts = OptsBuilder::new()
            .coloured(true)
            .report_caller(true)
            .output(Output::Stderr)
            .level(LogLevel::Warn)
            .build()
            .unwrap();
        let logger = Logger::new(opts);
        let result = logger.dispatch();
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_msg_with_stderr() {
        let args = format_args!("stderr message");
        let result = format_msg(&args, Stream::Stderr);
        assert!(result.contains("▶"));
        assert!(result.contains("stderr message"));
    }

    #[test]
    fn test_format_msg_empty() {
        let args = format_args!("");
        let result = format_msg(&args, Stream::Stdout);
        assert!(result.contains("▶"));
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
    fn test_logger_functions_create_dispatch() {
        // Test that logger() function creates a valid dispatch
        let dispatch = logger("2024-01-01".to_string(), LevelFilter::Info, Stream::Stdout);
        // Just verify it was created successfully (Dispatch doesn't expose level getter)
        let _ = dispatch;
    }

    #[test]
    fn test_report_caller_logger_creates_dispatch() {
        // Test that report_caller_logger() function creates a valid dispatch
        let dispatch =
            report_caller_logger("2024-01-01".to_string(), LevelFilter::Debug, Stream::Stderr);
        // Just verify it was created successfully (Dispatch doesn't expose level getter)
        let _ = dispatch;
    }

    #[test]
    fn test_all_log_levels_to_filter() {
        let levels = vec![
            (LogLevel::Trace, LevelFilter::Trace),
            (LogLevel::Debug, LevelFilter::Debug),
            (LogLevel::Info, LevelFilter::Info),
            (LogLevel::Warn, LevelFilter::Warn),
            (LogLevel::Error, LevelFilter::Error),
        ];

        for (log_level, expected_filter) in levels {
            let opts = OptsBuilder::new().level(log_level).build().unwrap();
            let logger = Logger::new(opts);
            assert_eq!(logger.level_to_filter(), expected_filter);
        }
    }

    #[test]
    fn test_dispatch_all_combinations() {
        // Test various combinations of options to exercise more code paths
        let test_cases = vec![
            (true, true, LogLevel::Trace),
            (true, false, LogLevel::Debug),
            (false, true, LogLevel::Info),
            (false, false, LogLevel::Warn),
        ];

        for (coloured, report_caller, level) in test_cases {
            let opts = OptsBuilder::new()
                .coloured(coloured)
                .report_caller(report_caller)
                .level(level)
                .build()
                .unwrap();
            let logger = Logger::new(opts);
            let result = logger.dispatch();
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_custom_time_formats() {
        let time_formats = vec![
            Some("%Y-%m-%d".to_string()),
            Some("%H:%M:%S".to_string()),
            Some("%Y-%m-%d %H:%M:%S%.3f".to_string()),
            None,
        ];

        for time_format in time_formats {
            let opts = match time_format {
                Some(fmt) => OptsBuilder::new()
                    .time_format(fmt)
                    .level(LogLevel::Debug)
                    .build()
                    .unwrap(),
                None => OptsBuilder::new().level(LogLevel::Debug).build().unwrap(),
            };
            let logger = Logger::new(opts);
            let ts = logger.format_ts();
            assert!(!ts.is_empty());
        }
    }

    #[test]
    fn test_dispatch_with_all_outputs() {
        use std::env;
        let temp_file = env::temp_dir().join("twyg-test-all-outputs.log");

        let outputs = vec![
            Output::Stdout,
            Output::Stderr,
            Output::File(temp_file.clone()),
        ];

        for output in outputs {
            let opts = OptsBuilder::new()
                .output(output.clone())
                .level(LogLevel::Trace)
                .report_caller(true)
                .coloured(true)
                .build()
                .unwrap();
            let logger = Logger::new(opts);
            let result = logger.dispatch();
            assert!(result.is_ok());
        }

        // Clean up
        let _ = std::fs::remove_file(temp_file);
    }
}
