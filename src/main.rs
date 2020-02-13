use log;
use std::env;
use twyg;

fn main() {
    let args: Vec<_> = env::args().collect();
    let default_opts = twyg::LoggerOpts {
        colored: true,
        file: String::from(""),
        level: String::from("trace"),
        report_caller: true,
    };
    let opts = if args.len() <= 1 {
        default_opts
    } else if args[1] == "no-caller" {
        twyg::LoggerOpts {
            colored: true,
            file: String::from(""),
            level: String::from("trace"),
            report_caller: false,
        }
    } else if args[1] == "no-color" {
        twyg::LoggerOpts {
            colored: false,
            file: String::from(""),
            level: String::from("trace"),
            report_caller: false,
        }
    } else {
        default_opts
    };

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
