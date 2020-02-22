use twyg;

mod common;

use common::demo;

fn main() {
    let opts = twyg::LoggerOpts {
        colored: true,
        file: String::from(""),
        level: String::from("trace"),
        report_caller: true,
    };
    demo::logs_sample(opts);
}
