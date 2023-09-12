use std::{error::Error, fs, env};

pub const CASE_INSENSITIVE_FLAG: &str = "-i";

pub struct Config {
  pub query: String,
  pub file_path: String,
  pub ignore_case: bool,
}

impl Config {
  pub fn build(args: Vec<String>) -> Result<Config, &'static str> {
    let mut args: Vec<String> = args;

    if args.len() < 3 {
      return Err("Not enough arguments")
    }

    if args.contains(&CASE_INSENSITIVE_FLAG.to_string()) {
      std::env::set_var("IGNORE_CASE", "1");
      if let Some(i) = args.iter().position(|s| s == &CASE_INSENSITIVE_FLAG.to_string()) {
        args.remove(i);
      }
    }

    let config: Config = Config {
      query: args[1].clone(),
      file_path: args[2].clone(),
      ignore_case: env::var("IGNORE_CASE").is_ok(),
    };

    Ok(config)
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let content: String = fs::read_to_string(config.file_path)?;

  if config.ignore_case {
    for line in search_case_insensitive(&config.query, &content) {
      println!("{line}");
    }
  } else {
    for line in search(&config.query, &content) {
      println!("{line}");
    }
  }

  Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
  let mut result: Vec<&str> = Vec::new();

  for line in content.lines() {
    if line.contains(query) {
      result.push(line.trim_start());
    }
  }

  result
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
  let query = query.to_lowercase();
  let mut result: Vec<&str> = Vec::new();

  for line in content.lines() {
    if line.to_lowercase().contains(&query) {
      result.push(line.trim_start());
    }
  }

  result
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn case_sensitive() {
    let query: &str = "duct";
    let content: &str = "Rust:
      safe, fast, productive
      Pick three.
      Duct tape.";

    assert_eq!(vec!["safe, fast, productive"], search(query, content));
  }

  #[test]
  fn case_isensitive() {
    let query: &str = "rUsT";
    let content: &str = "Rust:
      safe, fast, productive
      Pick three.
      Trust me.";

    assert_eq!(
      vec!["Rust:", "Trust me."],
      search_case_insensitive(query, content)
    );
  }
}
