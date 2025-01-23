use std::env;
use std::process;

use minigrep::run;
use minigrep::Config;

// A mini grep search CLI to search a text file and return the lines that the query term was included in.

fn main() {
    let args: Vec<String> = env::args().collect();

    // read from file
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
