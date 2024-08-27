use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut results = Vec::new();

    let contents = fs::read_to_string(config.file_path)?;
    let query = config.query;

//    println!("With text:\n{contents}");

    for line in contents.lines() {
        println!("line:\n{line}");
        if line.contains(&query) {
            println!("Process the lines before {query}");
            results.push(line);
        } else {
            results.push(line);
        }
    }

    Ok(())
}
