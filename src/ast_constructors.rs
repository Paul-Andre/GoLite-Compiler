use ast::*;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::os::raw::c_int;


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
create_vec_functions!(make_expression_vec, expression_vec_push, ExpressionNode);
create_vec_functions!(make_string_vec, string_vec_push, String);

//Statement vectors
create_vec_functions!(make_statement_vec, statement_vec_push, StatementNode);

create_vec_functions!(make_field_vec, field_vec_push, StatementNode);




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
    operator: BinaryOperator,
    left: *mut ExpressionNode,
    right: *mut ExpressionNode,
) -> *mut ExpressionNode {
    make_expr_ptr(
        line,
        Expression::BinaryOperation {
            op: operator,
            lhs: unsafe { Box::from_raw(left) },
            rhs: unsafe { Box::from_raw(right) },
        },
    )
}

fn make_unary_operation_expression(line: u32, operator: UnaryOperator, right: *mut ExpressionNode) -> *mut ExpressionNode {
    make_expr_ptr(
        line,
        Expression::UnaryOperation {
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
    make_statement_ptr(
        line,
        Statement::Empty
    )
}

#[no_mangle]
pub extern "C" fn make_block_statement(line: u32, stmts: *mut Vec<StatementNode>) -> *mut StatementNode {
    make_statement_ptr(
        line,
        Statement::Block(*unsafe {Box::from_raw(stmts)})
    )
}

#[no_mangle]
pub extern "C" fn make_expression_statement(line: u32, expr: *mut ExpressionNode) -> *mut StatementNode {
    make_statement_ptr(
        line,
        Statement::Expression(unsafe {Box::from_raw(expr)})
    )
}

#[no_mangle]
pub extern "C" fn make_assignment_statement(line: u32, lhs: *mut Vec<ExpressionNode>,
                                            rhs: *mut Vec<ExpressionNode>) -> *mut StatementNode {
    make_statement_ptr(
        line,
        Statement::Assignment {
            lhs: *unsafe{Box::from_raw(lhs)},
            rhs: *unsafe{Box::from_raw(rhs)}
        }
    )
}

#[no_mangle]
pub extern "C" fn make_op_assignment_statement(line: u32, lhs: *mut ExpressionNode,
                                               rhs: *mut ExpressionNode,
                                               op: BinaryOperator) -> *mut StatementNode {
    make_statement_ptr(
        line,
        Statement::OpAssignment {
            lhs: unsafe{Box::from_raw(lhs)},
            rhs: unsafe{Box::from_raw(rhs)},
            operator: op
        }
    )
}

#[no_mangle]
pub extern "C" fn make_var_declaration_statement(line: u32,
                                                 decls: *mut Vec<VarDeclaration>) -> *mut StatementNode {
    make_statement_ptr(
        line,
        Statement::VarDeclarations {
            declarations: *unsafe{Box::from_raw(decls)}
        }
    )
}

#[no_mangle]
pub extern "C" fn make_type_declaration_statement(line: u32, decls: *mut Vec<TypeDeclaration>) -> *mut StatementNode {
    make_statement_ptr(
        line,
        Statement::TypeDeclarations {
            declarations: *unsafe{Box::from_raw(decls)}
        }
    )
}

#[no_mangle]
pub extern "C" fn make_short_var_declaration_statement(line: u32, ids: *mut Vec<String>,
                                                       exprs: *mut Vec<ExpressionNode> ) -> *mut StatementNode {
    make_statement_ptr(
        line,
        Statement::ShortVariableDeclaration {
            identifier_list: *unsafe{Box::from_raw(ids)},
            expression_list: *unsafe{Box::from_raw(exprs)}
        }
    )
}

#[no_mangle]
pub extern "C" fn make_inc_dec_statement(line: u32, is_dec: c_int, expr: *mut ExpressionNode ) -> *mut StatementNode {
    make_statement_ptr(
        line,
        Statement::IncDec {
            if c_int == 0 {false} else {true},
            expr: unsafe{Box::from_raw(expr)}
        }
    )
}

#[no_mangle]
pub extern "C" fn make_print_statement(line: u32, exprs: *mut Vec<ExpressionNode> ) -> *mut StatementNode {
    make_statement_ptr(
        line,
        Statement::Print {
            exprs: *unsafe{Box::from_raw(exprs)}
        }
    )
}

#[no_mangle]
pub extern "C" fn make_println_statement(line: u32, exprs: *mut Vec<ExpressionNode> ) -> *mut StatementNode {
    make_statement_ptr(
        line,
        Statement::Println {
            exprs: *unsafe{Box::from_raw(exprs)}
        }
    )
}

#[no_mangle]
pub extern "C" fn make_if_statement(line: u32,
                                    init: *mut StatementNode,
                                    cond: *mut ExpressionNode,
                                    if_branch: *mut Vec<StatementNode>,
                                    else_branch: *mut StatementNode ) -> *mut StatementNode {

    if else_branch.is_null() {
        make_statement_ptr(
            line,
            Statement::If {
                init: unsafe{Box::from_raw(init)},
                condition: unsafe{Box::from_raw(cond)},
                if_branch: *unsafe{Box::from_raw(if_branch)},
                else_branch: None
            }
        )
    } else {
        make_statement_ptr(
            line,
            Statement::If {
                init: unsafe{Box::from_raw(init)},
                condition: unsafe{Box::from_raw(cond)},
                if_branch: *unsafe{Box::from_raw(if_branch)},
                else_branch: Some(unsafe{Box::from_raw(else_branch)})
            }
        )
    }
}


#[no_mangle]
pub extern "C" fn make_loop_statement(line: u32, body: *mut Vec<StatementNode> ) -> *mut StatementNode {
    make_statement_ptr(
        line,
        Statement::Loop {
            body: *unsafe{Box::from_raw(body)}
        }
    )
}

#[no_mangle]
pub extern "C" fn make_while_statement(line: u32, cond: *mut ExpressionNode, body: *mut Vec<StatementNode> ) -> *mut StatementNode {
    make_statement_ptr(
        line,
        Statement::While {
            condition: unsafe{Box::from_raw(cond)},
            body: *unsafe{Box::from_raw(body)}
        }
    )
}

#[no_mangle]
pub extern "C" fn make_for_statement(line: u32,
                                     init: *mut StatementNode,
                                     cond: *mut ExpressionNode,
                                     post: *mut StatementNode,
                                     body: *mut Vec<StatementNode> ) -> *mut StatementNode {
    make_statement_ptr(
        line,
        Statement::For {
            init: unsafe{Box::from_raw(init)},
            condition: unsafe{Box::from_raw(cond)},
            post: unsafe{Box::from_raw(post)},
            body: *unsafe{Box::from_raw(body)}
        }
    )
}


#[no_mangle]
pub extern "C" fn make_switch_statement(line: u32,
                                     init: *mut StatementNode,
                                     expr: *mut ExpressionNode,
                                     body: *mut Vec<CaseClause> ) -> *mut StatementNode {

    if expr.is_null() {
        make_statement_ptr(
            line,
            Statement::Switch {
                init: unsafe{Box::from_raw(init)},
                expr: None,
                body: *unsafe{Box::from_raw(body)}
            }
        )
    } else {
        make_statement_ptr(
            line,
            Statement::Switch {
                init: unsafe{Box::from_raw(init)},
                expr: Some(unsafe{Box::from_raw(expr)}),
                body: *unsafe{Box::from_raw(body)}
            }
        )
    }
}

pub extern "C" fn make_break_statement(line: u32) -> *mut StatementNode{
    make_statement_ptr(
        line,
        Statement::Break

    )
}

pub extern "C" fn make_continue_statement(line: u32) -> *mut StatementNode{
    make_statement_ptr(
        line,
        Statement::Continue

    )
}

pub extern "C" fn make_return_statement(line: u32, value: *mut ExpressionNode) -> *mut StatementNode{
    if value.is_null() {
        make_statement_ptr(
            line,
            Statement::Return(None)
        )
    } else {
        make_statement_ptr(
            line,
            Statement::Return(Some(unsafe{Box::from_raw(value)}))
        )
    }
    
}



/*
AST KIND NODE CONSTRUCTORS
=======================================
*/


/// This is a function that factors out most of the repetition
fn make_ast_kind_ptr(line: u32, expr: AstKind) -> *mut AstKindNode {
    Box::into_raw(Box::new( AstKindNode{
        line_number: line,
        ast_kind: expr,
    }))
}

#[no_mangle]
pub extern "C" fn make_identifier_kind(line: u32, string: *const c_char) -> *mut AstKindNode {
    make_ast_kind_ptr(
        line,
        AstKind::Identifier { name: unsafe { from_c_string(string) } },
    )
}


#[no_mangle]
pub extern "C" fn make_slice_kind(line: u32, base: *mut AstKindNode) -> *mut AstKindNode {
    make_ast_kind_ptr(
        line,
        AstKind::Slice { base: unsafe { Box::from_raw(base) } },
    )
}

#[no_mangle]
pub extern "C" fn make_array_kind(line: u32, base: *mut AstKindNode, size: *const c_char) -> *mut AstKindNode {
    make_ast_kind_ptr(
        line,
        AstKind::Array {
            base: unsafe { Box::from_raw(base) },
            size: unsafe { from_c_string(size) },
        },
    )
}

#[no_mangle]
pub extern "C" fn make_struct_kind(line: u32, fields: *mut Vec<Field>) -> *mut AstKindNode {
    make_ast_kind_ptr(
        line,
        AstKind::Struct {
            fields: *unsafe{ Box::from_raw(fields) }
        },
    )
}

#[no_mangle]
pub extern "C" fn make_field(line: u32, fields: *mut Vec<String>, kind: *mut AstKindNode)
-> *mut Field
{
    Box::into_raw( Box::new(
            Field {
                line_number: line,
                identifiers: *unsafe{ Box::from_raw(fields) },
                kind: unsafe{ Box::from_raw(kind) }
            }))
}


        

