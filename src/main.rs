use std::env;
use std::process::exit;

mod ast;
mod ast_constructors;
mod weed;
mod pretty;

pub use ast_constructors::*;
pub use weed::*;
pub use pretty::*;

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
        if let Some(_) = ast {
            match ast {
                Some(ast) => {
                    weed::weed_ast(&ast);
                    println!("OK");
                },
                None =>  eprintln!("Error: AST error")
            }
        }
    } else if &argv[1] == "print" {
        let ast = unsafe { from_raw_or_none(parse()) };
        println!("{:?}", ast);
    } else if &argv[1] == "pretty" {
        let ast = unsafe { from_raw_or_none(parse()) };
        if let Some(_) = ast {
            match ast {
                Some(ast) => {
                    weed::weed_ast(&ast);

                },
                None =>  eprintln!("Error: AST error")
            }
        }

    }else {
        eprintln!("Error: invalid mode");
        exit(1);
    }
}
