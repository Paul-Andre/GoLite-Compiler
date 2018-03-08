use ast::*;
use symbol_table::*;
use std::process::exit;
use std::collections::HashMap;

pub fn typecheck(root: &Program) {
    let universe_block = create_root_symbol_table(root);
    let mut symbol_table = universe_block.new_scope();

    for decl in &root.declarations {
        typecheck_top_level_declaration(decl, symbol_table);
    }
}

pub fn typecheck_top_level_declaration(decl: &TopLevelDeclarationNode, symbol_table: &mut SymbolTable) {
    match decl.top_level_declaration {
        TopLevelDeclaration::VarDeclarations { ref declarations } => {
            typecheck_variable_declarations(declarations, &symbol_table);
        }
        TopLevelDeclaration::KindDeclarations { ref declarations } => {
            typecheck_type_declarations(declarations, &symbol_table);
        }
        TopLevelDeclaration::FunctionDeclaration { ref name, ref parameters, ref return_kind, ref body } => {
            typecheck_function_declaration(name, parameters, return_kind, body, decl.line_number, symbol_table);

        }
    }
}

pub fn typecheck_variable_declarations(declarations: &[VarSpec], symbol_table: &mut SymbolTable) {
    panic!("unimplemented");
        /*
    for spec in declarations {
        let kinds = typecheck_expression_vec(spec.rhs, symbol_table); // 1

        match spec.kind {
            &Some(assigned_type) => {
                // TODO: see if types are identical here
                for id in spec.names {
                    symbol_table.add_symbol(id, assigned_type);
                }
            }
            &None => {},
        }

        for (id, exp_kind) in spec.names.iter().zip(kinds.iter()) {
            // Ok, wtf is happening here
            let id_kind = symbol_table.get_symbol(&id);
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
    */
}

pub fn typecheck_type_declarations(declarations: &[KindSpec], symbol_table: &mut SymbolTable) {
    for spec in declarations {
        symbol_table.add_symbol(spec.name);
    }
}

pub fn typecheck_function_declaration(name: &String,
                                       params: &[Field],
                                       return_kind: &Option<Box<AstKindNode>>,
                                       body: &[StatementNode],
                                       line: &int,
                                       table: &mut SymbolTable) {
    panic!("unimplemented");
}

pub fn typecheck_statement(stmt: &StatementNode,
                           symbol_table: &mut SymbolTable,
                           references: &mut Vec<kind::Definitions>) {
    match stmt.statement {
        Statement::Empty => {},
        Statement::Break => {},
        Statement::Continue => {},
        Statement::Expression(exp) => {
            typecheck_expression(exp);
        }

        Statement::Return(exp) => {
            // We know that return statements only happen inside functions
            match (exp, symbol_table.return_type)  {
                (&Some( ref exp ), &Some(ref required_kind) => {
                    actual_kind = typecheck_expression(exp, symbol_table);
                    if actual_kind != required_kind {
                        println!("Error: line {}: invalid return type.", stmt.line_number);
                        exit(1);
                    }
                }
                &None => {
                    match symbol_table.return_type {
                        &Some { ref t } => {
                            match t {
                                Void => {},
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
                let id_kind = symbol_table.get_type(id);
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
                          stmt.line_number);
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
            
            // how the hell do I compare a Kind with bool?
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
            
            // how the hell do I compare a Kind with bool?
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
            
            // how the hell do I compare a Kind with bool?
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
                &None => {},
            }
        }

        Statement::Switch { ref init, ref expr, ref body } => { // DO WE START A NEW SCOPE?
            let init_scope = make_new_symbol_table(symbol_table.return_type, symbol_table);
            typecheck_statement(init, init_scope);
            let exp_type = typecheck_expression(expr, init_scope);

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
                    SwitchCase::Default => {},
                }

                for stmt in cc.statements {
                    let new_scope = make_new_symbol_table(init_scope.return_type, init_scope);
                    typecheck_statement(stmt, new_scope);
                }
            }

        }
        Statement::IncDec { ref is_dec, ref expr } => {
            let exp_type = typecheck_expression(expr);

            // Resolve base type or something
        }
    }
}

pub fn typecheck_expression(exp: &mut ExpressionNode, symbol_table: &mut SymbolTable) -> Kind {
    match exp.expression {
        Expression::RawLiteral => {
            return exp.kind
        }
        Expression::Identifier { ref name } => {
            let symbol = symbol_table.get_symbol(name);
            match symbol {
                &Some(symbol) => {
                    match symbol.declaration {
                        Declaration::Variable(ref kind) => {
                            exp.kind = kind.clone();
                            kind.clone()
                        }
                        Declaration::Type(ref kind) => {
                            // error
                        }
                    }
                }
                &None => {},// error ,
            }
        }
        Expression::UnaryOperation { ref op, ref rhs } => {
        }
        Expression::BinaryOperation { ref op, ref lhs, ref rhs } => {

        }
        Expression::FunctionCall { ref primary, ref arguments } => {

        }
        Expression::Index { ref primary, ref index } => {

        }
        Expression::Selector { ref primary, ref name } => {

        }
        Expression::Append { ref lhs, ref rhs } => {

        }
        Expression::TypeCast { ref expr } => {

        }
    }
}

pub fn typecheck_expression_vec(exprs: &Vec<ExpressionNode>, 
                                 symbol_table: &mut SymbolTable) -> Vec<Kind> {
    for exp in exprs.iter() {
        // Do something
    }
}

// Checks if a type is addressable
pub fn is_adressable(kind: Kind) -> bool {

}

// Need also to check if kinds are valid for op
pub fn get_kind_binary_op(a: &Kind, b: &Kind, op: BinaryOperator) -> Option<Kind> {

}

pub fn get_kind_unary_op(a: Kind, op: UnaryOperator) -> Option<Kind> {

}
