mod common;

use twyg::{LogLevel, OptsBuilder};

use common::demo;

fn main() {
    let opts = OptsBuilder::new()
        .coloured(false)
        .level(LogLevel::Trace)
        .report_caller(false)
        .build()
        .unwrap();
    demo::logs_sample(opts);
}
