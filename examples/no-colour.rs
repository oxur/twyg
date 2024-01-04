mod common;

use twyg::{level, Opts};

use common::demo;

fn main() {
    let opts = Opts {
        coloured: false,
        level: level::trace(),
        report_caller: false,

        ..Default::default()
    };
    demo::logs_sample(opts);
}
