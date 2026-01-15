//! Example: Logging to standard error (stderr)
//!
//! Demonstrates how to configure twyg with:
//! - Output to stderr instead of stdout
//! - Colored ANSI output
//! - Caller information (file and line number)
//! - Trace level logging
//!
//! Useful when:
//! - You want to separate log output from application output
//! - Redirecting stdout to a file while keeping logs visible
//! - Following Unix conventions (diagnostics to stderr)
//!
//! Run with: `cargo run --example stderr`
//! Try: `cargo run --example stderr 2>/dev/null` to suppress logs

mod common;

use twyg::{LogLevel, OptsBuilder, Output};

use common::demo;

fn main() {
    let opts = OptsBuilder::new()
        .coloured(true)
        .output(Output::Stderr)
        .level(LogLevel::Trace)
        .report_caller(true)
        .build()
        .unwrap();
    demo::logs_sample(opts);
}
