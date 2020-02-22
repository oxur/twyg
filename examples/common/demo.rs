use log;
use twyg;

pub fn logs_sample(opts: twyg::LoggerOpts) {
    match twyg::setup_logger(&opts) {
        Ok(_) => {}
        Err(error) => panic!("Could not setup logger: {:?}", error),
    };

    println!();
    log::trace!("Testing trace log output using twig ...");
    log::debug!("Testing trace log output using twig ...");
    log::info!("Testing trace log output using twig ...");
    log::warn!("Testing trace log output using twig ...");
    log::error!("Testing trace log output using twig ...");
    println!();
    log::debug!("Here's some data: {:?}", opts);
    println!();
}
