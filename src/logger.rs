use std::fmt::Arguments;
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};
use chrono::Local;
use log::{self, Level, LevelFilter};
use owo_colors::{OwoColorize, Stream};
use serde::{Deserialize, Serialize};

use super::opts::{self, Opts};
use super::out;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Logger {
    opts: Opts,
}

impl Logger {
    pub fn new(opts: Opts) -> Logger {
        owo_colors::set_override(opts.coloured);
        Logger { opts }
    }

    pub fn dispatch(&self) -> Result<fern::Dispatch, Error> {
        let mut dispatch = if self.opts.report_caller {
            report_caller_logger(
                self.format_ts(),
                self.level_to_filter().unwrap(),
                self.stream(),
            )
        } else {
            logger(
                self.format_ts(),
                self.level_to_filter().unwrap(),
                self.stream(),
            )
        };
        dispatch = match self.opts.file.clone() {
            Some(opt) => match opt.as_str() {
                out::STDOUT => dispatch.chain(std::io::stdout()),
                out::STDERR => dispatch.chain(std::io::stderr()),
                f => dispatch.chain(fern::log_file(f)?),
            },
            _ => dispatch.chain(std::io::stdout()),
        };
        Ok(dispatch)
    }

    // Private methods

    fn format_ts(&self) -> String {
        let ts = match &self.opts.time_format {
            None => opts::default_ts_format().unwrap(),
            Some(ts) => ts.to_string(),
        };
        Local::now().format(ts.as_str()).to_string()
    }

    pub fn level(&self) -> String {
        let ts = match &self.opts.level {
            None => opts::default_level().unwrap(),
            Some(l) => l.to_string(),
        };
        Local::now().format(ts.as_str()).to_string()
    }

    fn level_to_filter(&self) -> Result<LevelFilter, Error> {
        let level = match &self.opts.level {
            None => opts::default_level().unwrap(),
            Some(l) => l.to_string(),
        };
        match LevelFilter::from_str(level.as_str()) {
            Ok(lf) => Ok(lf),
            Err(e) => Err(anyhow!(
                "couldn't convert log level String to LevelFilter ({:})",
                e
            )),
        }
    }

    fn stream(&self) -> Stream {
        match self.opts.clone().file {
            None => Stream::Stdout,
            Some(s) => {
                if s.as_str() == out::STDERR {
                    Stream::Stderr
                } else {
                    Stream::Stdout
                }
            }
        }
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
                    get_opt_str(record.file()),
                    get_opt_u32(record.line()),
                )
                .to_string()
                .if_supports_color(stream, |x| x.bright_yellow()),
                record
                    .target()
                    .to_string()
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
                    .to_string()
                    .if_supports_color(stream, |x| x.bright_yellow()),
                format_msg(message, stream).if_supports_color(stream, |x| x.bright_green())
            ))
        })
        .level(filter)
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
