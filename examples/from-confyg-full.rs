//! Example: Comprehensive configuration from TOML using confyg
//!
//! Demonstrates all available twyg configuration options:
//! - All color customization options (13 color fields)
//! - Timestamp format variants
//! - Level padding and alignment
//! - Custom separators and arrow characters
//! - Fine-grained control over every visual aspect
//!
//! The config file `examples/config-full.toml` demonstrates every
//! available configuration option with explanatory comments.
//!
//! Run with: `cargo run --example from-confyg-full`

mod common;

use anyhow::Result;
use confyg::Confygery;
use serde::Deserialize;

use twyg::Opts;

use common::demo;

const CONFIG_FILE: &str = "examples/config-full.toml";

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub logging: Opts,
}

fn main() -> Result<()> {
    let cfg: AppConfig = Confygery::new()?.add_file(CONFIG_FILE)?.build()?;
    demo::logs_sample(cfg.logging);
    Ok(())
}
