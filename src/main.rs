use std::env;
use std::process;

use minigrep::run;
use minigrep::Config;

// A mini grep search CLI to search a text file and return the lines that the query term was included in.

fn main() {
    //let args: Vec<String> = env::args().collect();

    // read from file
    // print output from std out to text -> cargo run -- to poem.txt > output.txt
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
