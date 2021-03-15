use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("With text:\n{}", contents);

    println!("==================================");
    for line in search(&config.query, &contents) {
        println!("FOUND: {}", line);
    }
    println!("==================================");

    Ok(())
}

pub fn search(query: &str, contents: &str) -> Vec<std::string::String> {
    let mut results: Vec<std::string::String> = Vec::new();

    let mut count = 0;
    for line in contents.lines() {
        count += 1;
        if line.contains(query) {
            let pre: std::string::String = format!("Ln {}:",count).to_string();
            results.push(pre + line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
