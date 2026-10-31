use ttk::prelude::*;

fn main() {
    logfile::init("ttk-cmd-demo");
    print::info(1, "Welcome to the TTK command demo!");

    print::ln();
    cmd::run!(2, "echo", "Hello, World!");

    print::ln();
    cmd::run!(2, "does-not-exist");
}
