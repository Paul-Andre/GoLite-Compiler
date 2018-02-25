use std::env;
use std::process::exit;

mod ast;
mod ast_constructors;
pub use ast_constructors::*;

extern "C" {
    fn scan();
    fn print_tokens();
    fn parse() -> *mut ast::Program;
}


fn main() {

    let argv = env::args().collect::<Vec<String>>();
    if argv.len() < 2 {
        eprintln!("Error: requires mode");
        exit(1);
    }
    if &argv[1] == "scan" {
        unsafe {
            scan();
        }
        println!("OK");
    } else if &argv[1] == "tokens" {
        unsafe {
            print_tokens();
        }
    } else if &argv[1] == "parse" {
        let ast = unsafe { from_raw_or_none(parse()) };
        //println!("{:?}",ast); 
        println!("OK");
    } else {
        eprintln!("Error: invalid mode");
        exit(1);
    }
}
