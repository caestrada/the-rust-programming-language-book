// To enable command line args we import the env lib.
// We bring the 'env' module in order to so that we could use the 'args' 
// function.
use std::env;

use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let _contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");
}

struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
}
