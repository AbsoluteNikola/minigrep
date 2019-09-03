use std::env;

mod lib;
use crate::lib::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Error while parsing arguments: {}", err);
        std::process::exit(1);
    });
    println!("query: {}", config.query);
    println!("file name: {}", config.file_name);
    println!("case insensitive: {}", config.case_insensitive);
    if let Err(err) = run(config) {
        eprintln!("Application error: {}", err);
        std::process::exit(1);
    }
}
