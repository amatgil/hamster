mod overlay;
use std::env::args;

use overlay::bring_up_overlay;

enum ExitCodes {
    ForgotArgument = 10,
}

fn main() {
    let Some(arg) = args().nth(1) else {
        eprintln!("No argument given, exiting...");
        std::process::exit(ExitCodes::ForgotArgument as i32);
    };
    dbg!(arg);
    //bring_up_overlay();
}
