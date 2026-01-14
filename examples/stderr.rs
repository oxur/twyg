mod common;

use twyg::{LogLevel, Opts, Output};

use common::demo;

fn main() {
    let opts = Opts {
        coloured: true,
        output: Output::Stderr,
        level: LogLevel::Trace,
        report_caller: true,

        ..Default::default()
    };
    demo::logs_sample(opts);
}
