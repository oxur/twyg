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
    println!("Loading comprehensive configuration from {}...\n", CONFIG_FILE);

    let cfg: AppConfig = Confygery::new()?.add_file(CONFIG_FILE)?.build()?;

    println!("Configuration loaded successfully!");
    println!("Features enabled:");
    println!("  • Colored output: {}", cfg.logging.coloured());
    println!("  • Report caller: {}", cfg.logging.report_caller());
    println!("  • Level padding: {} (amount: {}, side: {:?})",
             cfg.logging.pad_level(),
             cfg.logging.pad_amount(),
             cfg.logging.pad_side());
    println!("  • Message separator: {:?}", cfg.logging.msg_separator());
    println!("  • Arrow character: {:?}", cfg.logging.arrow_char());
    println!("  • Timestamp format: {:?}", cfg.logging.timestamp_format());
    println!();

    demo::logs_sample(cfg.logging);
    Ok(())
}
