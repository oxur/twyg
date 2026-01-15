use log;
use serde::Serialize;

use twyg::{self, Opts};

// Example data
#[derive(Debug, Serialize)]
struct Preferences {
    theme: &'static str,
    notify: bool,
}
#[derive(Debug, Serialize)]
struct User {
    id: u32,
    name: &'static str,
    email: &'static str,
    prefs: Preferences,
}

pub fn logs_sample(opts: Opts) {
    match twyg::setup(opts) {
        Ok(_) => {}
        Err(error) => panic!("Could not setup logger: {error:?}"),
    };

    println!();
    log::trace!("Testing trace log output using twyg ...");
    log::debug!("Testing trace log output using twyg ...");
    log::info!("Testing trace log output using twyg ...");
    log::warn!("Testing trace log output using twyg ...");
    log::error!("Testing trace log output using twyg ...");
    println!();
    let sample_data = User {
        id: 1,
        name: "Alice",
        email: "alice@example.com",
        prefs: Preferences {
            theme: "dark",
            notify: true,
        },
    };
    log::debug!("Here's some data: {sample_data:?}");
    println!();
}
