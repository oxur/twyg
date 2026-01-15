# twyg

[![][build-badge]][build]
[![][crate-badge]][crate]
[![][tag-badge]][tag]
[![][docs-badge]][docs]

[![][logo]][logo-large]

*A tiny logging setup for Rust applications*

I got used to logging my apps in with:

* [Twig](https://github.com/clojusc/twig) (Clojure)
* [Logjam](https://github.com/lfex/logjam) (LFE)
* [zylog](https://github.com/zylisp/zylog) (Go)

so here's something similar for Rust ;-)

## Features

* 🎨 Beautiful colored output with fine-grained color customization
* ⏰ Multiple timestamp formats (RFC3339, Standard, Simple, TimeOnly)
* 📍 Optional caller information (file, line, function)
* 📏 Configurable level padding for perfect alignment
* 🎯 Structured logging with key-value pairs
* ⚙️ Simple configuration via builder pattern or config files
* 🔄 Both foreground and background color support
* 🚀 Zero-overhead when color is disabled

## Installation

Add twyg to your `Cargo.toml`:

```toml
[dependencies]
twyg = "0.6"
```

## Quick Start

```rust
use twyg::{LogLevel, OptsBuilder};

// Set up with default configuration
let opts = OptsBuilder::new()
    .coloured(true)
    .level(LogLevel::Debug)
    .report_caller(true)
    .build()
    .unwrap();

twyg::setup(opts).expect("Failed to set up logger");

// Now use standard Rust logging macros
log::info!("Application started");
log::debug!(user = "alice", id = 42; "User logged in");
log::warn!("Configuration file missing, using defaults");
log::error!("Failed to connect to database");
```

Once the setup function has been called, all subsequent calls to the standard
Rust logging macros will use this configuration, providing beautifully formatted output:

[![][screenshot-thumb]][screenshot]

The output in the screenshot above (click for a full-sized view) is from
running the demos in the `examples` directory.

## Configuration Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `coloured` | `bool` | `true` | Enable/disable ANSI color output |
| `output` | `Output` | `Stdout` | Output destination: `Stdout`, `Stderr`, or `File(path)` |
| `level` | `LogLevel` | `Info` | Minimum log level: `Trace`, `Debug`, `Info`, `Warn`, `Error` |
| `report_caller` | `bool` | `false` | Include file name and line number in output |
| `timestamp_format` | `TSFormat` | `Standard` | Timestamp format (see below) |
| `pad_level` | `bool` | `false` | Enable padding of log level strings for alignment |
| `pad_amount` | `usize` | `5` | Number of characters to pad level strings to |
| `pad_side` | `PadSide` | `Right` | Padding side: `Left` (right-align) or `Right` (left-align) |
| `arrow_char` | `String` | `"▶"` | Arrow separator between metadata and message |
| `msg_separator` | `String` | `": "` | Separator before structured logging attributes |
| `colors` | `Colors` | See below | Fine-grained color control for each component |

### Timestamp Formats

twyg supports multiple timestamp format presets:

```rust
use twyg::TSFormat;

// RFC3339 format
TSFormat::RFC3339        // "2026-01-15T14:30:52-08:00"

// Standard format (default)
TSFormat::Standard       // "2026-01-15 14:30:52"

// Compact format
TSFormat::Simple         // "20260115.143052"

// Time only
TSFormat::TimeOnly       // "14:30:52"

// Custom chrono format string
TSFormat::Custom("%H:%M:%S%.3f".to_string())  // "14:30:52.123"
```

### Output Format

twyg produces clean, readable log output with optional caller information and structured key-value pairs:

**Without caller information:**

```
2026-01-15 14:30:52 INFO  [myapp] ▶ Application started
2026-01-15 14:30:52 DEBUG [myapp::auth] ▶ Processing login request
2026-01-15 14:30:52 WARN  [myapp::config] ▶ Using default configuration: file={config.yaml}
2026-01-15 14:30:52 ERROR [myapp::db] ▶ Connection failed: host={localhost}, port={5432}
```

**With caller information:**

```
2026-01-15 14:30:52 INFO  [main.rs:42 myapp] ▶ Application started
2026-01-15 14:30:52 DEBUG [auth.rs:127 myapp::auth] ▶ User logged in: user={alice}, id={42}
```

**With level padding and custom formatting:**

```
20260115.143052 INFO  [main.rs:42 myapp] → Application started
20260115.143052 WARN  [config.rs:18 myapp::config] → Missing key | key={api_token}
20260115.143052 ERROR [db.rs:93 myapp::db] → Failed to connect | error={timeout}
```

## Fine-Grained Color Configuration

twyg allows you to customize the foreground and background colors of every formatted element. By default, twyg uses sensible color defaults, but you can override any color you want.

### Simple Example - Changing a Few Colors

You don't need to configure every color. Just customize the ones you want to change:

```rust
use twyg::{Color, ColorAttribute, Colors, OptsBuilder};

let mut colors = Colors::default();

// Customize just the colors you want to change
colors.level_error = Some(Color::new(
    ColorAttribute::HiWhite,     // White text
    ColorAttribute::Red          // Red background
));
colors.message = Some(Color::fg(ColorAttribute::HiCyan));
colors.arrow = Some(Color::fg(ColorAttribute::Magenta));

let opts = OptsBuilder::new()
    .coloured(true)
    .colors(colors)
    .build()
    .unwrap();

twyg::setup(opts).unwrap();
log::error!("This error has white text on a red background!");
log::info!("This message is in high-intensity cyan");
```

### Disabling Color for Specific Elements

To disable color for a specific element while keeping others colored, set both foreground and background to `Reset`:

```rust
let mut colors = Colors::default();
colors.timestamp = Some(Color::new(
    ColorAttribute::Reset,
    ColorAttribute::Reset
));
// Timestamp will now be uncolored, but everything else remains colored
```

### Complete Color Configuration Reference

The `Colors` struct provides fine-grained control over every colored element:

```rust
pub struct Colors {
    // Timestamp color (default: Green)
    pub timestamp: Option<Color>,

    // Log level colors
    pub level_trace: Option<Color>,    // default: HiBlue
    pub level_debug: Option<Color>,    // default: Cyan
    pub level_info: Option<Color>,     // default: HiGreen
    pub level_warn: Option<Color>,     // default: HiYellow
    pub level_error: Option<Color>,    // default: Red

    // Message text color (default: Green)
    pub message: Option<Color>,

    // Arrow separator "▶" (default: Cyan)
    pub arrow: Option<Color>,

    // Caller information colors
    pub caller_file: Option<Color>,    // default: HiYellow
    pub caller_line: Option<Color>,    // default: HiYellow

    // Target/module name color (default: HiYellow)
    pub target: Option<Color>,

    // Structured logging attribute colors
    pub attr_key: Option<Color>,       // default: HiYellow
    pub attr_value: Option<Color>,     // default: Cyan
}

pub struct Color {
    pub fg: ColorAttribute,  // Foreground color
    pub bg: ColorAttribute,  // Background color
}
```

### Available Colors

`ColorAttribute` provides these options:

**Standard colors:**

* `Black`, `Red`, `Green`, `Yellow`, `Blue`, `Magenta`, `Cyan`, `White`

**Bright/high-intensity colors:**

* `HiBlack`, `HiRed`, `HiGreen`, `HiYellow`, `HiBlue`, `HiMagenta`, `HiCyan`, `HiWhite`

**Special:**

* `Reset` - No color (use for both foreground and background to disable coloring for an element)

You can create colors with just foreground:

```rust
Color::fg(ColorAttribute::Red)
```

Or with both foreground and background:

```rust
Color::new(ColorAttribute::White, ColorAttribute::Red)  // White text on red background
```

### Global Color Disable

The `coloured: false` option continues to work and will disable ALL colors regardless of individual color settings:

```rust
let opts = OptsBuilder::new()
    .coloured(false)  // Disables all colors globally
    .build()
    .unwrap();
```

### Complete Example

See `examples/fine-grained-colors.rs` for a complete working example with custom colors, padding, and formatting options.

## Examples

twyg includes several examples demonstrating different features:

```bash
# Run all examples
make run-examples

# Or run individual examples
cargo run --example simple
cargo run --example fine-grained-colors
cargo run --example from-confyg-full        # Comprehensive TOML config
cargo run --example from-confyg-env         # Environment variable config
```

## Configuration Files

twyg works seamlessly with configuration libraries thanks to serde support.

### Using with the config crate

Use with the [config][config] library:

**1. Set up your configuration file (YAML example):**

```yaml
logging:
    coloured: true
    level: debug
    output: stdout
    report_caller: true
    timestamp_format: Standard  # Or: RFC3339, Simple, TimeOnly
    pad_level: true
    pad_amount: 7
    pad_side: Right
    arrow_char: "→"
    msg_separator: " | "
    colors:
        timestamp:
            fg: HiCyan
            bg: Reset
        level_info:
            fg: HiWhite
            bg: Blue
        message:
            fg: HiWhite
            bg: Reset
```

**2. Add twyg to your config struct:**

```rust
use serde::Deserialize;
use twyg::Opts;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub logging: Opts,
}
```

**3. Load and apply configuration:**

```rust
let cfg: AppConfig = config::Config::builder()
    .add_source(config::File::with_name("config.yaml"))
    .build()?
    .try_deserialize()?;

twyg::setup(cfg.logging)?;
```

### Using with confyg

For TOML configuration with the [confyg](https://crates.io/crates/confyg) library:

```toml
[logging]
coloured = true
level = "debug"
output = "stdout"
report_caller = true
timestamp_format = "Simple"
pad_level = true
pad_amount = 7
pad_side = "Right"
arrow_char = "→"
msg_separator = " | "

[logging.colors]
timestamp = { fg = "HiBlack", bg = "Reset" }
level_info = { fg = "HiGreen", bg = "Reset" }
level_error = { fg = "White", bg = "Red" }
message = { fg = "Cyan", bg = "Reset" }
```

See `examples/config-full.toml` for a comprehensive configuration example with all available options.

### Using Environment Variables

Configuration via environment variables with the [envy](https://crates.io/crates/envy) crate:

```bash
export MYAPP_LOGGING_COLOURED=true
export MYAPP_LOGGING_OUTPUT=stdout
export MYAPP_LOGGING_LEVEL=debug
export MYAPP_LOGGING_REPORT_CALLER=true
export MYAPP_LOGGING_TIMESTAMP_FORMAT=Simple
export MYAPP_LOGGING_PAD_LEVEL=true
export MYAPP_LOGGING_PAD_AMOUNT=7
export MYAPP_LOGGING_PAD_SIDE=Left
```

```rust
use serde::Deserialize;
use twyg::Opts;

let logging: Opts = envy::prefixed("MYAPP_LOGGING_").from_env()?;
twyg::setup(logging)?;
```

See `examples/.env-example` and `examples/from-confyg-env.rs` for complete examples.

**Note:** Configuration uses lowercase serialization for enums, so use strings like `"debug"`, `"info"`, `"stdout"`, etc.

## Migration Guide

### Upgrading from v0.5 to v0.6

v0.6 adds fine-grained color configuration and new formatting options. All changes are backward compatible:

#### New Features (Optional)

All new fields have sensible defaults, so existing code works without changes:

```rust
// v0.5 code continues to work
let opts = OptsBuilder::new()
    .coloured(true)
    .level(LogLevel::Debug)
    .build()
    .unwrap();
```

To use new features:

```rust
use twyg::{Color, ColorAttribute, Colors, PadSide, TSFormat};

let mut colors = Colors::default();
colors.level_error = Some(Color::new(ColorAttribute::White, ColorAttribute::Red));

let opts = OptsBuilder::new()
    .coloured(true)
    .level(LogLevel::Debug)
    .timestamp_format(TSFormat::Simple)  // NEW
    .pad_level(true)                     // NEW
    .pad_amount(7)                       // NEW
    .pad_side(PadSide::Right)           // NEW
    .arrow_char("→")                     // NEW
    .msg_separator(" | ")                // NEW
    .colors(colors)                      // NEW
    .build()
    .unwrap();
```

#### Deprecated Methods

The `time_format()` method is deprecated in favor of `timestamp_format()`:

```rust
// Deprecated (still works)
.time_format("%H:%M:%S")

// Preferred
.timestamp_format(TSFormat::Custom("%H:%M:%S".to_string()))
```

### Upgrading from v0.4 to v0.5

v0.5 introduces type-safe enums and a builder pattern for better API ergonomics. Here's how to migrate:

#### 1. Replace stringly-typed level functions with LogLevel enum

**Before (v0.4):**

```rust
use twyg::{self, level};

let opts = twyg::Opts {
    level: level::debug(),
    ...
};
```

**After (v0.5):**

```rust
use twyg::{LogLevel, OptsBuilder};

let opts = OptsBuilder::new()
    .level(LogLevel::Debug)
    .build()
    .unwrap();
```

#### 2. Replace stringly-typed output with Output enum

**Before (v0.4):**

```rust
use twyg::{self, out};

let opts = twyg::Opts {
    file: out::stdout(),
    ...
};
```

**After (v0.5):**

```rust
use twyg::{Output, OptsBuilder};

let opts = OptsBuilder::new()
    .output(Output::Stdout)
    .build()
    .unwrap();
```

#### 3. Use OptsBuilder instead of struct literals

**Before (v0.4):**

```rust
let opts = twyg::Opts {
    coloured: true,
    level: level::debug(),
    report_caller: true,
    ..Default::default()
};
```

**After (v0.5):**

```rust
let opts = OptsBuilder::new()
    .coloured(true)
    .level(LogLevel::Debug)
    .report_caller(true)
    .build()
    .unwrap();
```

#### 4. Error handling now uses custom TwygError

**Before (v0.4):**

```rust
match twyg::setup(&opts) {
    Ok(_) => {},
    Err(error) => { /* anyhow::Error */ },
}
```

**After (v0.5):**

```rust
match twyg::setup(opts) {
    Ok(_) => {},
    Err(error) => { /* twyg::TwygError with specific variants */ },
}
```

#### Backwards Compatibility

The old stringly-typed functions are still available but deprecated:

* `level::debug()`, `level::info()`, etc. → Use `LogLevel::Debug`, `LogLevel::Info`
* `out::stdout()`, `out::stderr()` → Use `Output::Stdout`, `Output::Stderr`

## License

Copyright © 2020-2026, Oxur Group

Apache License, Version 2.0

[//]: ---Named-Links---

[logo]: assets/images/logo-250x.png
[logo-large]: assets/images/logo-1000x.png
[screenshot-thumb]: assets/images/screenshot-thumb.jpg
[screenshot]: https://raw.githubusercontent.com/oxur/twyg/main/assets/images/screenshot.png
[config]: https://github.com/mehcode/config-rs
[build]: https://github.com/oxur/twyg/actions/workflows/ci.yml
[build-badge]: https://github.com/oxur/twyg/actions/workflows/ci.yml/badge.svg
[crate]: https://crates.io/crates/twyg
[crate-badge]: https://img.shields.io/crates/v/twyg.svg
[docs]: https://docs.rs/twyg/
[docs-badge]: https://img.shields.io/badge/rust-documentation-blue.svg
[tag-badge]: https://img.shields.io/github/tag/oxur/twyg.svg
[tag]: https://github.com/oxur/twyg/tags
