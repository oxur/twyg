use std::fmt::Arguments;
use std::str::FromStr;

use chrono;
use colored::*;
use fern::InitError;
use log;
use serde::{Deserialize, Serialize};

/// A reference to the `LoggerOpts` struct is required as an argument to
/// the `setup_logger` function.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LoggerOpts {
    pub coloured: bool,
    pub file: Option<String>,
    pub level: String,
    pub report_caller: bool,
}

fn get_log_level(opts: &LoggerOpts) -> log::LevelFilter {
    let level = log::LevelFilter::from_str(&opts.level);
    level.unwrap()
}

fn get_opt_str(x: Option<&str>) -> String {
    match x {
        None => "??".to_string(),
        Some(_) => x.unwrap().to_string(),
    }
}

fn get_opt_u32(x: Option<u32>) -> String {
    match x {
        None => "??".to_string(),
        Some(_) => x.unwrap().to_string(),
    }
}

fn colour_level(level: log::Level) -> colored::ColoredString {
    match level {
        log::Level::Error => level.to_string().red(),
        log::Level::Warn => level.to_string().bright_yellow(),
        log::Level::Info => level.to_string().bright_green(),
        log::Level::Debug => level.to_string().cyan(),
        log::Level::Trace => level.to_string().bright_blue(),
    }
}

fn format_msg(msg: &Arguments<'_>) -> colored::ColoredString {
    format!("{} {}", "▶".cyan(), msg).green()
}

fn get_report_caller_logger(opts: &LoggerOpts) -> fern::Dispatch {
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{date} {level} [{file} {target}] {message}",
                date = chrono::Local::now()
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string()
                    .green(),
                target = record.target().to_string().bright_yellow(),
                level = colour_level(record.level()),
                file = format_args!(
                    "{}:{}",
                    get_opt_str(record.file()),
                    get_opt_u32(record.line()),
                )
                .to_string()
                .yellow(),
                message = format_msg(message).bright_green(),
            ))
        })
        .level(get_log_level(opts))
        .chain(std::io::stdout())
}

fn get_logger(opts: &LoggerOpts) -> fern::Dispatch {
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{date} {level} [{target}] {message}",
                date = chrono::Local::now()
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string()
                    .green(),
                target = record.target().to_string().bright_yellow(),
                level = colour_level(record.level()),
                message = format_msg(message).bright_green(),
            ))
        })
        .level(get_log_level(opts))
        .chain(std::io::stdout())
}
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
/// let opts = twyg::LoggerOpts{
///     coloured: true,
///     file: None,
///     level: String::from("debug"),
///     report_caller: true,
/// };
///
/// match twyg::setup_logger(&opts) {
///     Ok(_) => {},
///     Err(e) => {
///         panic!("Could not setup logger: {:?}", e)
///     },
/// };
/// ```
///
/// At which point, calls to the `log::*!` macros will be displayed and
/// formatted according to your configuration and twyg.
///
pub fn setup_logger(opts: &LoggerOpts) -> Result<(), InitError> {
    colored::control::set_override(opts.coloured);
    let mut logger = if opts.report_caller {
        get_report_caller_logger(opts)
    } else {
        get_logger(opts)
    };
    logger = match &opts.file {
        Some(f) => logger.chain(fern::log_file(f)?),
        _ => logger,
    };
    match logger.apply() {
        Ok(_) => Ok(()),
        Err(e) => {
            log::warn!("{e}");
            Ok(())
        }
    }
}
