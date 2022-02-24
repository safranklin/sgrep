use std::env;
use std::process;

use sgrep::Args;

fn main() {
    let args = Args::new(env::args()).unwrap_or_else(|error| {
        eprintln!("Problem parsing arguments: {}", error);
        process::exit(1);
    });

    if let Err(err) = sgrep::run(args) {
        eprintln!("Application failure: {}", err);
        process::exit(1);
    }
}
    
