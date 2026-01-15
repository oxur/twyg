//! Example: Loading configuration from environment variables using confyg
//!
//! Demonstrates confyg's environment variable support:
//! - Load `Opts` from environment variables with MYAPP_ prefix
//! - Alternative to file-based configuration
//! - Useful for containerized environments and cloud deployments
//!
//! To run this example, first export the environment variables:
//! ```bash
//! export MYAPP_LOGGING_COLOURED=true
//! export MYAPP_LOGGING_OUTPUT=stdout
//! export MYAPP_LOGGING_LEVEL=trace
//! export MYAPP_LOGGING_REPORT_CALLER=true
//! export MYAPP_LOGGING_TIMESTAMP_FORMAT=Simple
//! export MYAPP_LOGGING_PAD_LEVEL=true
//! export MYAPP_LOGGING_PAD_AMOUNT=7
//! export MYAPP_LOGGING_PAD_SIDE=Left
//! export MYAPP_LOGGING_MSG_SEPARATOR=" :: "
//! export MYAPP_LOGGING_ARROW_CHAR="⇒"
//! cargo run --example from-confyg-env
//! ```
//!
//! See `.env-example` for all available variables.

mod common;

use anyhow::Result;
use serde::Deserialize;
use std::env;

use twyg::Opts;

use common::demo;

#[derive(Clone, Debug, Deserialize)]
pub struct MyApp {
    pub logging: Opts,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub myapp: MyApp,
}

fn main() -> Result<()> {
    // Check if MYAPP variables are set
    let has_config = env::vars().any(|(key, _)| key.starts_with("MYAPP_LOGGING_"));

    if !has_config {
        eprintln!("❌ No MYAPP_LOGGING_* environment variables found!");
        eprintln!("\nTo run this example, export the variables first:");
        eprintln!("  export MYAPP_LOGGING_COLOURED=true");
        eprintln!("  export MYAPP_LOGGING_LEVEL=trace");
        eprintln!("  export MYAPP_LOGGING_PAD_SIDE=Left");
        eprintln!("  export MYAPP_LOGGING_MSG_SEPARATOR=' :: '");
        eprintln!("  export MYAPP_LOGGING_ARROW_CHAR='⇒'");
        eprintln!("  ... (see examples/.env-example for full list)");
        eprintln!("\nOr use confyg crate directly:");
        eprintln!("  cargo run --example from-confyg-full");
        std::process::exit(1);
    }

    // Build configuration from environment variables using envy
    // We deserialize directly into Opts using the MYAPP_LOGGING_ prefix
    let logging: Opts = envy::prefixed("MYAPP_LOGGING_").from_env()?;
    let cfg = AppConfig {
        myapp: MyApp { logging },
    };

    demo::logs_sample(cfg.myapp.logging);
    Ok(())
}
