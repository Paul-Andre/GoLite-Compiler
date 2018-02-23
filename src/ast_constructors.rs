use ast::*;
use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn exp_identifier(line: u32, string: *const c_char) -> *mut ExpressionNode {
    let copied_string = unsafe { CStr::from_ptr(string) }.to_str().unwrap().into();
    Box::into_raw(Box::new(ExpressionNode {
        location: SourceLocation { line_number: line },
        expression: Expression::Identifier { name: copied_string },
        kind: Kind::Undefined,
    }))
}

#[no_mangle]
pub extern "C" fn make_string(string: *const c_char) -> *mut String {
    Box::into_raw(Box::new(
            unsafe { CStr::from_ptr(string) }.to_str().unwrap().into()));
}

macro_rules! create_vec_functions {
    ($make_name:ident, $push_name:ident,  $T:ty) => {
        #[no_mangle]
        pub extern "C" fn $make_name() -> *mut Vec<$T> {
            Box::into_raw(Box::new(Vec::new()))
        }
        
        #[no_mangle]
        pub extern "C" fn $push_name(vec_ptr: *mut Vec<$T>, t_ptr: *mut $T) {
            unsafe{&mut *vec_ptr}.push(*unsafe{Box::from_raw(t_ptr)});
        }
    }
}

create_vec_functions!(make_expr_vec, expr_vec_push, ExpressionNode);
create_vec_functions!(make_string_vec, string_vec_push, String);

/*
fn exp_rawliteral(line: u32, str: String) -> Box<ExpressionNode> {
    Box::new(ExpressionNode {location: line, expression: Expression::RawLiteral{ value: str }})
}

fn exp_binoperation(line: u32, str: operator, left: Box<ExpressionNode>, right: Box<ExpressionNode>) -> Box<ExpressionNode> {
    Box::new(
        ExpressionNode {
            location: line, 
            expression: Expression::BinOperation{ op: str, lhs: left, rhs: right }
        }
    )
}

fn exp_unoperation(line: u32, str: operator, right: Box<ExpressionNode>) -> Box<ExpressionNode> {
    Box::new(
        ExpressionNode {
            location: line, 
            expression: Expression::UnOperation{ op: str, rhs: right }
        }
    )
}

fn exp_index(line: u32, p: Box<ExpressionNode>, i: Box<ExpressionNode>) -> Box<ExpressionNode> {
    Box::new(
        ExpressionNode {
            location: line, 
            expression: Expression::Index{ primary: p, index: i }
        }
    )
}

fn exp_selector(line: u32, p: Box<ExpressionNode>, str: String) -> Box<ExpressionNode> {
    Box::new(
        ExpressionNode {
            location: line, 
            expression: Expression::Selector{ primary: p, name: str }
        }
    )
}

fn exp_functioncall(line: u32, p: Box<ExpressionNode>, args: Vec<ExpressionNode>) -> Box<ExpressionNode> {
    Box::new(
        ExpressionNode {
            location: line, 
            expression: Expression::FunctionCall{ primary: p, arguments: args }
        }
    )
}

fn exp_append(line: u32, left: Box<ExpressionNode>, right: Box<ExpressionNode>) -> Box<ExpressionNode> {
    Box::new(
        ExpressionNode {
            location: line, 
            expression: Expression::Append{ lhs: left, rhs: right }
        }
    )
}

fn exp_typecast(line: u32, exp: Box<ExpressionNode>) -> Box<ExpressionNode> {
    Box::new(
        ExpressionNode {
            location: line, 
            expression: Expression::TypeCast{ expr: exp }
        }
    )
}
*/
