use std::error::Error;
use std::{env, fs};

#[cfg(test)]
mod tests;

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}
impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = args.next().ok_or("Didn't get a query string")?;
        let file_path = args.next().ok_or("Didn't get a file path")?;

        let ignore_case_local = args.next().is_some();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case: ignore_case_local || ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    let mut results = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
