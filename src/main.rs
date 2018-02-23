use std::env;
use std::process::exit;

mod ast;
mod ast_constructors;
pub use ast_constructors::*;

extern "C" {
    fn scan();
    fn print_tokens();
    fn parse() -> Box<Program>;
}

#[derive(Debug)]
pub enum Program {
    One { a: i32 },
    Two { a: i32, b: i32 },
}


#[no_mangle]
pub extern "C" fn make_program(a: i32) -> Box<Program> {
    Box::new(Program::One { a: a })
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
        let ast: Box<Program> = unsafe { parse() };
        println!("OK");
    } else {
        eprintln!("Error: invalid mode");
        exit(1);
    }
}
