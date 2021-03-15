use std::env;
use std::process;

//https://doc.rust-lang.org/stable/book/ch12-04-testing-the-librarys-functionality.html

use minigrep::Config;

const USAGE: &str = "minigrep <search string> <path to file>";

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}. USAGE: {}", err, USAGE);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let path = env::current_dir();
    println!("current_dir {:?}", path);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
