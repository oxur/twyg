//! Example: Structured logging with key-value pairs
//!
//! Demonstrates twyg's support for structured logging using the log crate's
//! kv feature. Key-value pairs are displayed after the message with colored
//! formatting.
//!
//! Run with: `cargo run --example structured-logging`

use twyg::{LogLevel, OptsBuilder};

fn main() {
    // Set up twyg with colored output and caller reporting
    let opts = OptsBuilder::new()
        .coloured(true)
        .level(LogLevel::Trace)
        .report_caller(true)
        .build()
        .unwrap();

    twyg::setup(opts).unwrap();

    println!("\n=== Structured Logging Examples ===\n");

    // Example 1: User action logging
    log::info!(user = "alice", action = "login"; "User logged in");

    // Example 2: HTTP request logging
    log::warn!(
        method = "GET",
        status = 404,
        path = "/api/users";
        "Request failed"
    );

    // Example 3: Multiple value types
    log::debug!(
        user_id = 42,
        email = "bob@example.com",
        admin = true;
        "User details"
    );

    // Example 4: Variable shorthand
    let session_id = "abc123";
    let ip_address = "192.168.1.100";
    log::info!(session_id, ip_address; "Session started");

    // Example 5: Error with context
    log::error!(
        error_code = 500,
        message = "database connection failed",
        retry_count = 3;
        "Service error"
    );

    // Example 6: Performance metrics
    log::trace!(
        duration_ms = 234,
        cache_hit = false,
        query = "SELECT * FROM users";
        "Query executed"
    );

    // Example 7: Regular logging still works (no kv pairs)
    log::info!("This is a regular log message without key-value pairs");

    println!("\n=== Examples Complete ===\n");
}
