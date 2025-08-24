use std::env;
use std::process::exit;

mod ast;
mod ast_constructors;
mod weed;
mod pretty;
mod kind;
mod symbol_table;
mod typecheck;
mod util;
mod codegen;
mod codegen_c;
mod value;
mod interpret;



pub use ast_constructors::*;
pub use weed::*;
pub use pretty::*;
//pub use symbol_table::*;
//pub use symbol_table::*;
//pub use typecheck::*;

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
        match ast {
            Some(ast) => {
                weed::weed_ast(&ast);
                println!("OK");
            },
            None =>  eprintln!("Error: AST error")
        }
    } else if &argv[1] == "print" {
        let ast = unsafe { Box::from_raw(parse()) };
        println!("{:?}", ast);
    } else if &argv[1] == "pretty" {
        let ast = unsafe { Box::from_raw(parse()) };
        weed::weed_ast(&ast);
        pretty::pretty_print_program(&ast)
    } else if &argv[1] == "rename" {
        let mut ast = unsafe { Box::from_raw(parse()) };
        weed::weed_ast(&ast);
        weed::weed_terminating_statements(&ast);
        typecheck::typecheck(&mut ast, false, false );
        pretty::pretty_print_program(&ast)
    } else if &argv[1] == "obfuscate" {
        let mut ast = unsafe { Box::from_raw(parse()) };
        weed::weed_ast(&ast);
        weed::weed_terminating_statements(&ast);
        typecheck::typecheck(&mut ast, false, true );
        pretty::pretty_print_program(&ast)
    } else if &argv[1] == "symbol" {
        let mut ast = unsafe { Box::from_raw(parse()) };
        weed::weed_ast(&ast);
        weed::weed_terminating_statements(&ast);
        typecheck::typecheck(&mut ast, true, false);
    } else if &argv[1] == "typecheck" {
        let mut ast = unsafe { Box::from_raw(parse()) };
        weed::weed_ast(&ast);
        weed::weed_terminating_statements(&ast);
        typecheck::typecheck(&mut ast, false, false);
        print!("OK");
    } else if &argv[1] == "interpret" {
        let mut ast = unsafe { Box::from_raw(parse()) };
        weed::weed_ast(&ast);
        weed::weed_terminating_statements(&ast);
        typecheck::typecheck(&mut ast, false, false);

        interpret::interpret(&mut ast);

    } else if &argv[1] == "codegen" {
        let mut ast = unsafe { Box::from_raw(parse()) };
        weed::weed_ast(&ast);
        weed::weed_terminating_statements(&ast);
        typecheck::typecheck(&mut ast, false, false);

        codegen::codegen(&mut ast);

    } else if &argv[1] == "codegen_c" {
        let mut ast = unsafe { Box::from_raw(parse()) };
        weed::weed_ast(&ast);
        weed::weed_terminating_statements(&ast);
        typecheck::typecheck(&mut ast, false, false);

        codegen_c::codegen(&mut ast);

    } else {
        eprintln!("Error: invalid mode");
        exit(1);
    }
}
