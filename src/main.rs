use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);
    println!("{}", config.filename)
}

struct Config<'a> {
    str: &'a String,
    filename: &'a String,
}

impl Config<'_> {
    fn new(args: &[String]) -> Config {
        //check length
        if args.len() < 2 {
            panic!("");
        }

        Config {
            str: &args[1],
            filename: &args[2],
        }
    }
}
