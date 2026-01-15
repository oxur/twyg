//! Example: Loading configuration from YAML using the config crate
//!
//! Demonstrates how to integrate twyg with the popular config crate:
//! - Load `Opts` directly from YAML configuration file
//! - Support environment variable overrides (TWYG_ prefix)
//! - Seamless deserialization thanks to serde support
//!
//! The YAML config in `examples/config.yml` sets:
//! - Colored output enabled
//! - Debug log level
//! - Caller reporting enabled
//!
//! Try:
//! ```bash
//! cargo run --example from-config
//! TWYG_LOGGING__LEVEL=trace cargo run --example from-config
//! ```
//!
//! Run with: `cargo run --example from-config`

use anyhow::{anyhow, Error, Result};
use config as cfglib;
use serde::Deserialize;

use twyg::Opts;

const CONFIG_FILE: &str = "examples/config.yml";

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub logging: Opts,
}

impl AppConfig {
    pub fn new() -> Result<AppConfig, Error> {
        let build = cfglib::Config::builder()
            .add_source(cfglib::File::new(CONFIG_FILE, cfglib::FileFormat::Yaml))
            .add_source(cfglib::Environment::with_prefix("TWYG"))
            .build();
        match build {
            Ok(cfg) => Ok(cfg.try_deserialize()?),
            Err(e) => Err(anyhow!(e)),
        }
    }
}

mod common;
use common::demo;

fn main() {
    let cfg = AppConfig::new().unwrap();
    demo::logs_sample(cfg.logging);
}
