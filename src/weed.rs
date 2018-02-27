use ast::*;
use std::process::exit;

/// Weeds the tree for incorrect break, continue, and blank identifier usage
pub fn weed_ast(root: &Program){
    for node in root.declarations.iter() {
        match node.top_level_declaration {
            TopLevelDeclaration::FunctionDeclaration { ref name, ref parameters, ref return_kind, ref body } => {
                for stmt in body.iter() {
                    check_for_correct_break_and_continue_usage(stmt);
                    traverse_stmt_for_invalid_blank(stmt);
                }

                check_blank_func_decl(name, parameters, return_kind, body, node.line_number)
            },
            TopLevelDeclaration::VarDeclarations { ref declarations } => {
                for decl in declarations.iter() {
                    check_blank_var_decl(&decl);
                }
            },
            _ => continue
        }
    }
}


/*
BREAK/CONTINUE USAGE WEED FUNCTIONS
========================================= */

/// Checks for instances where break and/or continue are not used within a loop context
fn check_for_correct_break_and_continue_usage(stmt: &StatementNode){
    match stmt.statement {
        Statement::Block(ref v) => {
            for x in v {
                check_for_correct_break_and_continue_usage(&x)
            }
        },
        Statement::If { ref init, ref condition, ref if_branch, ref else_branch } => {
            check_for_correct_break_and_continue_usage(&*init);

            for x in if_branch {
                check_for_correct_break_and_continue_usage(&x);
            }

            match else_branch {
                &Some(ref else_branch) => check_for_correct_break_and_continue_usage(&*else_branch),
                &None => return,
            }
        },
        Statement::For{ref init, ref condition, ref post, ref body} => {
            check_for_correct_break_and_continue_usage(&*init);
            check_for_correct_break_and_continue_usage(&*post);
        },
        Statement::Break | Statement::Continue =>  {
            eprintln!("Error: line {}: misused break or continue statement. Must be used within a loop statement.", stmt.line_number);
            exit(1);
        },
        _ => return
    }
}

/*
BLANK IDENTIFIER USAGE WEED FUNCTIONS
========================================= */

/// Checks if any blanks exist on rhs of variable declaration
fn check_blank_var_decl(var_spec: &VarSpec){
    match var_spec.rhs {
        Some(ref vec) => {
            for exp in vec {
                traverse_exp_for_invalid_blank(exp);
            }
        },
        None => return
    }
    match var_spec.kind {
        Some(ref kind) => check_blank_type(kind),
        None => return
    }
}

/// Checks a functions name, params and body for any invalid blank identifier usage
fn check_blank_func_decl(name: &String,
                         params: &Vec<Field>,
                         return_kind: &Option<Box<AstKindNode>>,
                         body: &Vec<StatementNode>,
                         line: u32){
/*
    if name == "_" {
        eprintln!("Error: line {}: Invalid naming of function. Cannot be blank identifier.", line);
        exit(1);
    }

    for field in params.iter(){
        check_blank_field(field);
    }
*/

    match return_kind {
        &Some(ref return_kind) => check_blank_type(return_kind),
        &None => return
    }

    /*
    for stmt in body.iter(){
        traverse_stmt_for_invalid_blank(stmt)
    }
    */
}

/// Checks a type for blank identifier usage
fn check_blank_type(kind: &Box<AstKindNode>) {
    match kind.ast_kind {
        AstKind::Identifier { ref name } => {
            if name == "_" {
                eprintln!("Error: line {}: Invalid type name. Cannot be blank identifier.", kind.line_number);
                exit(1);
            }
        }
        AstKind::Slice { ref base } => return,
        AstKind::Array { ref base, ref size } => return,
        AstKind::Struct { ref fields } => return
    }
}

/// Checks if any of the field identifiers are the blank identifier
fn check_blank_field(field: &Field){
    for id in field.identifiers.iter(){
        if id == "_" {
            eprintln!("Error: line {}: Invalid parameter. Cannot be blank identifier.", field.line_number);
            exit(1);
        }
    }
}

