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

Version warnings:

* v0.4 - Due to the more complex nature of `OwoColors`, a major code refactor was required to fix the colour regression of v0.3, and as part of that several breaking changes were introduced, including a `struct` raname, new fields, etc.
* v0.3 - A regression was introduced due to the move away from the unsupported (and insecure) `colors` library whereby one could no longer disable ANSI colour of logged output.

## Usage

First, update your `Cargo.toml`s dependencies section:

```toml
[dependencies]
twyg = "0.5"
```

I like to put my logging setup in YAML config files for my apps, but however
you prefer to store your config, you'll next need to create a `twyg::Opts`
using the builder pattern:

```rust
use twyg::{LogLevel, OptsBuilder};

let opts = OptsBuilder::new()
    .coloured(true)
    .level(LogLevel::Debug)
    .report_caller(true)
    .build()
    .unwrap();

match twyg::setup(opts) {
    Ok(_) => {},
    Err(error) => {
        panic!("Could not setup logger: {:?}", error)
    },
};
```

The supported options are:

* `coloured`: setting to false will disable ANIS colours in the logging output
* `file`: provide a path to a file, and output will be logged there too
* `level`: case-insensitive logging level
* `report_caller`: setting to true will output the filename and line number
   where the logging call was made

Once the setup function has been called, all subsequent calls to the standard
Rust logging macros will use this setup, providing output like the following:

[![][screenshot-thumb]][screenshot]

The output in the screenshot above (click for a full-sized view) is from
running the demos in the `examples` directory.

## Config

Use with the [config][config] library is seamless thanks to serde support:

1. Set up some YAML:

    ```yaml
    logging:
        coloured: true
        level: debug
        output: stdout
        report_caller: true
        time_format: "%Y-%m-%d %H:%M:%S"
    ```

1. Add an entry to your config struct:

    ```rust
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    pub struct YourAppConfig {
        ...
        pub logging: twyg::Opts,
        ...
    }
    ```

1. Create a constructor for `YourAppConfig` (see config library docs and examples)
1. Build your config:

    ```rust
    let cfg = YourAppConfig::default().unwrap();
    ```

1. Pass the logging config to twyg:

    ```rust
    match twyg::setup(cfg.logging) {
        Ok(_) => {}
        Err(error) => panic!("Could not setup logger: {error:?}"),
    };
    ```

Note: The `Opts` struct uses lowercase serialization for enums (`"debug"`, `"info"`, etc.),
so your YAML/JSON config files should use lowercase strings for `level` and `output` fields.

## Migration Guide

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
