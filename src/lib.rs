use std::error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("参数不够");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    let content = fs::read_to_string(config.file_path)?;
    for line in search(&config.query, &content) {
      println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str>{
  let mut results = Vec::new();
  for line in content.lines() {
    if line.contains(query) {
      results.push(line);
    }
  }
  results
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str>{
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
  fn on_result(){
    let query = "productive";
    let content = "\
Rust:
fast, safe, productive.
Pick three.";
    assert_eq!(vec!["fast, safe, productive."], search(query, content));
  }

  #[test]
  fn case_sensitive(){
    let query = "productive";
    let content = "\
Rust:
fast, safe, productive.
Pick three.";
    assert_eq!(vec!["fast, safe, productive."], search(query, content));
  }

  #[test]
  fn case_insensitive(){
    let query = "Rust";
    let content = "\
Rust:
fast, safe, productive.
Trust me.";
    assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, content));
  }
}