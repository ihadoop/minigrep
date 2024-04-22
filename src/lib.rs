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

    contents.lines().filter(|v| v.contains(query)).collect()

}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename).unwrap();

    for line in search(&config.str, content.as_str()) {
        println!("{line}")
    }
    Ok(())
}
pub struct Config {
    str: String,
    filename: String,
}

impl Config {
    pub fn build<'a>(mut args: impl Iterator<Item = String>) -> Result<Config, &'a str> {
        args.next();
        //check length
        let query = match args.next() {
            Some(str) => str,
            None => return Err("Didn't get a query string"),
        };

        let file_name = match args.next() {
            Some(str) => str,
            None => return Err("filename parse error"),
        };
        Ok(Config {
            str: query,
            filename: file_name,
        })
    }
}
