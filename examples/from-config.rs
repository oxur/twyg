extern crate config as cfglib;
use serde::Deserialize;
use twyg;

const CONFIG_FILE: &str = "examples/config";

#[derive(Clone, Deserialize)]
pub struct AppConfig {
    pub logging: twyg::LoggerOpts,
}

impl AppConfig {
    pub fn new() -> Self {
        match new_app_config() {
            Ok(c) => c,
            Err(e) => {
                println!("{:?}", e);
                panic!("Configuration error: check the config file");
            }
        }
    }
}

pub fn new_app_config() -> Result<AppConfig, cfglib::ConfigError> {
    let mut c = cfglib::Config::default();
    // Start off by merging in the default configuration values
    c.merge(cfglib::File::with_name(CONFIG_FILE))?;
    c.try_into()
}

mod common;
use common::demo;

fn main() {
    let cfg = AppConfig::new();
    demo::logs_sample(cfg.logging);
}