/// Recursively traverses statements to detect any invalid blank id usage
fn traverse_stmt_for_invalid_blank(stmt: &StatementNode){
    match stmt.statement {
        Statement::Block(ref v) => {
            for x in v {
                traverse_stmt_for_invalid_blank(&x)
            }
        },
        Statement::Expression(ref exp) => {
            traverse_exp_for_invalid_blank(&*exp)
        },
        Statement::Assignment {ref lhs, ref rhs} => {
            for exp in rhs.iter(){
                traverse_exp_for_invalid_blank(exp)
            }
        },
        Statement::OpAssignment { ref lhs, ref rhs, ref operator } => {
            traverse_exp_for_invalid_blank(&*rhs)
        },
        Statement::VarDeclarations { ref declarations } => {
            for decl in declarations.iter(){
                check_blank_var_decl(decl)
            }
        },
        Statement::ShortVariableDeclaration { ref identifier_list, ref expression_list } => {
            for exp in expression_list.iter(){
                traverse_exp_for_invalid_blank(exp)
            }
        },
        Statement::IncDec { ref is_dec, ref expr } => {
            traverse_exp_for_invalid_blank(&*expr)
        },
        Statement::Print { ref exprs } => {
            for exp in exprs.iter(){
                traverse_exp_for_invalid_blank(exp)
            }
        },
        Statement::Println { ref exprs } => {
            for exp in exprs.iter(){
                traverse_exp_for_invalid_blank(exp)
            }
        },
        Statement::If { ref init, ref condition, ref if_branch, ref else_branch } => {
            traverse_stmt_for_invalid_blank(&*init);
            traverse_exp_for_invalid_blank(&*condition);

            for stmt in if_branch.iter() {
                traverse_stmt_for_invalid_blank(stmt)
            }

            match else_branch {
                &Some(ref else_branch) => traverse_stmt_for_invalid_blank(&*else_branch),
                &None => return,
            }
        },
        Statement::Loop { ref body } => {
            for stmt in body.iter() {
                traverse_stmt_for_invalid_blank(stmt)
            }
        },
        Statement::While { ref condition, ref body } => {
            traverse_exp_for_invalid_blank(&*condition);

            for stmt in body.iter() {
                traverse_stmt_for_invalid_blank(stmt)
            }
        },
        Statement::For { ref init, ref condition, ref post, ref body } => {
            traverse_stmt_for_invalid_blank(&*init);
            traverse_exp_for_invalid_blank(&*condition);
            traverse_stmt_for_invalid_blank(&*post);

            for stmt in body.iter() {
                traverse_stmt_for_invalid_blank(stmt)
            }
        },
        Statement::Switch { ref init, ref expr, ref body } => {
            traverse_stmt_for_invalid_blank(&*init);

            match expr {
                &Some(ref expr) => traverse_exp_for_invalid_blank(&*expr),
                &None => ()
            }

            for case_clause in body.iter() {
                traverse_case_clause_for_invalid_blank(case_clause)
            }
        },
        Statement::Return( ref expr) => {
            match expr {
                &Some( ref expr ) => traverse_exp_for_invalid_blank(&*expr),
                &None => ()
            }
        },
        _ => return
    }
}


/// Recursively traverses expression in order to detect any invalid blank id usage
fn traverse_exp_for_invalid_blank(exp: &ExpressionNode){
    match exp.expression {
        Expression::Identifier { ref name } => {
            if name == "_" {
                eprintln!("Error: line {}: invalid use of blank identifier within expression.", exp.line_number);
                exit(1);
            }
        },
        Expression::RawLiteral { ref value } => {
            if value == "_" {
                eprintln!("Error: line {}: invalid use of blank identifier within expression.", exp.line_number);
                exit(1);
            }
        },
        Expression::BinaryOperation { ref op, ref lhs, ref rhs } => {
            traverse_exp_for_invalid_blank(&*lhs);
            traverse_exp_for_invalid_blank(&*rhs);

        },
        Expression::UnaryOperation { ref op, ref rhs } => {
            traverse_exp_for_invalid_blank( &*rhs);
        }
        Expression::Index { ref primary, ref index } => {
            traverse_exp_for_invalid_blank( &*primary);
            traverse_exp_for_invalid_blank( &*index);
        }
        Expression::Selector { ref primary, ref name } => {
            if name == "_" {
                eprintln!("Error: line {}: invalid use of blank identifier within selector.", exp.line_number);
                exit(1);
            }

            traverse_exp_for_invalid_blank(&*primary)
        }
        Expression::FunctionCall {ref primary, ref arguments } => {
            traverse_exp_for_invalid_blank(&*primary);

            for arg in arguments.iter() {
                traverse_exp_for_invalid_blank( &arg);
            }
        }
        Expression::Append { ref lhs, ref rhs } => {
            traverse_exp_for_invalid_blank( &*rhs);
        }
        Expression::TypeCast { ref expr } => {
            traverse_exp_for_invalid_blank( &*expr);
        }
    }
}


/// Traverses through the statements and expressions that make up a case clause to detect any invalid blank id usage
fn traverse_case_clause_for_invalid_blank(case_clause: &CaseClause){
    match case_clause.switch_case {
        SwitchCase::Cases(ref vec) => {
            for expr in vec.iter() {
                traverse_exp_for_invalid_blank(expr)
            }
        },
        _ => ()
    }

    for stmt in case_clause.statements.iter() {
        traverse_stmt_for_invalid_blank(stmt)
    }
}





