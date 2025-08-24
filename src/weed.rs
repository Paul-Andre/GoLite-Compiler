use ast::*;
use std::process::exit;

/// Weeds the tree for incorrect break, continue, and blank identifier usage
pub fn weed_ast(root: &Program){
    if &root.package_name == "_" {
        eprintln!("Error: package name cannot be `_`");
        exit(1);
    }
    for node in root.declarations.iter() {
        match node.variant {
            TopLevelDeclarationVariant::FunctionDeclaration (Function { ref name, ref parameters, ref return_kind, ref body }) => {
                check_blank_func_decl(name, parameters, return_kind, body, node.line_number);
                for stmt in body.iter() {
                    check_for_correct_break_and_continue_usage(stmt, false);
                    traverse_stmt_for_invalid_blank(stmt);
                }

            },
            TopLevelDeclarationVariant::VarDeclarations { ref declarations } => {
                for decl in declarations.iter() {
                    check_blank_var_decl(&decl);
                }
            },
            _ => {},
        }
    }
}

pub fn weed_terminating_statements(root: &Program) {
    for node in root.declarations.iter() {
        match node.variant {
            TopLevelDeclarationVariant::FunctionDeclaration (Function { ref return_kind, ref body, .. }) => {
                match return_kind {
                    &Some(..) => check_correct_terminating_statements(body, node.line_number),
                    &None => {},
                }
            }
            _ => {},
        }
    }
}

fn check_correct_terminating_statements(body: &Vec<StatementNode>, line_number: u32){
    let length = body.len();
    match length {
        0 => error_missing_terminating_statement(line_number),
        _ => {
            match body[length-1].variant {
                StatementVariant::Return(..) => return,
                StatementVariant::Block(ref body) => check_correct_terminating_statements(body,line_number),
                StatementVariant::If { ref if_branch, ref else_branch, .. } => {
                    match else_branch {
                        &Some(ref else_branch) => {
                            check_correct_terminating_statements(if_branch, line_number);
                            check_correct_terminating_statements_elseif(else_branch, line_number);
                        }
                        &None => error_missing_terminating_statement(line_number),
                    }
                }
                StatementVariant::For { ref body, ref condition, .. } => {
                    match condition {
                        &None => {},
                        &Some(..) => error_missing_terminating_statement(line_number),
                    }
                    if find_break(body) {
                        error_missing_terminating_statement(line_number)
                    }
                }
                StatementVariant::Switch { ref body, .. } => {
                    let mut flag = false;
                    for case_clause in body {
                        match case_clause.switch_case {
                            SwitchCase::Default => flag = true,
                            _ => {},
                        }
                        if find_break(&case_clause.statements) {
                            error_missing_terminating_statement(line_number)
                        } 
                        check_correct_terminating_statements(&case_clause.statements, line_number);
                    }

                    if !flag {
                        error_missing_terminating_statement(line_number)
                    }
                }
                _ => error_missing_terminating_statement(line_number),
            }
        }
    }
}

fn find_break(body: &Vec<StatementNode>) -> bool {
    for stmt in body {
        match stmt.variant {
            StatementVariant::Break => return true,
            StatementVariant::Block(ref body) => {
                if find_break(body) {
                    return true;
                }
            }
            StatementVariant::If { ref if_branch, ref else_branch, .. } => {
                if find_break(if_branch) {
                    return true;
                }
                match else_branch {
                    &Some(ref else_branch) => {
                        if find_break_elseif(else_branch) {
                            return true;
                        }
                    }
                    &None => {},
                }
            }
            _ => {},
        }
    }
    return false;
}

fn find_break_elseif(stmt: &StatementNode) -> bool {
    match stmt.variant {
        StatementVariant::Block(ref body) => {
            return find_break(body)
        }
        StatementVariant::If { ref if_branch, ref else_branch, .. } => {
            if find_break(if_branch) {
                return true;
            }
            match else_branch {    
                &Some(ref else_branch) => {
                    if find_break_elseif(else_branch) {
                        return true;
                    }
                }
                &None => {},
            }
        }
        _ => return false,
    }
    return false;
}

