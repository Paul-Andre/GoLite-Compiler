use ast::*;
use kind;
use kind::Kind;
use kind::BasicKind;
use symbol_table::*;
use std::process::exit;
use std::collections::HashMap;

pub fn typecheck(root: &Program) {
    // Because of how we defined the back pointers for the symbol table, the parent should be
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
                    if !kind::are_identical(id_kind, exp_kind) { // 3
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

pub fn typecheck_type_declarations(declarations: &[TypeSpec], symbol_table: &mut SymbolTable) {
    for spec in declarations {
        symbol_table.add_symbol(spec.name);
    }
}

pub fn typecheck_function_declaration(name: &String,
                                       params: &[Field],
                                       return_kind: &Option<Box<AstKindNode>>,
                                       body: &[StatementNode],
                                       line: u32,
                                       table: &mut SymbolTable) {
    panic!("unimplemented");
}

pub fn typecheck_statement(stmt: &StatementNode,
                           symbol_table: &mut SymbolTable) {
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
                (&Some( ref exp ), &Some(ref required_kind)) => {
                    let actual_kind = typecheck_expression(exp, symbol_table);
                    if actual_kind != required_kind {
                        eprintln!("Error: line {}: invalid return type {},
                        type used in function header is {}.",
                                 stmt.line_number, actual_kind, required_kind);
                        exit(1);
                    }
                },
                (&None, &None) => {},
                (&Some(_), &None) => {
                        eprintln!("Error: line {}: trying to return something from void function.",
                                 stmt.line_number);
                        exit(1);

                },
                (&None, &Some(ref k)) => {
                        eprintln!("Error: line {}: must return a value of type {}", stmt.line_number, k);
                        exit(1);
                },
            }

        },
        Statement::ShortVarDeclaration { ref identifier_list, ref expression_list } => {
            panic!("Unimplemented");
            /*
            let kinds = typecheck_expression_vec(expression_list, symbol_table); // 1
            let mut count = 0;

            for it in identifier_list.iter().zip(kinds.iter()) {
                let (id, exp_kind) = it;
                let id_kind = symbol_table.get_type(id);
                match id_kind {
                    &Some(id_kind) => {
                        if !kind::are_identical(id_kind, exp_kind) { // 3
                            println!("Error: line {}: invalid type of expression assigned to {}.", 
                                     stmt.line_number,
                                     id);
                            exit(1);
                        }
                    }
                    &None => {
                        count = count + 1; // 2
                        symbol_table.add_variable(id, exp_kind);
                    }
                }
            }

            if count == 0 {
                println!("Error: line {}: All variables on the lhs of the assignment are already declared.", stmt.line_number);
                exit(1);
            }
            */
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
                if !kind::are_identical(lhs_kind, rhs_kind) {
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

            match get_kind_binary_op(lhs_kind, rhs_kind, operator) {
                &Some(assigned_kind) => {
                    if !kind::are_identical(lhs_kind, assigned_kind) {
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
            let new_scope = symbol_table.new_scope();
            for stmt in statements {
                typecheck_statements(stmt, &mut new_scope);
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

        Statement::For { ref init, ref condition, ref post, ref body } => {
            let init_scope = symbol_table.new_scope();

            // init, condition and post are in the same scope; body is in a different one
            typecheck_statement(init, init_scope);
            let exp_type = typecheck_expression(condition, init_scope);
            typecheck_statement(post, init_scope);
            
            if !kind::are_identical(exp_type, Kind::BasicKind(kind::BasicKind::Bool)) {
                println!("Error: line {}: condition must be of type bool.", 
                         stmt.line_number);
                exit(1);
            }

            let new_scope = init_scope.new_scope();
            for stmt in body {
                typecheck_statements(stmt, new_scope);
            }
        }

        Statement::If { ref init, ref condition, ref if_branch, ref else_branch } => {
            {
                let init_scope = symbol_table.new_scope();
                typecheck_statement(init, init_scope);
                let exp_type = typecheck_expression(condition, init_scope);

                // how the hell do I compare a Kind with bool?
                if !kind::are_identical(exp_type, Kind::BasicKind(kind::BasicKind::Bool)) {
                    println!("Error: line {}: condition must be of type bool.", 
                             stmt.line_number);
                    exit(1);
                }

                let new_scope = init_scope.new_scope();
                for stmt in if_branch {
                    typecheck_statements(stmt, new_scope);
                }
            }

            match else_branch {
                &Some(stmt) => typecheck_statement(stmt, symbol_table),
                &None => {},
            }
        }

        Statement::Switch { ref init, ref expr, ref body } => {
            let init_scope = symbol_table.new_scope();
            typecheck_statement(init, init_scope);
            let exp_type = typecheck_expression(expr, init_scope);

            for cc in body {
                match cc.switch_case {
                    SwitchCase::Cases(exp) => {
                        let cc_type = typecheck_expression(exp, init_scope);
                        if !kind::are_identical(cc_type, exp_type) {
                            println!("Error: line {}: mismatched case type.", 
                                     cc.line_number);
                            exit(1);
                        }
                    }
                    SwitchCase::Default => {},
                }

                for stmt in cc.statements {
                    let new_scope = init_scope.new_scope();
                    typecheck_statement(stmt, &mut new_scope);
                }
            }

        }
        Statement::IncDec { ref is_dec, ref expr } => {
            let exp_type = typecheck_expression(expr, symbol_table);

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
                            return kind.clone()
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
    return Kind::Undefined;
}

pub fn typecheck_expression_vec(exprs: &[ExpressionNode],
                                 symbol_table: &mut SymbolTable) -> Vec<Kind> {
    exprs.iter().map(|ref e| {
        typecheck_expression(e)
    }).collect::<Vec<_>>()

}

// Checks if a type is addressable
// Question: isn't it an expression that is addressable or not?
pub fn is_addressable(kind: Kind) -> bool {
    true

}

// Need also to check if kinds are valid for op
pub fn get_kind_binary_op(a: &Kind, b: &Kind, op: BinaryOperator) -> Option<Kind> {
    Some(Kind::Undefined)
}

pub fn get_kind_unary_op(a: Kind, op: UnaryOperator) -> Option<Kind> {
    Some(Kind::Undefined)
}
