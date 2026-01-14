mod common;

use twyg::{LogLevel, Opts};

use common::demo;

fn main() {
    let opts = Opts {
        coloured: true,
        level: LogLevel::Trace,
        report_caller: false,

        ..Default::default()
    };
    demo::logs_sample(opts);
}
