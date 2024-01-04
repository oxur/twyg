use serde::{Deserialize, Serialize};

pub const STDOUT: &str = "stdout";
pub const STDERR: &str = "stderr";
const DEFAULT_LEVEL: &str = "error";
const DEFAULT_TS_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Opts {
    pub coloured: bool,
    pub file: Option<String>,
    pub level: Option<String>,
    pub report_caller: bool,
    pub time_format: Option<String>,
}

impl Opts {
    pub fn new() -> Opts {
        let mut opts = Opts::default();
        if opts.file.is_none() {
            opts.file = default_file()
        }
        if opts.level.is_none() {
            opts.level = default_level()
        }
        if opts.time_format.is_none() {
            opts.time_format = default_ts_format();
        }
        opts
    }
}

pub fn default_file() -> Option<String> {
    Some(STDOUT.to_string())
}

pub fn default_level() -> Option<String> {
    Some(DEFAULT_LEVEL.to_string())
}

pub fn default_ts_format() -> Option<String> {
    Some(DEFAULT_TS_FORMAT.to_string())
}