fn error_missing_terminating_statement(line_number: u32) {
    print!("Error: line {}: missing terminating statement in function declaration", line_number);
    exit(1);
}

fn check_correct_terminating_statements_elseif(stmt: &StatementNode, line_number: u32) {
    match stmt.variant {
        StatementVariant::If { ref if_branch, ref else_branch, .. } => {
            match else_branch {
                &Some(ref else_branch) => {
                    check_correct_terminating_statements(if_branch, line_number);
                    check_correct_terminating_statements_elseif(else_branch, line_number);
                }
                &None => error_missing_terminating_statement(line_number),
            }
        }
        StatementVariant::Block(ref body) => {
            check_correct_terminating_statements(body, line_number);
        }
        _ => error_missing_terminating_statement(line_number),
    }
}

/*
BREAK/CONTINUE USAGE WEED FUNCTIONS
========================================= */

/// Checks for correct usage of break and continue
/// Note that `continue` is only valid for loops whereas `break` is balid for loops and switch
fn check_for_correct_break_and_continue_usage(stmt: &StatementNode, can_break: bool){
    match stmt.variant {
        StatementVariant::Block(ref v) => {
            for x in v {
                check_for_correct_break_and_continue_usage(&x, can_break);
            }
        },
        StatementVariant::If { ref if_branch, ref else_branch, .. } => {

            for x in if_branch {
                check_for_correct_break_and_continue_usage(&x, can_break);
            }

            match else_branch {
                &Some(ref else_branch) => check_for_correct_break_and_continue_usage(&*else_branch, can_break),
                &None => return,
            }
        },
        StatementVariant::Switch { ref body, .. } => {

            for case_clause in body {
                for stmt in &case_clause.statements {
                    check_for_correct_break_and_continue_usage(&stmt, true);
                }
            }
        },
        StatementVariant::Break => {
            if !can_break {
                eprintln!("Error: line {}: break outside loop or switch.", stmt.line_number);
                exit(1);
            }
        },
        StatementVariant::Continue => {
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
    match kind.variant {
        AstKindVariant::Identifier { ref name } => {
            if name == "_" {
                eprintln!("Error: line {}: Invalid type name. Cannot be blank identifier.", kind.line_number);
                exit(1);
            }
        }
        AstKindVariant::Slice { ref base } => check_blank_type(&** base),
        AstKindVariant::Array { ref base, .. } => check_blank_type(&** base),
        AstKindVariant::Struct { ref fields } => {
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
    match stmt.variant {
        StatementVariant::Block(ref v) => {
            for x in v {
                traverse_stmt_for_invalid_blank(&x)
            }
        },
        StatementVariant::Expression(ref exp) => {
            traverse_exp_for_invalid_blank(&*exp)
        },
        StatementVariant::Assignment {ref lhs, ref rhs, ..} => {
            for exp in lhs.iter(){
                traverse_assignable_exp_for_invalid_blank(exp)
            }
            for exp in rhs.iter(){
                traverse_exp_for_invalid_blank(exp)
            }
        },
        StatementVariant::OpAssignment { ref lhs, ref rhs, .. } => {
            traverse_exp_for_invalid_blank(&*lhs);
            traverse_exp_for_invalid_blank(&*rhs)
        },
        StatementVariant::VarDeclarations { ref declarations } => {
            for decl in declarations.iter(){
                check_blank_var_decl(decl)
            }
        },
        StatementVariant::ShortVariableDeclaration { ref expression_list, .. } => {
            for exp in expression_list.iter(){
                traverse_exp_for_invalid_blank(exp)
            }
        },
        StatementVariant::IncDec { ref expr, .. } => {
            traverse_exp_for_invalid_blank(&*expr)
        },
        StatementVariant::Print { ref exprs } => {
            for exp in exprs.iter(){
                traverse_exp_for_invalid_blank(exp)
            }
        },
        StatementVariant::Println { ref exprs } => {
            for exp in exprs.iter(){
                traverse_exp_for_invalid_blank(exp)
            }
        },
        StatementVariant::If { ref init, ref condition, ref if_branch, ref else_branch } => {
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
        StatementVariant::For { ref init, ref condition, ref post, ref body } => {
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
        StatementVariant::Switch { ref init, ref expr, ref body } => {
            traverse_stmt_for_invalid_blank(&*init);

            match expr {
                &Some(ref expr) => traverse_exp_for_invalid_blank(&*expr),
                &None => ()
            }

            for case_clause in body.iter() {
                traverse_case_clause_for_invalid_blank(case_clause)
            }
        },
        StatementVariant::Return( ref expr) => {
            match expr {
                &Some( ref expr ) => traverse_exp_for_invalid_blank(&*expr),
                &None => ()
            }
        }
        StatementVariant::TypeDeclarations{ ref declarations } => {
            for spec in declarations {
                check_blank_type_decl(spec)
            }
        }
        StatementVariant::Empty | StatementVariant::Break | StatementVariant::Continue => {}
    }
}


fn traverse_assignable_exp_for_invalid_blank(exp: &Expression) {
    match exp.variant {
        ExpressionVariant::Identifier { .. } => return,
        ExpressionVariant::RawLiteral { .. } => {
            eprintln!("Error: line {}: cannot assign to RawLiteral.", exp.line_number);
            exit(1);
        }
        ExpressionVariant::BinaryOperation { .. } => {
            eprintln!("Error: line {}: cannot assign to Binary expression.", exp.line_number);
            exit(1);
        }
        ExpressionVariant::UnaryOperation { .. } => {
            eprintln!("Error: line {}: cannot assign to Unary expression.", exp.line_number);
            exit(1);
        }
        ExpressionVariant::Index { .. } => {
            traverse_exp_for_invalid_blank(exp);
        }
        ExpressionVariant::Selector { .. } => {
            traverse_exp_for_invalid_blank(exp);
        }
        ExpressionVariant::FunctionCall { .. } => {
            eprintln!("Error: line {}: cannot assign to function call.", exp.line_number);
            exit(1);
        }
        ExpressionVariant::Append { .. } => {
            eprintln!("Error: line {}: cannot assign to append expression.", exp.line_number);
            exit(1);
        }
        ExpressionVariant::TypeCast { .. } => {
            eprintln!("Error: line {}: cannot assign to type cast.", exp.line_number);
            exit(1);
        }
    }
}


/// Recursively traverses expression in order to detect any invalid blank id usage
fn traverse_exp_for_invalid_blank(exp: &Expression){
    match exp.variant {
        ExpressionVariant::Identifier { ref name, .. } => {
            if name == "_" {
                eprintln!("Error: line {}: invalid use of blank identifier within expression.", exp.line_number);
                exit(1);
            }
        },
        ExpressionVariant::RawLiteral { ref value } => {
            if value == "_" {
                eprintln!("Error: line {}: invalid use of blank identifier within expression.", exp.line_number);
                exit(1);
            }
        },
        ExpressionVariant::BinaryOperation { ref lhs, ref rhs, .. } => {
            traverse_exp_for_invalid_blank(&*lhs);
            traverse_exp_for_invalid_blank(&*rhs);

        },
        ExpressionVariant::UnaryOperation { ref rhs, .. } => {
            traverse_exp_for_invalid_blank( &*rhs);
        }
        ExpressionVariant::Index { ref primary, ref index } => {
            traverse_exp_for_invalid_blank( &*primary);
            traverse_exp_for_invalid_blank( &*index);
        }
        ExpressionVariant::Selector { ref primary, ref name } => {
            if name == "_" {
                eprintln!("Error: line {}: invalid use of blank identifier within selector.", exp.line_number);
                exit(1);
            }

            traverse_exp_for_invalid_blank(&*primary)
        }
        ExpressionVariant::FunctionCall {ref primary, ref arguments } => {
            traverse_exp_for_invalid_blank(&*primary);

            for arg in arguments.iter() {
                traverse_exp_for_invalid_blank( &arg);
            }
        }
        ExpressionVariant::Append { ref lhs, ref rhs } => {
            traverse_exp_for_invalid_blank( &*lhs);
            traverse_exp_for_invalid_blank( &*rhs);
        }
        ExpressionVariant::TypeCast { ref expr, .. } => {
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





