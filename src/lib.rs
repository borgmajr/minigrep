use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string")
        };


        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("With text:\n{}", contents);

    println!("==================================");
    // for line in search(&config.query, &contents) {
    //     println!("FOUND: {}", line);
    // }

    let results = if config.case_sensitive {
        println!(">>>>>> search_case_sensitive");
        search_case_sensitive(&config.query, &contents)
    } else {
        println!(">>>>>> search_case_insensitive");
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    println!("==================================");

    Ok(())
}

pub fn search_case_sensitive(query: &str, contents: &str) -> Vec<std::string::String> {
    let mut results: Vec<std::string::String> = Vec::new();

    let mut count = 0;
    for line in contents.lines() {
        count += 1;
        if line.contains(query) {
            let pre: std::string::String = format!("Ln {}:", count).to_string();
            results.push(pre + line);
        }
    }

    results
}

pub fn search_case_insensitive(query: &str, contents: &str) -> Vec<std::string::String> {
    let query = query.to_lowercase();
    let mut results: Vec<std::string::String> = Vec::new();

    let mut count = 0;
    for line in contents.lines() {
        count += 1;
        if line.to_lowercase().contains(&query) {
            let pre: std::string::String = format!("Ln {}:", count).to_string();
            results.push(pre + line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["Ln 2:safe, fast, productive."],
            search_case_sensitive(query, contents)
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
            vec!["Ln 1:Rust:", "Ln 4:Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
