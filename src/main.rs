use std::env;
use std::fs;
use std::process;

fn main() {
    // when we use collect we need to specify the type of the receiver variable
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing argument: {err}");
        process::exit(1);
    });

    println!("searching for: {}", config.query);
    println!("in file: {}", config.file_path);

    run(config)
}

fn run(config: Config) {
    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");
    println!("content of the file: {contents}");
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

struct Config {
    query: String,
    file_path: String,
}
