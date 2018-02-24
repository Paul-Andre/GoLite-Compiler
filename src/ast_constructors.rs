use ast::*;
use std::ffi::CStr;
use std::os::raw::c_char;


/// This function turns a C string into a Rust String
/// If the C string isn't proper utf-8 (shouldn't be a problem for us), it panics
unsafe fn from_c_string(s: *const c_char) -> String {
    CStr::from_ptr(s).to_str().unwrap().into()
}

/// This function will be used in C to turn a C string into a Rust String
#[no_mangle]
pub extern "C" fn make_string(string: *const c_char) -> *mut String {
    Box::into_raw(Box::new(unsafe { from_c_string(string) }))
}

/// This is a macros to generate functions that will be used to generate vectors in C.
/// It assumes that the type that is used will be passed as an opaque pointer in C.
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

// Generate the functions to be used in C
create_vec_functions!(make_expr_vec, expr_vec_push, ExpressionNode);
create_vec_functions!(make_string_vec, string_vec_push, String);

/// This is a function that factors out most of the repetition from creating expression nodes
fn make_expr_ptr(line: u32, expr: Expression) -> *mut ExpressionNode {
    Box::into_raw(Box::new(ExpressionNode {
        location: SourceLocation { line_number: line },
        expression: expr,
        kind: Kind::Undefined,
    }))
}

#[no_mangle]
pub extern "C" fn expr_identifier(line: u32, string: *const c_char) -> *mut ExpressionNode {
    make_expr_ptr(
        line,
        Expression::Identifier { name: unsafe { from_c_string(string) } },
    )
}


#[no_mangle]
pub extern "C" fn expr_literal(
    line: u32,
    string: *const c_char,
    kind: BasicKind,
) -> *mut ExpressionNode {
    Box::into_raw(Box::new(ExpressionNode {
        location: SourceLocation { line_number: line },
        expression: Expression::RawLiteral { value: unsafe { from_c_string(string) } },
        kind: Kind::Basic(kind),
    }))
}

#[no_mangle]
pub extern "C" fn expr_append(
    line: u32,
    lhs: *mut ExpressionNode,
    rhs: *mut ExpressionNode,
) -> *mut ExpressionNode {

    make_expr_ptr(
        line,
        Expression::Append {
            lhs: unsafe { Box::from_raw(lhs) },
            rhs: unsafe { Box::from_raw(rhs) },
        },
    )
}




/*
 

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


/*
STATEMENT NODE CONSTRUCTORS
=======================================
*/


/// This is a function that factors out most of the repetition from creating statement nodes
fn make_stmt_ptr(line: u32, expr: Expression) -> *mut ExpressionNode {
    Box::into_raw(Box::new(ExpressionNode {
        location: SourceLocation { line_number: line },
        expression: expr,
        kind: Kind::Undefined,
    }))
}





