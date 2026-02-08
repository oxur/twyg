use log::{debug, error, info, trace, warn};
use serde::Deserialize;
use twyg::{LogLevel, Logger, Opts, OptsBuilder, Output, TSFormat};

/// Test that setup works with default options and actually write log messages
/// to exercise the formatting closures.
/// This is the only test that actually initializes the global logger.
#[test]
fn test_setup_with_defaults() {
    let opts = OptsBuilder::new()
        .level(LogLevel::Trace)
        .report_caller(true)
        .coloured(false)
        .build()
        .unwrap();
    let result = twyg::setup(opts);
    assert!(result.is_ok());

    // Write log messages at all levels to exercise the formatting closures
    trace!("This is a trace message");
    debug!("This is a debug message");
    info!("This is an info message");
    warn!("This is a warning message");
    error!("This is an error message");

    // Test with different message formats
    trace!("Message with {} formatting", "args");
    debug!("Number: {}, String: {}", 42, "test");
    info!("Boolean: {}, Float: {:.2}", true, 12.34);
}

// The remaining tests verify that Logger can be created and configured
// without actually initializing the global logger (which can only be done once).
// We verify logger configuration but don't call dispatch() since the global
// logger can only be initialized once per process.

#[test]
fn test_logger_with_trace_level() {
    let opts = OptsBuilder::new().level(LogLevel::Trace).build().unwrap();
    let logger = Logger::new(opts.clone());
    assert_eq!(logger.level(), LogLevel::Trace);
    assert_eq!(opts.level(), LogLevel::Trace);
}

#[test]
fn test_logger_with_debug_level() {
    let opts = OptsBuilder::new().level(LogLevel::Debug).build().unwrap();
    let logger = Logger::new(opts.clone());
    assert_eq!(logger.level(), LogLevel::Debug);
    assert_eq!(opts.level(), LogLevel::Debug);
}

#[test]
fn test_logger_with_info_level() {
    let opts = OptsBuilder::new().level(LogLevel::Info).build().unwrap();
    let logger = Logger::new(opts.clone());
    assert_eq!(logger.level(), LogLevel::Info);
    assert_eq!(opts.level(), LogLevel::Info);
}

#[test]
fn test_logger_with_warn_level() {
    let opts = OptsBuilder::new().level(LogLevel::Warn).build().unwrap();
    let logger = Logger::new(opts.clone());
    assert_eq!(logger.level(), LogLevel::Warn);
    assert_eq!(opts.level(), LogLevel::Warn);
}

#[test]
fn test_logger_with_error_level() {
    let opts = OptsBuilder::new().level(LogLevel::Error).build().unwrap();
    let logger = Logger::new(opts.clone());
    assert_eq!(logger.level(), LogLevel::Error);
    assert_eq!(opts.level(), LogLevel::Error);
}

#[test]
fn test_logger_with_coloured() {
    let opts = OptsBuilder::new()
        .coloured(true)
        .level(LogLevel::Debug)
        .build()
        .unwrap();
    let logger = Logger::new(opts.clone());
    assert_eq!(logger.level(), LogLevel::Debug);
    assert!(opts.coloured());
}

#[test]
fn test_logger_with_caller() {
    let opts = OptsBuilder::new()
        .report_caller(true)
        .level(LogLevel::Debug)
        .build()
        .unwrap();
    let logger = Logger::new(opts.clone());
    assert_eq!(logger.level(), LogLevel::Debug);
    assert!(opts.report_caller());
}

#[test]
fn test_logger_with_stdout() {
    let opts = OptsBuilder::new()
        .output(Output::Stdout)
        .level(LogLevel::Debug)
        .build()
        .unwrap();
    let logger = Logger::new(opts.clone());
    assert_eq!(logger.level(), LogLevel::Debug);
    assert_eq!(opts.output(), &Output::Stdout);
}

