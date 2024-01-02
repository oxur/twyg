use std::fmt::Arguments;
use std::str::FromStr;

use chrono::Local;
use fern::InitError;
use log;
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

const TIMESTAMP: &str = "%Y-%m-%d %H:%M:%S";

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
        Some(val) => val.to_string(),
    }
}

fn colour_level(level: log::Level) -> String {
    let str_level = level.to_string();
    match level {
        log::Level::Error => str_level.red().to_string(),
        log::Level::Warn => str_level.bright_yellow().to_string(),
        log::Level::Info => str_level.bright_green().to_string(),
        log::Level::Debug => str_level.cyan().to_string(),
        log::Level::Trace => str_level.bright_blue().to_string(),
    }
}

fn format_msg(msg: &Arguments<'_>) -> String {
    format!("{} {}", "▶".cyan(), msg).green().to_string()
}

fn get_report_caller_logger(opts: &LoggerOpts) -> fern::Dispatch {
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{date} {level} [{file} {target}] {message}",
                date = Local::now().format(TIMESTAMP).to_string().green(),
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
}

fn get_logger(opts: &LoggerOpts) -> fern::Dispatch {
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{date} {level} [{target}] {message}",
                date = Local::now().format(TIMESTAMP).to_string().green(),
                target = record.target().to_string().bright_yellow(),
                level = colour_level(record.level()),
                message = format_msg(message).bright_green(),
            ))
        })
        .level(get_log_level(opts))
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
///         panic!("Could not setup logger: {e:?}")
///     },
/// };
/// ```
///
/// At which point, calls to the `log::*!` macros will be displayed and
/// formatted according to your configuration and twyg.
///
pub fn setup_logger(opts: &LoggerOpts) -> Result<(), InitError> {
    let mut logger = if opts.report_caller {
        get_report_caller_logger(opts)
    } else {
        get_logger(opts)
    };
    logger = match opts.file.clone() {
        Some(opt) => match opt.as_str() {
            "stdout" => logger.chain(std::io::stdout()),
            "stderr" => logger.chain(std::io::stderr()),
            f => logger.chain(fern::log_file(f)?),
        },
        _ => logger.chain(std::io::stdout()),
    };
    match logger.apply() {
        Ok(_) => Ok(()),
        Err(e) => {
            log::warn!("{e}");
            Ok(())
        }
    }
}
