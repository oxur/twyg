use anyhow::Result;
use confyg::Confygery;
use serde::Deserialize;

mod common;
use common::demo;

const CONFIG_FILE: &str = "examples/config.toml";

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub logging: twyg::LoggerOpts,
}

fn main() -> Result<()> {
    let cfg: AppConfig = Confygery::new().add_file(CONFIG_FILE).build()?;
    demo::logs_sample(cfg.logging);
    Ok(())
}
