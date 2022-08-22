// To enable command line args we import the env lib.
// We bring the 'env' module in order to so that we could use the 'args' 
// function.
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);
}
