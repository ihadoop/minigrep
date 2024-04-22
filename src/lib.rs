
use std::fs;
use std::error::Error;

pub fn run(config:Config)->Result<(),Box<dyn Error>>{

    let content  = fs::read_to_string(config.filename);

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
