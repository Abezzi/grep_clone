use grep_clone::Config;
use std::env;
use std::process;

fn main() {
    // when we use collect we need to specify the type of the receiver variable
    let args: Vec<String> = env::args().collect();

    // store the configuration and handle errors
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing argument: {err}");
        process::exit(1);
    });

    // println!("searching for: {}", config.query);

    if let Err(e) = grep_clone::run(config) {
        println!("Applicattion error: {e}");
        process::exit(1);
    }
}
