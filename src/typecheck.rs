use ast::*;
use symbol_table::*;
use std::process::exit;
use std::collections::HashMap;

pub fn typecheck(root: &Program) {
    let symbol_table = construct_program_symbol_table(root);

    for decl in &root.declarations {
        typecheck_top_level_declaration(decl, &mut *root_scope.SymbolTable);
    }
}

pub fn typecheck_top_level_declaration(decl: &TopLevelDeclarationNode, symbol_table: &SymbolTable) {
    match decl.top_level_declaration {
        TopLevelDeclaration::VarDeclarations { ref declarations } => {
            typecheck_variable_declarations(declarations, &symbol_table);
        }
        TopLevelDeclaration::TypeDeclarations { ref declarations } => {
            typecheck_type_declarations(declarations, &symbol_table);
        }
        TopLevelDeclaration::FunctionDeclaration { ref name, ref parameters, ref return_kind, ref body } => {
            typecheck_function_declaration(name, params, return_kind, body, line, table);

        }
    }
}

pub fn typecheck_variable_declarations(declarations: &Vec<VarSpec>, symbol_table: &SymbolTable) {
    for spec in declarations {
        let kinds = typecheck_expression_vec(spec.rhs, symbol_table); // 1

        match spec.kind {
            &Some(assigned_type) => {
                for id in spec.names {
                    add_symbol(id, assigned_type, symbol_table);
                }
            }
            &None => return;
        }

        for it in identifier_list.iter().zip(kinds.iter()) {
            let (id, exp_kind) = it;
            let id_kind = get_type(id, symbol_table);
            match id_kind {
                &Some(id_kind) => {
                    if !type_are_equal(id_kind, exp_kind) { // 3
                        println!("Error: line {}: invalid type of expression assigned to {}.", 
                                 stmt.line_number,
                                 id);
                        exit(1);
                    }
                }
                &None => {
                    add_symbol(id, exp_kind, symbol_table);
                }
            }
        }
    }
}

pub fn typecheck_type_declarations(declarations: &Vec<TypeSpec>, symbol_table: &SymbolTable) {
    for spec in declarations {
        add_symbol(spec.name, );
    }
}

pub fn typecheck_function_declaration(name: &String,
                                       params: &Vec<Field>,
                                       return_kind: &Option<Box<AstKindNode>>,
                                       body: &Vec<StatementNode>,
                                       line: &int,
                                       table: &SymbolTable) {

}

