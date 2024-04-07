use std::env;
use std::error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}
impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("没有获取到参数"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("没有获取到文件路径"),
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    let content = fs::read_to_string(config.file_path)?;
    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &content)
    } else {
        search(&config.query, &content)
    };

    for line in result {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn on_result() {
        let query = "productive";
        let content = "\
Rust:
fast, safe, productive.
Pick three.";
        assert_eq!(vec!["fast, safe, productive."], search(query, content));
    }

    #[test]
    fn case_sensitive() {
        let query = "productive";
        let content = "\
Rust:
fast, safe, productive.
Pick three.";
        assert_eq!(vec!["fast, safe, productive."], search(query, content));
    }

    #[test]
    fn case_insensitive() {
        let query = "Rust";
        let content = "\
Rust:
fast, safe, productive.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, content)
        );
    }
}
