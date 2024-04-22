use std::error::Error;
use std::fs;
#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn for_one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename).unwrap();

    for line in search(&config.str, content.as_str()) {
        println!("{line}")
    }
    Ok(())
}
pub struct Config<'a> {
    str: &'a String,
    filename: &'a String,
}

impl Config<'_> {
    pub fn build(args: &[String]) -> Result<Config, &str> {
        //check length
        if args.len() < 3 {
            return Err("input args size must eq 2");
        }

        Ok(Config {
            str: &args[1],
            filename: &args[2],
        })
    }
}
