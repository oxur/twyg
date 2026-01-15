//! Example: Colored output without caller information
//!
//! Demonstrates how to configure twyg with:
//! - Colored ANSI output
//! - No caller information (cleaner output)
//! - Trace level logging
//!
//! This produces more concise log output compared to colour-caller.
//!
//! Run with: `cargo run --example no-caller`

mod common;

use twyg::{LogLevel, OptsBuilder};

use common::demo;

fn main() {
    let opts = OptsBuilder::new()
        .coloured(true)
        .level(LogLevel::Trace)
        .report_caller(false)
        .build()
        .unwrap();
    demo::logs_sample(opts);
}
