use std::env;
use std::process;

use minigrep::Config;
use minigrep::run;
fn main() {
    let args: Vec<String> = env::args().collect();

    //exception process,using unwrap_or_else
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(error) = run(config){
        print!("system occur error:{}",error);
    }

}

