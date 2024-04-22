use std::env;
use std::fs;
fn main() {
    
    let args:Vec<String>  =env::args().collect();
    
   let config =parse_config(&args);
   println!("{}",config.filename)

}
fn parse_config(args:&[String])->Config{
    //check length
    if args.len()<2 {
      panic!("");
    }
    
    Config{
        str:&args[1],
        filename:&args[2]
    }

}

struct Config<'a>{
    str:&'a String,
    filename:&'a String
}