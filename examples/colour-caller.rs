mod common;

use twyg::{LogLevel, Opts};

use common::demo;

fn main() {
    let opts = Opts {
        coloured: true,
        level: LogLevel::Trace,
        report_caller: true,

        ..Default::default()
    };
    demo::logs_sample(opts);
}
