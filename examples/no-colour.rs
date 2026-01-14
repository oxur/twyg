mod common;

use twyg::{LogLevel, Opts};

use common::demo;

fn main() {
    let opts = Opts {
        coloured: false,
        level: LogLevel::Trace,
        report_caller: false,

        ..Default::default()
    };
    demo::logs_sample(opts);
}
