//! Example: Plain text output without ANSI colors
//!
//! Demonstrates how to configure twyg with:
//! - No ANSI color codes (suitable for file logging or non-terminal output)
//! - No caller information
//! - Trace level logging
//!
//! Useful when:
//! - Logging to files
//! - Output is being parsed by other tools
//! - Running in environments that don't support ANSI colors
//!
//! Run with: `cargo run --example no-colour`

mod common;

use twyg::{LogLevel, OptsBuilder};

use common::demo;

fn main() {
    let opts = OptsBuilder::new()
        .coloured(false)
        .level(LogLevel::Trace)
        .report_caller(false)
        .build()
        .unwrap();
    demo::logs_sample(opts);
}