pub fn typecheck_statement(stmt: &StatementNode, symbol_table: &SymbolTable) {
    match stmt.statement {
        Statement::Empty => return;
        Statement::Break => return;
        Statement::Continue => return;
        Statement::Expression(exp) => {
            typecheck_expression(exp);
        }

        Statement::Return(exp) => {
            let mut return_type;
            match exp {
                &Some { ref exp } => {
                    return_type = typecheck_expression(exp, symbol_table);
                    if return_type != symbol_table.return_type {
                        println!("Error: line {}: invalid return type.", stmt.line_number);
                        exit(1);
                    }
                }
                &None => {
                    match symbol_table.return_type {
                        &Some { ref t } => {
                            match t {
                                Void => return;
                                _ => {
                                    println!("Error: line {}: invalid return type.", stmt.line_number);
                                    exit(1);
                                }
                            }
                        }
                        &None => {
                            println!("Error: line {}: returning when not in a function call.", stmt.line_number);
                            exit(1);
                        }
                    }
                }
            }

        }

        Statement::ShortVarDeclaration { ref identifier_list, ref expression_list } => {
            let kinds = typecheck_expression_vec(expression_list, symbol_table); // 1
            let mut count = 0;

            for it in identifier_list.iter().zip(kinds.iter()) {
                let (id, exp_kind) = it;
                let id_kind = get_type(id, symbol_table);
                match id_kind {
                    &Some(id_kind) => {
                        if !type_are_equal(id_kind, exp_kind) { // 3
                            println!("Error: line {}: invalid type of expression assigned to {}.", 
                                     stmt.line_number,
                                     id);
                            exit(1);
                        }
                    }
                    &None => {
                        count = count + 1; // 2
                        add_symbol(id, exp_kind, symbol_table);
                    }
                }
            }

            if count == 0 {
                println!("Error: line {}: All variables on the lhs of the assignment are already declared.", stmt.line_number);
                exit(1);
            }
        }

        Statement::VarDeclaration { ref declarations } => {
            typecheck_variable_declarations(declarations, symbol_table);
        }

        Statement::Assignment { ref lhs, ref rhs } => {
            let lhs_kinds = typecheck_expression_vec(lhs, symbol_table);
            let rhs_kinds = typecheck_expression_vec(rhs, symbol_table);
            let mut count = 0;

            for it in lhs_kinds.iter().zip(rhs_kinds.iter()) {
                let (lhs_kind, rhs_kind) = it;
                if !is_addressable(lhs_kind) {
                     println!("Error: line {}: unadressable lvalue {} in list.", 
                              stmt.line_number,
                              count);
                     exit(1);
                }
                if !type_are_equal(lhs_kind, rhs_kind) {
                     println!("Error: line {}: invalid type of expression {} in list.", 
                              stmt.line_number,
                              count);
                     exit(1);
                }
                count = count + 1;
            }
        }

        Statement::OpAssignment { ref lhs, ref rhs, ref operator } => {
            let lhs_kind = typecheck_expression(lhs, symbol_table);
            let rhs_kind = typecheck_expression(rhs, symbol_table);

            if !is_addressable(lhs_kind) {
                 println!("Error: line {}: unadressable lvalue.", 
                          stmt.line_number,
                          count);
                 exit(1);
            }

            match get_type_binary_op(lhs_kind, rhs_kind, operator) {
                &Some(assinged_kind) => {
                    if !type_are_equal(lhs_kind, assigned_kind) {
                        println!("Error: line {}: invalid assignment type.", 
                                 stmt.line_number);
                        exit(1);
                    }
                },
                &None => {
                    println!("Error: line {}: invalid operand types.", 
                             stmt.line_number);
                    exit(1);
                },
            }
        }

        Statement::Block(statements) => {
            let new_scope = make_new_symbol_table(symbol_table.return_type, symbol_table);
            for stmt in statements {
                typecheck_statements(stmt, new_scope);
            }
        }

        Statement::Print { ref exprs } => {
            let exp_kinds = typecheck_expression_vec(exprs, symbol_table);
            for kind in exp_kinds {
                // Resolve type somehow
            }
        }
        Statement::Prinln { ref exprs } => {
            let exp_kinds = typecheck_expression_vec(exprs, symbol_table);
            for kind in exp_kinds {
                // Resolve type somehow
            }

        }

        Statement::Loop { ref body } => {
            let new_scope = make_new_symbol_table(symbol_table.return_type, symbol_table);
            for stmt in body {
                typecheck_statements(stmt, new_scope);
            }
        }

        Statement::While { ref condition, ref body } => {
            let new_scope = make_new_symbol_table(symbol_table.return_type, symbol_table);
            let exp_type = typecheck_expression(condition, symbol_table);
            
            // how the hell do I compare a Type with bool?
            if !type_are_equal(exp_type, fix_me) {
                println!("Error: line {}: condition must be of type bool.", 
                         stmt.line_number);
                exit(1);
            }

            for stmt in body {
                typecheck_statements(stmt, new_scope);
            }

        }

        Statement::For { ref init, ref condition, ref post, ref body } => {
            let init_scope = make_new_symbol_table(symbol_table.return_type, symbol_table);

            // init, condition and post are in the same scope; body is in a different one
            typecheck_statement(init, init_scope);
            let exp_type = typecheck_expression(condition, init_scope);
            typecheck_statement(post, init_scope);
            
            // how the hell do I compare a Type with bool?
            if !type_are_equal(exp_type, fix_me) {
                println!("Error: line {}: condition must be of type bool.", 
                         stmt.line_number);
                exit(1);
            }

            let new_scope = make_new_symbol_table(init_scope.return_type, init_scope);
            for stmt in body {
                typecheck_statements(stmt, new_scope);
            }
        }

        Statement::If { ref init, ref condition, ref if_branch, ref else_branch } => {
            let init_scope = make_new_symbol_table(symbol_table.return_type, symbol_table);
            typecheck_statement(init, init_scope);
            let exp_type = typecheck_expression(condition, init_scope);
            
            // how the hell do I compare a Type with bool?
            if !type_are_equal(exp_type, fix_me) {
                println!("Error: line {}: condition must be of type bool.", 
                         stmt.line_number);
                exit(1);
            }

            let new_scope = make_new_symbol_table(init_scope.return_type, init_scope);
            for stmt in if_branch {
                typecheck_statements(stmt, new_scope);
            }

            match else_branch {
                &Some(stmt) => typecheck_statement(stmt, init_scope),
                &None => return;
            }
        }

        Statement::Switch { ref init, ref expr, ref body } => { // DO WE START A NEW SCOPE?
            let init_scope = make_new_symbol_table(symbol_table.return_type, symbol_table);
            typecheck_statement(init, init_scope);
            let exp_type = typecheck_expression(expr, init_scope);

            let new_scope = make_new_symbol_table(init_scope.return_type, init_scope);
            for cc in body {
                match cc.switch_case {
                    SwitchCase::Cases(exp) => {
                        let cc_type = typecheck_expression(exp, init_scope);
                        if !type_are_equal(cc_type, exp_type) {
                            println!("Error: line {}: mismatched case type.", 
                                     cc.line_number);
                            exit(1);
                        }
                    }
                    SwitchCase::Default => return;
                }

                for stmt in cc.statements {
                    typecheck_statement(stmt, 
                }
            }

        }
        Statement::IncDec { ref is_dec, ref expr } => {
            let exp_type = typecheck_expression(expr);

            // Resolve base type or something
        }
    }
}

pub fn typecheck_expression(exp: &ExpressionNode, symbol_table: &SymbolTable) -> Type {
    match exp.expression {
        Expression::RawLiteral => {
            // Resolve base type
        }
        Expression::Identifier => {
            
        }
        Expression::UnaryOperation => {

        }
        Expression::BinaryOperation => {

        }
        Expression::FunctionCall => {

        }
        Expression::Index => {

        }
        Expression::Selector => {

        }
        Expression::Append => {

        }
        Expression::TypeCast => {

        }
    }
}

pub fn typecheck_expression_vec(exprs: &Vec<ExpressionNode>, 
                                 symbol_table: &SymbolTable) -> Vec<Type> {
    for exp in exprs.iter() {
        // Do something
    }
}

// Checks if a type is addressable
pub fn is_adressable(kind: Type) -> bool {

}

pub fn get_type_binary_op(a: Type, b: Type, op: BinaryOperator) -> Option<Type> {

}

pub fn get_type_unary_op(a: Type, op: UnaryOperator) -> Option<Type> {

}
