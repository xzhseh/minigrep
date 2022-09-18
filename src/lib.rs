use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // Remember the first argument of the CLI will always be the file name in Rust
        args.next(); // So just skip it because we won't use it this time

        // Note the return type of next() is Option<T>,
        // But what we should return is Result<T, E>
        let query = match args.next() { // Query for the word to find, "hello", "hi"...etc
            Some(arg) => arg,
            None => return Err("Missing the query character/word"),
        };

        let file_path = match args.next() { // The file path to which you wish to grep the word
            Some(arg) => arg,
            None => return Err("Missing the file path to search for"),
        };

        // We only cares about is the environment variable set or not, so basically just check is_ok()
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    println!("Search results:\n---------------");

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // The return vector which contains all possible results generated from contents
    // With the iterator and closure implementation, this search() method now support
    // concurrency within multiple threads because we don't need to keep track of the
    // mutual access to the previous mutable return vector
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    // The return vector which contains all possible results generated from contents
    // The holistic behaviour is the same as search(), read the documentation above
    // for more details
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
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

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "RuSt";
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

