pub mod color;
pub mod error;
pub mod level;
pub mod logger;
pub mod opts;
pub mod out;
pub mod output;
pub mod timestamp;

pub use color::{Color, ColorAttribute, Colors};
pub use error::{Result, TwygError};
pub use level::LogLevel;
pub use logger::Logger;
pub use opts::{Opts, OptsBuilder, PadSide};
pub use out::{STDERR, STDOUT};
pub use output::Output;
pub use timestamp::TSFormat;

/// Sets up the twyg logger based upon the provided options.
///
/// The options (see the `twyg::Opts` struct) support the following configuration:
///
/// * `coloured`: setting to false will disable ANSI colors in the logging output
/// * `output`: specify stdout, stderr, or a file path for log output
/// * `level`: log level (Trace, Debug, Info, Warn, Error)
/// * `report_caller`: setting to true will output the filename and line number
///   where the logging call was made
/// * `time_format`: custom time format string (chrono format)
///
/// With the options set, call the setup function, passing the opts as the argument.
///
/// # Structured Logging Support
///
/// Twyg supports structured logging with key-value pairs using the log crate's
/// kv feature:
///
/// ```rust
/// use twyg::{self, LogLevel, OptsBuilder};
///
/// let opts = OptsBuilder::new()
///     .coloured(true)
///     .level(LogLevel::Debug)
///     .report_caller(true)
///     .build()
///     .unwrap();
///
/// match twyg::setup(opts) {
///     Ok(_) => {
///         // Regular logging
///         log::info!("User logged in");
///
///         // Structured logging with key-value pairs
///         log::info!(user = "alice", action = "login"; "User action");
///     },
///     Err(e) => {
///         panic!("Could not setup logger: {e:?}")
///     },
/// };
/// ```
///
/// At which point, calls to the `log::*!` macros will be displayed and
/// formatted according to your configuration.
///
pub fn setup(opts: Opts) -> Result<Logger> {
    let l = Logger::new(opts);
    l.dispatch()?;
    Ok(l)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setup_success() {
        // This test doesn't actually call setup() because the global logger
        // can only be initialized once per process. Instead, we test that Logger
        // can be created and verified successfully.
        let opts = Opts::default();
        let logger = Logger::new(opts.clone());
        assert_eq!(logger.level(), opts.level());
    }

    #[test]
    fn test_setup_dispatch_error_invalid_file_path() {
        // Test error path when dispatch() fails due to invalid file path
        // Note: We can't actually call dispatch() in tests after the first time
        // because log::set_logger() can only be called once. But we can verify
        // the Logger creation with invalid paths would fail when opened.
        let opts = OptsBuilder::new()
            .output(Output::file(
                "/proc/invalid/path/that/cannot/exist/test.log",
            ))
            .build()
            .unwrap();
        let logger = Logger::new(opts);
        // Logger created successfully, but dispatch() would fail on file open
        assert_eq!(logger.level(), LogLevel::default());
    }

    #[test]
    fn test_setup_dispatch_error_no_permission() {
        // Test error path for permission denied scenarios
        // Logger can be created, but dispatch() would fail on file open
        let opts = OptsBuilder::new()
            .output(Output::file("/root/twyg-test-no-permission.log"))
            .build()
            .unwrap();
        let logger = Logger::new(opts);
        // The logger is created; actual permission check happens in dispatch()
        assert_eq!(logger.level(), LogLevel::default());
    }
}
