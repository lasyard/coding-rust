use std::env;
use std::fs;

use minigrep::{search, search_case_insensitive};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    // These two results are not the same type
    if config.ignore_case {
        let results = search_case_insensitive(&config.query, &contents);
        for line in results {
            println!("{}", line);
        }
    } else {
        let results = search(&config.query, &contents);
        for line in results {
            println!("{}", line);
        }
    }
    Ok(())
}

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // skip the first argument which is the program name
        let query = args.next().ok_or("Missing query")?;
        let file_path = args.next().ok_or("Missing file path")?;
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
