use std::env;
use std::process;

use minigrep::run;
use minigrep::Config;
fn main() {
    //exception process,using unwrap_or_else
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(error) = run(config) {
        eprint!("system occur error:{}", error);
    }
}
