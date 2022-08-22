// To enable command line args we import the env lib.
// We bring the 'env' module in order to so that we could use the 'args' 
// function.
use std::env;

use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, file_path) = parse_config(&args);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}
