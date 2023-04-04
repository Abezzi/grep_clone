use std::env;
use std::fs;

fn main() {
    // when we use collect we need to specify the type of the receiver variable
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("searching for: {query}");
    println!("in file: {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("content of the file: {contents}");
}
