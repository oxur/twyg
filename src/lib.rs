pub mod logger;
pub mod opts;

use anyhow::{anyhow, Error, Result};

use logger::Logger;
pub use opts::{Opts, STDERR, STDOUT};

/// Sets up a `fern::Dispatch` based upon the provided options.
///
/// The options (see the `twyg::LoggerOpts` struct) require that all of the
/// following fields be set:
///
/// * `coloured`: setting to false will disable ANIS colors in the logging output
/// * `file`: provide a path to a file, and output will be logged there too
/// * `level`: case-insensitive logging level
/// * `report_caller`: setting to true will output the filename and line number
///    where the logging call was made
///
/// With the options set, next call the setup function, passing a reference to
/// the opts as as the sole argument.
///
/// Usage example:
///
/// ```rust
/// use twyg;
///
/// let opts = twyg::Opts{
///     coloured: true,
///     file: None,
///     level: Some(String::from("debug")),
///     report_caller: true,
///
///     ..Default::default()
/// };
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
pub fn setup(opts: Opts) -> Result<Logger, Error> {
    let l = Logger::new(opts);
    match l.dispatch() {
        Err(e) => Err(anyhow!("couldn't set up Twyg logger ({:?}", e)),
        Ok(d) => match d.apply() {
            Err(e) => Err(anyhow!("couldn't apply setup to Fern logger ({:?}", e)),
            Ok(()) => Ok(l),
        },
    }
}
