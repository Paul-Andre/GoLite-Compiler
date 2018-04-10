use ast::*;
use kind;
use kind::Kind;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::os::raw::c_int;

use std::process::exit;


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


pub unsafe fn from_raw_or_none<T>(t: *mut T) -> Option<Box<T>> {
    if t.is_null() {
        None
    } else {
        Some( Box::from_raw(t))
    }
}

/// This macro allows pattern matching with enums
macro_rules! matches {
    ($e:expr, $p:pat) => (
        match $e {
            $p => true,
            _ => false
        }
    )
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

create_vec_functions!(make_field_vec, field_vec_push, Field);
create_vec_functions!(make_case_clause_vec, case_clause_vec_push, CaseClause);
create_vec_functions!(make_top_level_declaration_vec,
                      top_level_declaration_vec_push,
                      TopLevelDeclarationNode);

create_vec_functions!(make_var_spec_vec, var_spec_vec_push, VarSpec);
create_vec_functions!(make_type_spec_vec, type_spec_vec_push, TypeSpec);

/*
PROGRAM CONSTRUCTOR
=======================================
*/

#[no_mangle]
pub extern "C" fn make_program(pkg: *const c_char,
                                   dcls: *mut Vec<TopLevelDeclarationNode>) -> *mut Program {

    Box::into_raw(Box::new(Program {
        package_name: unsafe { from_c_string(pkg) } ,
        declarations: *unsafe{Box::from_raw(dcls)}
    }))
}


/*
TOP DECLARATION NODE CONSTRUCTORS
=======================================
*/


/// This is a function that factors out most of the repetition from creating top level declaration nodes
fn make_top_level_declaration_ptr(line: u32, dcl: TopLevelDeclaration) -> *mut TopLevelDeclarationNode {
    Box::into_raw(Box::new(TopLevelDeclarationNode {
        line_number: line ,
        top_level_declaration: dcl
    }))
}

#[no_mangle]
pub extern "C" fn make_var_top_level_declaration(line: u32,
                                                 decls:  *mut Vec<VarSpec>) -> *mut TopLevelDeclarationNode {
    make_top_level_declaration_ptr(
        line,
        TopLevelDeclaration::VarDeclarations{ declarations:  *unsafe { Box::from_raw(decls) }}
    )
}

#[no_mangle]
pub extern "C" fn make_type_top_level_declaration(line: u32,
                                                 decls:  *mut Vec<TypeSpec>) -> *mut TopLevelDeclarationNode {
    make_top_level_declaration_ptr(
        line,
        TopLevelDeclaration::TypeDeclarations{ declarations:  *unsafe { Box::from_raw(decls) }}
    )
}

#[no_mangle]
pub extern "C" fn make_function_top_level_declaration(line: u32,
                                                      name:  *const c_char,
                                                      params: *mut Vec<Field>,
                                                      return_kind: *mut AstKindNode,
                                                      body: *mut Vec<StatementNode>) -> *mut TopLevelDeclarationNode {
    make_top_level_declaration_ptr(
        line,
        TopLevelDeclaration::FunctionDeclaration{
            name: unsafe { from_c_string(name) },
            parameters: *unsafe { Box::from_raw(params) },
            return_kind: unsafe { from_raw_or_none(return_kind) },
            body: *unsafe { Box::from_raw(body) }
        }
    )
}



/*
EXPRESSION NODE CONSTRUCTORS
=======================================
*/

/// This is a function that factors out most of the repetition from creating expression nodes
fn make_expr_ptr(line: u32, expr: Expression) -> *mut ExpressionNode {
    Box::into_raw(Box::new(ExpressionNode {
        line_number: line,
        expression: expr,
        kind: kind::Kind::Undefined,
    }))
}

#[no_mangle]
pub extern "C" fn make_identifier_expression(line: u32, string: *const c_char) -> *mut ExpressionNode {
    let name: String = unsafe { from_c_string(string) };
    make_expr_ptr(
        line,
        Expression::Identifier { name: name.clone(), original_name: name }
    )
}


#[no_mangle]
pub extern "C" fn make_literal_expression(
    line: u32,
    string: *const c_char,
    kind: kind::BasicKind,
) -> *mut ExpressionNode {
    Box::into_raw(Box::new(ExpressionNode {
        line_number: line,
        expression: Expression::RawLiteral { value: unsafe { from_c_string(string) } },
        kind: kind::Kind::Basic(kind),
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


#[no_mangle]
pub extern "C" 
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

#[no_mangle]
pub extern "C" 
fn make_unary_operation_expression(line: u32, operator: UnaryOperator, right: *mut ExpressionNode) -> *mut ExpressionNode {
    make_expr_ptr(
        line,
        Expression::UnaryOperation {
            op: operator,
            rhs: unsafe { Box::from_raw(right)},
        },
    )
}

#[no_mangle]
pub extern "C" 
fn make_index_expression(line: u32, p: *mut ExpressionNode, i: *mut ExpressionNode) -> *mut ExpressionNode {
    make_expr_ptr(
        line,
        Expression::Index {
            primary: unsafe{ Box::from_raw(p) },
            index: unsafe{ Box::from_raw(i) },
        },
    )
}

#[no_mangle]
pub extern "C" 
fn make_selector_expression(line: u32, p: *mut ExpressionNode, str: *const c_char) -> *mut ExpressionNode {
    make_expr_ptr(
        line,
        Expression::Selector {
            primary: unsafe { Box::from_raw(p) },
            name: unsafe { from_c_string(str) },
        },
    )
}

#[no_mangle]
pub extern "C" 
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
    let expr  = unsafe {Box::from_raw(expr)};
    match expr.expression {
        Expression::FunctionCall{..} => {
            make_statement_ptr(
                line,
                Statement::Expression(expr)
                )
        }
        _ => {
            eprintln!("Error: line {}: expression statements can only be function calls", line);
            exit(1)
        }
    }
}

#[no_mangle]
pub extern "C" fn make_assignment_statement(line: u32, lhs: *mut Vec<ExpressionNode>,
                                            rhs: *mut Vec<ExpressionNode>) -> *mut StatementNode {

    let lhs = *unsafe{Box::from_raw(lhs)};
    let rhs = *unsafe{Box::from_raw(rhs)};
    if lhs.len() != rhs.len() {
        eprintln!("Error: line {}: lhs and rhs of assignment have a different amount of elements.", line);
        exit(1);
    }

    make_statement_ptr(
        line,
        Statement::Assignment {
            lhs,
            rhs
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
                                                 decls: *mut Vec<VarSpec>) -> *mut StatementNode {
    make_statement_ptr(
        line,
        Statement::VarDeclarations {
            declarations: *unsafe{Box::from_raw(decls)}
        }
    )
}

#[no_mangle]
pub extern "C" fn make_type_declaration_statement(line: u32, decls: *mut Vec<TypeSpec>) -> *mut StatementNode {
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

    let lhs = *unsafe{Box::from_raw(ids)};
    let rhs = *unsafe{Box::from_raw(exprs)};
    if lhs.len() != rhs.len() {
        eprintln!("Error: line {}: lhs and rhs of short declaration have a different number of elements.",line);
        exit(1);
    }

    make_statement_ptr(
        line,
        Statement::ShortVariableDeclaration {
            identifier_list: lhs,
            expression_list: rhs,
        }
    )
}

#[no_mangle]
pub extern "C" fn make_inc_dec_statement(line: u32, is_dec: c_int, expr: *mut ExpressionNode ) -> *mut StatementNode {

    make_statement_ptr(
        line,
        Statement::IncDec {
            is_dec: if is_dec == 0 {false} else {true},
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
    make_statement_ptr(
        line,
        Statement::If {
            init: unsafe{Box::from_raw(init)},
            condition: unsafe{Box::from_raw(cond)},
            if_branch: *unsafe{Box::from_raw(if_branch)},
            else_branch: unsafe{from_raw_or_none(else_branch)},
        }
    )
}

#[no_mangle]
pub extern "C" fn make_for_statement(line: u32,
                                     init: *mut StatementNode,
                                     cond: *mut ExpressionNode,
                                     post: *mut StatementNode,
                                     body: *mut Vec<StatementNode> ) -> *mut StatementNode {
    let post = unsafe{Box::from_raw(post)};
    if let Statement::ShortVariableDeclaration{..} =  post.statement {
        eprintln!("Error: line {}: cannot have short variable declaration in the post condition of loop", line);
        exit(1)
    }
    else {
        make_statement_ptr(
            line,
            Statement::For {
                init: unsafe{Box::from_raw(init)},
                condition: unsafe{from_raw_or_none(cond)},
                post: post,
                body: *unsafe{Box::from_raw(body)}
            }
        )
    }
}


#[no_mangle]
pub extern "C" fn make_switch_statement(line: u32,
                                     init: *mut StatementNode,
                                     expr: *mut ExpressionNode,
                                     body: *mut Vec<CaseClause> ) -> *mut StatementNode {

    let body = *unsafe{Box::from_raw(body)};

    verify_only_one_default(line, &body);

    make_statement_ptr(
        line,
        Statement::Switch {
            init: unsafe{Box::from_raw(init)},
            expr: unsafe{from_raw_or_none(expr)},
            body: body
        }
    )
}

#[no_mangle]
pub extern "C" fn make_break_statement(line: u32) -> *mut StatementNode{
    make_statement_ptr(
        line,
        Statement::Break

    )
}

#[no_mangle]
pub extern "C" fn make_continue_statement(line: u32) -> *mut StatementNode{
    make_statement_ptr(
        line,
        Statement::Continue

    )
}

#[no_mangle]
pub extern "C" fn make_return_statement(line: u32, value: *mut ExpressionNode) -> *mut StatementNode{
    make_statement_ptr(
        line,
        Statement::Return(unsafe{from_raw_or_none(value)})
    )
}


/*
STATEMENT NODE HELPERS
=======================================
*/
#[no_mangle]
pub extern "C" fn make_case_clause(line: u32,
                                   tags: *mut Vec<ExpressionNode>,
                                   stmts: *mut Vec<StatementNode>) -> *mut CaseClause {

    let tag: SwitchCase;

    if tags.is_null() {
        tag = SwitchCase::Default;
    } else {
        tag = SwitchCase::Cases( *unsafe{Box::from_raw(tags)} )
    }

    Box::into_raw(Box::new(CaseClause {
        line_number: line,
        switch_case: tag,
        statements: *unsafe{Box::from_raw(stmts)}
    }))
}

#[no_mangle]
pub extern "C" fn make_var_spec(line: u32, names: *mut Vec<String>, kind: *mut AstKindNode, rhs: *mut Vec<ExpressionNode>) 
    -> *mut VarSpec
{
    let names = *unsafe { Box::from_raw( names ) };
    if !rhs.is_null() {
        let rhs = *unsafe { Box::from_raw( rhs ) };
        if names.len() != rhs.len() {
            eprintln!("Error: line {}: different number of elements on the sides or the assignment", line);
            exit(1);
        }

        Box::into_raw( Box::new(
                VarSpec{
                    line_number: line,
                    names,
                    kind: unsafe{ from_raw_or_none(kind) },
                    rhs: Some(rhs),
                    evaluated_kind: Kind::Undefined
                }))
    } else {
        Box::into_raw( Box::new(
                VarSpec{
                    line_number: line,
                    names,
                    kind: unsafe{ from_raw_or_none(kind) },
                    rhs: None,
                    evaluated_kind: Kind::Undefined
                }))
    }
}

#[no_mangle]
pub extern "C" 
fn make_type_spec(line: u32, name: *mut c_char, kind: *mut AstKindNode)
    -> *mut TypeSpec
{
        Box::into_raw( Box::new(
                TypeSpec{
                    line_number: line,
                    name: unsafe{ from_c_string(name) },
                    kind: unsafe{ Box::from_raw(kind) },
                }))
}

/// Verify that only one default exists in any switch clause
fn verify_only_one_default(line: u32, body: &Vec<CaseClause>) {
    let mut default_exists: bool = false;

    for case in body.iter() {
        if matches!(case.switch_case, SwitchCase::Default) && !default_exists {
            default_exists = true;
            //eprintln!("one exists");
        } else if matches!(case.switch_case, SwitchCase::Default) {
            eprintln!("Error: line {}: declared more than one default switch case.",line);
            exit(1);
        }
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


        

