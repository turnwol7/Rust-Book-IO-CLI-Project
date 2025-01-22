use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    // get args
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In File {file_path}");

    // read from file
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
