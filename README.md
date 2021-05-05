# twyg

[![][build-badge]][build]
[![][crate-badge]][crate]
[![][tag-badge]][tag]
[![][docs-badge]][docs]

[![][logo]][logo-large]

*A tiny logging setup for Rust applications*

I got used to logging my apps in Clojure with [Twig](https://github.com/clojusc/twig),
in LFE with [Logjam](https://github.com/lfex/logjam), and in Go with
[zylog](https://github.com/geomyidia/zylog), so here this is for Rust.

## Usage

First, update your `Cargo.toml`s dependencies section:

```toml
[dependencies]
twyg = "0.1.8"
```

I like to put my logging setup in YAML config files for my apps, but however
you prefer to store your config, you'll next need to populate the
`twyg::LoggerOpts` struct for your preferred mechanism:

```rust
use twyg;

let opts = twyg::LoggerOpts{
    coloured: true,
    file: None,
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
Rust logging macros will use this setup, providing output like the following:

[![][screenshot-thumb]][screenshot]

The output in the screenshot above (click for a full-sized view) is from
running the demos in the `examples` directory.

## Config

Use with the [config][config] library is seamless:

1. Set up some YAML:

    ```yaml
    logging:
        coloured: true
        level: debug
        report_caller: true
    ```

1. Add an entry to your config struct:

    ```rust
    #[derive(Debug, Deserialize)]
    pub struct YourAppConfig {
        ...
        pub logging: twyg::LoggerOpts,
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
    match twyg::setup_logger(&cfg.logging) {
        Ok(_) => {}
        Err(error) => panic!("Could not setup logger: {:?}", error),
    };
    ```

## License

Copyright © 2020-2021, Oxur Group

Apache License, Version 2.0

<!-- Named page links below: /-->

[logo]: resources/images/logo-250x.png
[logo-large]: resources/images/logo-1000x.png
[screenshot-thumb]: resources/images/screenshot-thumb.jpg
[screenshot]: resources/images/screenshot.png
[build]: https://github.com/oxur/twyg/actions?query=workflow%3Abuild+
[build-badge]: https://github.com/oxur/twyg/workflows/build/badge.svg
[crate]: https://crates.io/crates/twyg
[crate-badge]: https://img.shields.io/crates/v/twyg.svg
[docs]: https://docs.rs/twyg/
[docs-badge]: https://img.shields.io/badge/rust-documentation-blue.svg
[tag-badge]: https://img.shields.io/github/tag/oxur/twyg.svg
[tag]: https://github.com/oxur/twyg/tags
[config]: https://github.com/mehcode/config-rs
