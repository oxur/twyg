# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.0] - 2026-01-14

### Added

- **Type-safe enums** for configuration:
  - `LogLevel` enum (`Trace`, `Debug`, `Info`, `Warn`, `Error`, `Fatal`) replacing stringly-typed level functions
  - `Output` enum (`Stdout`, `Stderr`, `File(PathBuf)`) replacing stringly-typed output configuration
  - Full serde support with lowercase serialization for config file compatibility

- **Builder pattern** for ergonomic API:
  - `OptsBuilder` with fluent method chaining
  - Compile-time validation of time format strings
  - Type-safe configuration construction

- **Custom error types**:
  - `TwygError` enum with specific variants (`InvalidTimeFormat`, `InitError`, `FileError`, `ConfigError`)
  - Better error messages with context using `thiserror`
  - Replaced `anyhow::Error` with custom `Result<T>` type alias

- **Comprehensive documentation**:
  - Module-level documentation for all modules
  - Usage examples in all public APIs
  - Documented examples with use cases
  - Migration guide in README

- **Performance optimizations**:
  - Reduced string allocations in hot logging paths using `Cow<'static, str>`
  - Removed unnecessary `.to_string()` calls in format closures
  - Zero-cost abstractions for enum conversions

- **Test coverage**:
  - 95%+ line coverage across all modules
  - 112 total tests (87 unit + 15 integration + 10 doc tests)
  - Comprehensive test suite for all features

### Changed

- **Breaking**: `Opts` struct fields are now private with getter methods
- **Breaking**: `setup()` now takes `Opts` by value instead of reference
- **Breaking**: Error type changed from `anyhow::Error` to `twyg::TwygError`
- **Breaking**: Time format validation now happens at build time via `OptsBuilder`
- Logger struct is now public for advanced use cases
- Improved error messages with more context

### Deprecated

- `level::debug()`, `level::info()`, etc. - Use `LogLevel::Debug`, `LogLevel::Info` instead
- `out::stdout()`, `out::stderr()` - Use `Output::Stdout`, `Output::Stderr` instead
- Direct struct initialization of `Opts` - Use `OptsBuilder` instead

### Migration from v0.4

See the [Migration Guide](README.md#migration-guide) in the README for detailed upgrade instructions.

**Before (v0.4):**
```rust
use twyg::{self, level};

let opts = twyg::Opts {
    level: level::debug(),
    coloured: true,
    ..Default::default()
};
twyg::setup(&opts)?;
```

**After (v0.5):**
```rust
use twyg::{LogLevel, OptsBuilder};

let opts = OptsBuilder::new()
    .level(LogLevel::Debug)
    .coloured(true)
    .build()?;
twyg::setup(opts)?;
```

## [0.4.0] - Previous Release

Major code refactor to fix color regression and update to `owo-colors`.

- Introduced `OwoColors` for better color support
- Fixed ANSI color disabling functionality
- Breaking changes to struct names and fields

## [0.3.0] - Previous Release

Regression introduced due to move away from unsupported `colors` library.

- Moved away from `colors` crate (security/maintenance)
- Regression: Could no longer disable ANSI color output

[0.5.0]: https://github.com/oxur/twyg/releases/tag/v0.5.0
[0.4.0]: https://github.com/oxur/twyg/releases/tag/v0.4.0
[0.3.0]: https://github.com/oxur/twyg/releases/tag/v0.3.0
