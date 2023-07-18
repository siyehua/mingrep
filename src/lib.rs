use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub search_text: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Input arguments isn't right! Please enter two arguments, first is search text and second is file_path");
        }
        let search_text = args[1].clone();
        let file_path = args[2].clone();
        // 读取环境变量 env::var(环境变量 key)
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            search_text,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.file_path)?;
    let result = if config.ignore_case {
        search_ignore_cast(config.search_text.as_str(), content.as_str())
    } else {
        search(config.search_text.as_str(), content.as_str())
    };
    println!("the {} content is: ", config.file_path);
    // println!("{}", content);
    for item in result {
        println!("result: {}", item);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    return result;
}

pub fn search_ignore_cast<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines() {
        let line1 = line.to_lowercase();
        let query1 = query.to_lowercase();
        if line1.contains(query1.as_str()) {
            result.push(line);
        }
    }
    return result;
}