mod common;

use twyg::{level, Opts};

use common::demo;

fn main() {
    let opts = Opts {
        coloured: true,
        level: level::trace(),
        report_caller: true,

        ..Default::default()
    };
    demo::logs_sample(opts);
}
