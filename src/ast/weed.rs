use ast::ast::*;
use std::process::exit;

/// Weeds the tree for incorrect break, continue, and blank identifier usage
pub fn weed_ast(root: &Program){
    if &root.package_name == "_" {
        eprintln!("Error: package name cannot be `_`");
        exit(1);
    }
    for node in root.declarations.iter() {
        match node.top_level_declaration {
            TopLevelDeclaration::FunctionDeclaration { ref name, ref parameters, ref return_kind, ref body } => {
                for stmt in body.iter() {
                    check_for_correct_break_and_continue_usage(stmt, false);
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


/*a
BREAK/CONTINUE USAGE WEED FUNCTIONS
========================================= */

/// Checks for correct usage of break and continue
/// Note that `continue` is only valid for loops whereas `break` is balid for loops and switch
fn check_for_correct_break_and_continue_usage(stmt: &StatementNode, can_break: bool){
    match stmt.statement {
        Statement::Block(ref v) => {
            for x in v {
                check_for_correct_break_and_continue_usage(&x, can_break);
            }
        },
        Statement::If { ref if_branch, ref else_branch, .. } => {

            for x in if_branch {
                check_for_correct_break_and_continue_usage(&x, can_break);
            }

            match else_branch {
                &Some(ref else_branch) => check_for_correct_break_and_continue_usage(&*else_branch, can_break),
                &None => return,
            }
        },
        Statement::Switch { ref body, .. } => {

            for case_clause in body {
                for stmt in &case_clause.statements {
                    check_for_correct_break_and_continue_usage(&stmt, true);
                }
            }
        },
        Statement::Break => {
            if !can_break {
                eprintln!("Error: line {}: break outside loop or switch.", stmt.line_number);
                exit(1);
            }
        },
        Statement::Continue => {
            eprintln!("Error: line {}: continue outside loop.", stmt.line_number);
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
    if let Some(ref vec) = var_spec.rhs {
            for exp in vec {
                traverse_exp_for_invalid_blank(exp);
            }
        }
    if let Some(ref kind) = var_spec.kind {
        check_blank_type(kind);
    }
}

/// Checks if any blanks exist on rhs of type declarations
fn check_blank_type_decl(spec: &TypeSpec){
    check_blank_type(&*spec.kind);
}

/// Checks a functions name, params and body for any invalid blank identifier usage
fn check_blank_func_decl(_name: &String,
                         params: &Vec<Field>,
                         return_kind: &Option<Box<AstKindNode>>,
                         body: &Vec<StatementNode>,
                         _line: u32){

    for field in params.iter(){
        check_blank_field(field);
    }

    match return_kind {
        &Some(ref return_kind) => check_blank_type(return_kind),
        &None => return
    }

    for stmt in body.iter(){
        traverse_stmt_for_invalid_blank(stmt)
    }
}

/// Checks a type for blank identifier usage
fn check_blank_type(kind: &AstKindNode) {
    match kind.ast_kind {
        AstKind::Identifier { ref name } => {
            if name == "_" {
                eprintln!("Error: line {}: Invalid type name. Cannot be blank identifier.", kind.line_number);
                exit(1);
            }
        }
        AstKind::Slice { ref base } => check_blank_type(&** base),
        AstKind::Array { ref base, .. } => check_blank_type(&** base),
        AstKind::Struct { ref fields } => {
            for field in fields {
                check_blank_field(field);
            }
        }
    }
}

/// Checks if any of the field have blank type
/// (In no situation should blank identifiers be rejected
fn check_blank_field(field: &Field){
    check_blank_type(&*field.kind)
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
        Statement::Assignment {ref rhs, ..} => {
            for exp in rhs.iter(){
                traverse_exp_for_invalid_blank(exp)
            }
        },
        Statement::OpAssignment { ref lhs, ref rhs, .. } => {
            traverse_exp_for_invalid_blank(&*lhs);
            traverse_exp_for_invalid_blank(&*rhs)
        },
        Statement::VarDeclarations { ref declarations } => {
            for decl in declarations.iter(){
                check_blank_var_decl(decl)
            }
        },
        Statement::ShortVariableDeclaration { ref expression_list, .. } => {
            for exp in expression_list.iter(){
                traverse_exp_for_invalid_blank(exp)
            }
        },
        Statement::IncDec { ref expr, .. } => {
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
            match condition {
                &Some(ref condition) => traverse_exp_for_invalid_blank(&*condition),
                &None => return,
            }
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
        }
        Statement::TypeDeclarations{ ref declarations } => {
            for spec in declarations {
                check_blank_type_decl(spec)
            }
        }
        Statement::Empty | Statement::Break | Statement::Continue => {}
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
        Expression::BinaryOperation { ref lhs, ref rhs, .. } => {
            traverse_exp_for_invalid_blank(&*lhs);
            traverse_exp_for_invalid_blank(&*rhs);

        },
        Expression::UnaryOperation { ref rhs, .. } => {
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
            traverse_exp_for_invalid_blank( &*lhs);
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





