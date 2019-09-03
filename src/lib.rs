use std::fs;
use std::error::Error;


pub struct Config {
    pub query: String,
    pub file_name: String,
    pub case_insensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!\nUse <pattern> <filename>\n");
        }

        let query = args[1].clone();
        let file_name = args[2].clone();
        let case_insensitive = std::env::var("CASE_INSENSITIVE").is_ok();
        Ok(Config { query, file_name,  case_insensitive})
    }
}

pub fn search <'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

pub fn search_case_insensitive <'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.file_name)?;
    let result = if config.case_insensitive {
        search_case_insensitive(&config.query, &content)
    } else {
        search(&config.query, &content)
    };
    for line in result {
        println!("{}", line);
    }
    Ok(())
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, content)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

