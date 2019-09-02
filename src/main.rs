use std::env;
use std::fs;

mod lib;
use crate::lib::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error: {}", err);
        std::process::exit(1);
    });
    let content = fs::read_to_string(&config.file_name).expect("Error :)");
    println!("query: {}", config.query);
    println!("file name: {}", config.file_name);
    println!("\n\ncontent:\n{}", content);
}
