
use std::{env, error::Error, fs};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_args(&args).unwrap_or_else(|error| {
        println!("Error: {}", error);
        process::exit(1);
    });
    
    println!("Searching {}\nfor {}", &config.file, &config.query);


    let contents = fs::read_to_string(config.file).unwrap();

    println!("{}", contents)

}

struct Config {
    query: String,
    file: String,
}

fn parse_args(args: &[String]) -> Result<Config, &str> {

    if args.len() < 3 {
        return Err("Not enough arguments")
    }

    let query = args[1].clone();
    let file = args[2].clone();

    Ok(Config{query, file})
}