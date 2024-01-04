use log;

use twyg::{self, Opts};

pub fn logs_sample(opts: Opts) {
    match twyg::setup(opts.clone()) {
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
    log::debug!("Here's some data: {opts:?}");
    println!();
}
