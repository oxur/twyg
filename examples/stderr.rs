mod common;

use twyg::{level, out, Opts};

use common::demo;

fn main() {
    let opts = Opts {
        coloured: true,
        file: out::stderr(),
        level: level::trace(),
        report_caller: true,

        ..Default::default()
    };
    demo::logs_sample(opts);
}
