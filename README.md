# twyg

[![][build-badge]][build]
[![][crate-badge]][crate]
[![][docs-badge]][docs]

[![][logo]][logo-large]

*A tiny logging setup for Rust applications*

I got used to logging my apps in Clojure with [Twig](https://github.com/clojusc/twig)
and in LFE with [Logjam](https://github.com/lfex/logjam), so here this is.

## Usage

First, update your `Cargo.toml`s dependencies section:

```toml
[dependencies]
twyg = "0.1.3"
```

I like to put my logging setup in YAML config files for my apps, but however
you prefer to store your config, you'll next need to populate the
`twyg::LoggerOpts` struct for your preferred mechanism:

```rust
use twyg;

let opts = twyg::LoggerOpts{
    colored: true,
    file: String::from(""),
    level: String::from("debug"),
    report_caller: true,
    };

match twyg::setup_logger(&opts) {
    Ok(_) => {},
    Err(error) => {
        panic!("Could not setup logger: {:?}", error)
    },
};
```

The supported options are:

* `colored`: setting to false will disable ANIS colors in the logging output
* `file`: provide a path to a file, and output will be logged there too
* `level`: case-insensitive logging level
* `report_caller`: setting to true will output the filename and line number
   where the logging call was made

Once the setup function has been called, all subsequent calls to the standard
Rust log functions will use this setup, providing output like the following:

[![][screenshot-thumb]][screenshot]

The output in the screenshot above (click for a a full-sized view) is from
running the little demo in [main.rs](src/main.rs).

<!-- Named page links below: /-->

[logo]: resources/images/logo-250x.png
[logo-large]: resources/images/logo-1000x.png
[screenshot-thumb]: resources/images/screenshot-thumb.png
[screenshot]: resources/images/screenshot.png
[build]: https://github.com/oxur/twyg/actions?query=workflow%3Abuild+
[build-badge]: https://github.com/oxur/twyg/workflows/build/badge.svg
[crate]: https://crates.io/crates/twyg
[crate-badge]: https://img.shields.io/crates/v/twyg.svg
[docs]: https://docs.rs/twyg/
[docs-badge]: https://img.shields.io/badge/rust-documentation-blue.svg
