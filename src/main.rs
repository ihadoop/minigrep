use std::env;
use std::process;

use minigrep::run;
use minigrep::Config;
fn main() {
    let args: Vec<String> = env::args().collect();

    //exception process,using unwrap_or_else
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(error) = run(config) {
        eprint!("system occur error:{}", error);
    }
}
