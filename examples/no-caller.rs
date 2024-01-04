mod common;

use twyg::Opts;

use common::demo;

fn main() {
    let opts = Opts {
        coloured: true,
        file: None,
        level: Some(String::from("trace")),
        report_caller: false,

        ..Default::default()
    };
    demo::logs_sample(opts);
}
