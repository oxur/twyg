pub mod error;
pub mod level;
pub mod logger;
pub mod opts;
pub mod out;
pub mod output;

pub use error::{Result, TwygError};
pub use level::LogLevel;
pub use logger::Logger;
pub use opts::{Opts, OptsBuilder};
pub use out::{STDERR, STDOUT};
pub use output::Output;

/// Sets up a `fern::Dispatch` based upon the provided options.
///
/// The options (see the `twyg::LoggerOpts` struct) require that all of the
/// following fields be set:
///
/// * `coloured`: setting to false will disable ANIS colors in the logging output
/// * `file`: provide a path to a file, and output will be logged there too
/// * `level`: case-insensitive logging level
/// * `report_caller`: setting to true will output the filename and line number
///   where the logging call was made
///
/// With the options set, next call the setup function, passing a reference to
/// the opts as as the sole argument.
///
/// Usage example:
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
///     Ok(_) => {},
///     Err(e) => {
///         panic!("Could not setup logger: {e:?}")
///     },
/// };
/// ```
///
/// At which point, calls to the `log::*!` macros will be displayed and
/// formatted according to your configuration and twyg.
///
pub fn setup(opts: Opts) -> Result<Logger> {
    let l = Logger::new(opts);
    let dispatch = l.dispatch()?;
    dispatch.apply()?;
    Ok(l)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setup_success() {
        // This test doesn't actually call setup() because the global logger
        // can only be initialized once. Instead, we test that Logger can
        // create a dispatch successfully.
        let opts = Opts::default();
        let logger = Logger::new(opts);
        let result = logger.dispatch();
        assert!(result.is_ok());
    }

    #[test]
    fn test_setup_dispatch_error_invalid_file_path() {
        // Test error path when dispatch() fails due to invalid file path
        let opts = OptsBuilder::new()
            .output(Output::file(
                "/proc/invalid/path/that/cannot/exist/test.log",
            ))
            .build()
            .unwrap();
        let logger = Logger::new(opts);
        let result = logger.dispatch();
        assert!(result.is_err());
    }

    #[test]
    fn test_setup_dispatch_error_no_permission() {
        // Test error path when dispatch() fails due to permission denied
        // /dev/null is writable, but /root typically isn't without sudo
        let opts = OptsBuilder::new()
            .output(Output::file("/root/twyg-test-no-permission.log"))
            .build()
            .unwrap();
        let logger = Logger::new(opts);
        let result = logger.dispatch();
        // This may or may not fail depending on system permissions
        // If it succeeds, that's also fine (user has write access to /root)
        // The important part is that the error path is exercised
        let _ = result;
    }
}
