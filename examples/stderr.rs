mod common;

use common::demo;

fn main() {
    let opts = twyg::LoggerOpts {
        coloured: true,
        file: Some("stderr".to_string()),
        level: "trace".to_string(),
        report_caller: true,
    };
    demo::logs_sample(opts);
}
