use config as cfglib;
use serde::Deserialize;

const CONFIG_FILE: &str = "examples/config.yml";

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub logging: twyg::LoggerOpts,
}

impl AppConfig {
    pub fn new() -> Result<Self, cfglib::ConfigError> {
        let cfg = cfglib::Config::builder()
            .add_source(cfglib::File::new(CONFIG_FILE, cfglib::FileFormat::Yaml))
            .add_source(cfglib::Environment::with_prefix("TWYG"))
            .build()?;
        cfg.try_deserialize()
    }
}

mod common;
use common::demo;

fn main() {
    let cfg = AppConfig::new().unwrap();
    demo::logs_sample(cfg.logging);
}