#[test]
fn test_logger_with_stderr() {
    let opts = OptsBuilder::new()
        .output(Output::Stderr)
        .level(LogLevel::Debug)
        .build()
        .unwrap();
    let logger = Logger::new(opts.clone());
    assert_eq!(logger.level(), LogLevel::Debug);
    assert_eq!(opts.output(), &Output::Stderr);
}

#[test]
fn test_logger_with_custom_time_format() {
    let opts = OptsBuilder::new()
        .timestamp_format(TSFormat::TimeOnly)
        .level(LogLevel::Debug)
        .build()
        .unwrap();
    let logger = Logger::new(opts.clone());
    assert_eq!(logger.level(), LogLevel::Debug);
    assert_eq!(opts.timestamp_format(), &TSFormat::TimeOnly);
}

#[test]
fn test_logger_with_all_options() {
    let opts = OptsBuilder::new()
        .coloured(true)
        .output(Output::Stdout)
        .level(LogLevel::Trace)
        .report_caller(true)
        .timestamp_format(TSFormat::Standard)
        .build()
        .unwrap();
    let logger = Logger::new(opts.clone());
    assert_eq!(logger.level(), LogLevel::Trace);
    assert!(opts.coloured());
    assert_eq!(opts.output(), &Output::Stdout);
    assert!(opts.report_caller());
    assert_eq!(opts.timestamp_format(), &TSFormat::Standard);
}

#[test]
fn test_opts_new() {
    let opts = Opts::new();
    assert!(!opts.coloured());
    assert_eq!(opts.output(), &Output::Stdout);
    assert_eq!(opts.level(), LogLevel::Error);
    assert!(!opts.report_caller());
    assert_eq!(opts.timestamp_format(), &TSFormat::Standard);
}

#[test]
fn test_log_level_parsing() {
    assert_eq!("trace".parse::<LogLevel>().unwrap(), LogLevel::Trace);
    assert_eq!("debug".parse::<LogLevel>().unwrap(), LogLevel::Debug);
    assert_eq!("info".parse::<LogLevel>().unwrap(), LogLevel::Info);
    assert_eq!("warn".parse::<LogLevel>().unwrap(), LogLevel::Warn);
    assert_eq!("error".parse::<LogLevel>().unwrap(), LogLevel::Error);

    // Case insensitive
    assert_eq!("TRACE".parse::<LogLevel>().unwrap(), LogLevel::Trace);
    assert_eq!("DEBUG".parse::<LogLevel>().unwrap(), LogLevel::Debug);

    // Invalid should error
    assert!("invalid_level".parse::<LogLevel>().is_err());
}

#[test]
fn test_output_parsing() {
    assert_eq!("stdout".parse::<Output>().unwrap(), Output::Stdout);
    assert_eq!("stderr".parse::<Output>().unwrap(), Output::Stderr);

    // File paths
    let file_output = "/tmp/test.log".parse::<Output>().unwrap();
    assert!(file_output.is_file());
    assert_eq!(
        file_output.file_path().unwrap().to_str().unwrap(),
        "/tmp/test.log"
    );
}

/// Verifies that a partial TOML config (e.g. an application `[logging]` section)
/// deserializes into `Opts` with missing fields filled by defaults.
#[test]
fn test_opts_partial_toml_logging_section_uses_defaults() {
    #[derive(Deserialize)]
    struct AppConfig {
        logging: Opts,
    }

    let toml_str = r#"
[logging]
level = "debug"
coloured = true
"#;

    let config: AppConfig = toml::from_str(toml_str).unwrap();
    let opts = config.logging;

    // Explicitly set fields should have the provided values.
    assert_eq!(opts.level(), LogLevel::Debug);
    assert!(opts.coloured());

    // All missing fields should fall back to their defaults.
    assert_eq!(opts.output(), &Output::Stdout);
    assert!(!opts.report_caller());
    assert_eq!(opts.timestamp_format(), &TSFormat::Standard);
}
