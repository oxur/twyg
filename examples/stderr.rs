mod common;

use twyg::{LogLevel, OptsBuilder, Output};

use common::demo;

fn main() {
    let opts = OptsBuilder::new()
        .coloured(true)
        .output(Output::Stderr)
        .level(LogLevel::Trace)
        .report_caller(true)
        .build()
        .unwrap();
    demo::logs_sample(opts);
}
