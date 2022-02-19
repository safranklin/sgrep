use std::env;
use std::process;

use sgrep::Args;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Shadowing args
    let args = Args::new(&args).unwrap_or_else(|error| {
        eprintln!("Problem parsing arguments: {}", error);
        process::exit(1);
    });

    if let Err(err) = sgrep::run(args) {
        eprintln!("Application failure: {}", err);
        process::exit(1);
    }

}
    
