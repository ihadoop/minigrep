use std::env;
fn main() {
    
    let args:Vec<String>  =env::args().collect();
    
    let search_string   = &args[1];
    let file_name = &args[2];
    println!("filename is:{0},search string is:{1}",file_name,search_string);
}
