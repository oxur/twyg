use twyg;

mod common;

use common::demo;

fn main() {
    let opts = twyg::LoggerOpts {
        coloured: false,
        file: None,
        level: String::from("trace"),
        report_caller: false,
    };
    demo::logs_sample(opts);
}
