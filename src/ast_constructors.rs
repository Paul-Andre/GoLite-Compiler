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

//Statement vectors
create_vec_functions!(make_stmt_vec, stmt_vec_push, StatementNode);




/*
EXPRESSION NODE CONSTRUCTORS
=======================================
*/

/// This is a function that factors out most of the repetition from creating expression nodes
fn make_expr_ptr(line: u32, expr: Expression) -> *mut ExpressionNode {
    Box::into_raw(Box::new(ExpressionNode {
        line_number: line,
        expression: expr,
        kind: Kind::Undefined,
    }))
}

#[no_mangle]
pub extern "C" fn make_identifier_expression(line: u32, string: *const c_char) -> *mut ExpressionNode {
    make_expr_ptr(
        line,
        Expression::Identifier { name: unsafe { from_c_string(string) } },
    )
}


#[no_mangle]
pub extern "C" fn make_literal_expression(
    line: u32,
    string: *const c_char,
    kind: BasicKind,
) -> *mut ExpressionNode {
    Box::into_raw(Box::new(ExpressionNode {
        line_number: line,
        expression: Expression::RawLiteral { value: unsafe { from_c_string(string) } },
        kind: Kind::Basic(kind),
    }))
}

#[no_mangle]
pub extern "C" fn make_append_expression(
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


fn make_binary_operation_expression(
    line: u32,
    operator: BinOperator,
    left: *mut ExpressionNode,
    right: *mut ExpressionNode,
) -> *mut ExpressionNode {
    make_expr_ptr(
        line,
        Expression::BinOperation {
            op: operator,
            lhs: unsafe { Box::from_raw(left) },
            rhs: unsafe { Box::from_raw(right) },
        },
    )
}

fn make_unary_operation_expression(line: u32, operator: UnOperator, right: *mut ExpressionNode) -> *mut ExpressionNode {
    make_expr_ptr(
        line,
        Expression::UnOperation {
            op: operator,
            rhs: unsafe { Box::from_raw(right)},
        },
    )
}

fn make_index_expression(line: u32, p: *mut ExpressionNode, i: *mut ExpressionNode) -> *mut ExpressionNode {
    make_expr_ptr(
        line,
        Expression::Index {
            primary: unsafe{ Box::from_raw(p) },
            index: unsafe{ Box::from_raw(i) },
        },
    )
}

fn make_selector_expression(line: u32, p: *mut ExpressionNode, str: *const c_char) -> *mut ExpressionNode {
    make_expr_ptr(
        line,
        Expression::Selector {
            primary: unsafe { Box::from_raw(p) },
            name: unsafe { from_c_string(str) },
        },
    )
}

fn make_function_call_expression(
    line: u32,
    p: *mut ExpressionNode,
    args: *mut Vec<ExpressionNode>
) -> *mut ExpressionNode {
    make_expr_ptr(
        line,
        Expression::FunctionCall {
            primary: unsafe{ Box::from_raw(p) },
            arguments: *unsafe{ Box::from_raw(args) },
        },
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



/*
STATEMENT NODE CONSTRUCTORS
=======================================
*/

/// This is a function that factors out most of the repetition from creating statement nodes
fn make_statement_ptr(line: u32, stmt: Statement) -> *mut StatementNode {
    Box::into_raw(Box::new(StatementNode {
        line_number: line ,
        statement: stmt
    }))
}


#[no_mangle]
pub extern "C" fn make_empty_statement(line: u32) -> *mut StatementNode {
    make_stmt_ptr(
        line,
        Statement::Empty
    )
}

#[no_mangle]
pub extern "C" fn make_block_statement(line: u32, stmts: *mut Vec<ExpressionNode>) -> *mut StatementNode {
    make_stmt_ptr(
        line,
        Statement::Block(stmts)
    )
}

#[no_mangle]
pub extern "C" fn make_expression_statement(line: u32, expr: *mut ExpressionNode) -> *mut StatementNode {
    make_stmt_ptr(
        line,
        Statement::Expression(Box::from_raw(expr))
    )
}

#[no_mangle]
pub extern "C" fn make_assignment_statement(line: u32, lhs: *mut Vec<ExpressionNode>, rhs: *mut Vec<ExpressionNode>) -> *mut StatementNode {
    make_stmt_ptr(
        line,
        Statement::Assignment {
            lhs: *unsafe{Box::from_raw(lhs)},
            rhs: *unsafe{Box::from_raw(rhs)}
        }
    )
}

#[no_mangle]
pub extern "C" fn make_op_assignment_statement(line: u32, lhs: *mut Vec<ExpressionNode>, rhs: *mut Vec<ExpressionNode>) -> *mut StatementNode {
    make_stmt_ptr(
        line,
        Statement::Assignment {
            lhs: *unsafe{Box::from_raw(lhs)},
            rhs: *unsafe{Box::from_raw(rhs)}
        }
    )
}











