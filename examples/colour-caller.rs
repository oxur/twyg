mod common;

use common::demo;

fn main() {
    let opts = twyg::LoggerOpts {
        coloured: true,
        file: None,
        level: String::from("trace"),
        report_caller: true,
    };
    demo::logs_sample(opts);
}
