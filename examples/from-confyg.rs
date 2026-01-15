//! Example: Loading configuration from TOML using the confyg crate
//!
//! Demonstrates integration with the confyg configuration crate:
//! - Load `Opts` from TOML configuration file
//! - Simpler alternative to the config crate
//! - Direct deserialization into application config struct
//!
//! The TOML config in `examples/config.toml` defines logging options.
//!
//! Run with: `cargo run --example from-confyg`

mod common;

use anyhow::Result;
use confyg::Confygery;
use serde::Deserialize;

use twyg::Opts;

use common::demo;

const CONFIG_FILE: &str = "examples/config.toml";

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub logging: Opts,
}

fn main() -> Result<()> {
    let cfg: AppConfig = Confygery::new()?.add_file(CONFIG_FILE)?.build()?;
    demo::logs_sample(cfg.logging);
    Ok(())
}
