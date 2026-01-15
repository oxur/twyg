mod common;

use twyg::{LogLevel, OptsBuilder};

use common::demo;

fn main() {
    let opts = OptsBuilder::new()
        .coloured(true)
        .level(LogLevel::Trace)
        .report_caller(true)
        .build()
        .unwrap();
    demo::logs_sample(opts);
}
