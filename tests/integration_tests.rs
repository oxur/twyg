use twyg::{LogLevel, Opts, Output};

#[test]
fn test_setup_with_defaults() {
    let opts = Opts::default();
    let result = twyg::setup(opts);
    assert!(result.is_ok());
}

#[test]
fn test_setup_with_trace_level() {
    let opts = Opts {
        level: LogLevel::Trace,
        ..Default::default()
    };
    let result = twyg::setup(opts);
    assert!(result.is_ok());
}

#[test]
fn test_setup_with_debug_level() {
    let opts = Opts {
        level: LogLevel::Debug,
        ..Default::default()
    };
    let result = twyg::setup(opts);
    assert!(result.is_ok());
}

#[test]
fn test_setup_with_info_level() {
    let opts = Opts {
        level: LogLevel::Info,
        ..Default::default()
    };
    let result = twyg::setup(opts);
    assert!(result.is_ok());
}

#[test]
fn test_setup_with_warn_level() {
    let opts = Opts {
        level: LogLevel::Warn,
        ..Default::default()
    };
    let result = twyg::setup(opts);
    assert!(result.is_ok());
}

#[test]
fn test_setup_with_error_level() {
    let opts = Opts {
        level: LogLevel::Error,
        ..Default::default()
    };
    let result = twyg::setup(opts);
    assert!(result.is_ok());
}

#[test]
fn test_setup_with_coloured() {
    let opts = Opts {
        coloured: true,
        level: LogLevel::Debug,
        ..Default::default()
    };
    let result = twyg::setup(opts);
    assert!(result.is_ok());
}

#[test]
fn test_setup_with_caller() {
    let opts = Opts {
        report_caller: true,
        level: LogLevel::Debug,
        ..Default::default()
    };
    let result = twyg::setup(opts);
    assert!(result.is_ok());
}

#[test]
fn test_setup_with_stdout() {
    let opts = Opts {
        output: Output::Stdout,
        level: LogLevel::Debug,
        ..Default::default()
    };
    let result = twyg::setup(opts);
    assert!(result.is_ok());
}

#[test]
fn test_setup_with_stderr() {
    let opts = Opts {
        output: Output::Stderr,
        level: LogLevel::Debug,
        ..Default::default()
    };
    let result = twyg::setup(opts);
    assert!(result.is_ok());
}

#[test]
fn test_setup_with_custom_time_format() {
    let opts = Opts {
        time_format: Some("%H:%M:%S".to_string()),
        level: LogLevel::Debug,
        ..Default::default()
    };
    let result = twyg::setup(opts);
    assert!(result.is_ok());
}

#[test]
fn test_setup_with_all_options() {
    let opts = Opts {
        coloured: true,
        output: Output::Stdout,
        level: LogLevel::Trace,
        report_caller: true,
        time_format: Some("%Y-%m-%d %H:%M:%S".to_string()),
    };
    let result = twyg::setup(opts);
    assert!(result.is_ok());
}

#[test]
fn test_opts_new() {
    let opts = Opts::new();
    assert!(!opts.coloured);
    assert_eq!(opts.output, Output::Stdout);
    assert_eq!(opts.level, LogLevel::Error);
    assert!(!opts.report_caller);
    assert_eq!(opts.time_format, Some("%Y-%m-%d %H:%M:%S".to_string()));
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
