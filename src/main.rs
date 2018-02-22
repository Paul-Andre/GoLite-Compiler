
extern "C" {
    fn say_hello();
    fn scan();
    fn tokens();
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
    /*
    unsafe {
        tokens();
    }
    */
    let p = unsafe { parse() };
    println!("{:?}", p);
}
