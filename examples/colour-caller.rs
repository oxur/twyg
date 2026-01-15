//! Example: Colored output with caller information
//!
//! Demonstrates how to configure twyg with:
//! - Colored ANSI output
//! - File name and line number reporting (caller information)
//! - Trace level logging (most verbose)
//!
//! Run with: `cargo run --example colour-caller`

mod common;

use twyg::{LogLevel, OptsBuilder};

use common::demo;

fn main() {
    let opts = OptsBuilder::new()
        .coloured(true)
        .level(LogLevel::Trace)
        .report_caller(true)
        .build()
        .unwrap();
    demo::logs_sample(opts);
}
