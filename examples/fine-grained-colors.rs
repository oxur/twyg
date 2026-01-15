//! Example demonstrating fine-grained color configuration.
//!
//! This example shows how to customize every aspect of the log output including:
//! - Individual colors for each log component
//! - Level padding and alignment
//! - Custom arrow character and message separator
//! - Timestamp format presets
//! - Background colors for dramatic effect

use twyg::{Color, ColorAttribute, Colors, LogLevel, OptsBuilder, PadSide, TSFormat};

fn main() {
    // Create a radically different color scheme with vivid, high-contrast colors
    let mut colors = Colors::default();

    // Bright cyan timestamp (instead of default green)
    colors.timestamp = Some(Color::fg(ColorAttribute::HiCyan));

    // Dramatically different level colors with backgrounds
    colors.level_error = Some(Color::new(ColorAttribute::HiWhite, ColorAttribute::Red));
    colors.level_warn = Some(Color::new(ColorAttribute::Black, ColorAttribute::HiYellow));
    colors.level_info = Some(Color::new(ColorAttribute::HiWhite, ColorAttribute::Blue));
    colors.level_debug = Some(Color::fg(ColorAttribute::HiMagenta));
    colors.level_trace = Some(Color::new(ColorAttribute::Yellow, ColorAttribute::Magenta));

    // Bright white message text (highly visible)
    colors.message = Some(Color::fg(ColorAttribute::HiWhite));

    // Yellow arrow separator (warm accent)
    colors.arrow = Some(Color::fg(ColorAttribute::Yellow));

    // Caller information in contrasting colors
    colors.caller_file = Some(Color::fg(ColorAttribute::Cyan));
    colors.target = Some(Color::fg(ColorAttribute::HiGreen));

    // Structured logging with vibrant colors
    colors.attr_key = Some(Color::fg(ColorAttribute::Cyan));
    colors.attr_value = Some(Color::fg(ColorAttribute::HiGreen));

    let opts = OptsBuilder::new()
        .coloured(true)
        .level(LogLevel::Trace)
        .report_caller(true)
        .timestamp_format(TSFormat::Simple)
        .pad_level(true)
        .pad_amount(7)
        .pad_side(PadSide::Right)
        .arrow_char("→")
        .msg_separator(" | ")
        .colors(colors)
        .build()
        .unwrap();

    twyg::setup(opts).unwrap();

    println!("\n=== Fine-Grained Colors Example ===\n");
    println!("Demonstrating custom colors, padding, and separators:\n");

    log::trace!("This is a trace message with custom magenta color");
    log::debug!("This is a debug message with custom cyan color");
    log::info!("This is an info message with custom bright green color");
    log::warn!("This is a warning with black text on yellow background");
    log::error!("This is an error with white text on red background");

    println!("\nStructured logging with custom separator and colors:\n");
    log::info!(user = "alice", action = "login"; "User logged in");
    log::warn!(temperature = 85, threshold = 80; "Temperature warning");
    log::error!(code = 500, endpoint = "/api/data"; "API request failed");

    println!("\n=== Example Complete ===\n");
}
