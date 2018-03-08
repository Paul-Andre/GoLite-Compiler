use ast::*;
use ast::Field;
use kind;
use kind::*;
use kind::Kind;
use kind::BasicKind;
use symbol_table::*;
use std::process::exit;
use std::collections::HashMap;

pub fn typecheck(root: &mut Program, print_table: bool) {
    // Because of how we defined the back pointers for the symbol table, the parent should be
    let universe_block = create_root_symbol_table(print_table);
    let symbol_table = &mut universe_block.new_scope();

    for decl in &mut root.declarations {
        typecheck_top_level_declaration(decl, symbol_table);
    }
}

pub fn typecheck_top_level_declaration(decl: &mut TopLevelDeclarationNode, symbol_table: &mut SymbolTable) {
    match decl.top_level_declaration {
        TopLevelDeclaration::VarDeclarations { ref mut declarations } => {
            typecheck_variable_declarations(declarations, symbol_table);
        }
        TopLevelDeclaration::TypeDeclarations { ref mut declarations } => {
            typecheck_type_declarations(declarations, symbol_table);
        }
        TopLevelDeclaration::FunctionDeclaration { ref name, ref mut parameters, ref mut return_kind, ref mut body } => {
            typecheck_function_declaration(name, parameters, return_kind, body, decl.line_number, symbol_table);
        }
    }
}


pub fn typecheck_variable_declarations(declarations: &mut [VarSpec], symbol_table: &mut SymbolTable) {
    panic!("unimplemented");

    /*
    for spec in declarations {

        let kinds = typecheck_expression_vec(&mut spec.rhs, symbol_table); // 1

        match spec.kind {
            &Some(assigned_type) => {
                for id in spec.names {
                    symbol_table.add_symbol(id, assigned_type);
                }
            }
            &None => {},
        }

        for (id, exp_kind) in spec.names.iter().zip(kinds.iter()) {
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

pub fn typecheck_type_declarations(declarations: &mut [TypeSpec], symbol_table: &mut SymbolTable) {

    for spec in declarations {
        let kind = typecheck_kind(&mut spec.kind, symbol_table, Some(&spec.name));
        symbol_table.add_declaration(spec.name.clone(),
                                     spec.line_number,
                                     Declaration::Type(kind),
                                     /*inferred*/ false);
    }

}

pub fn typecheck_statements(statements: &mut [StatementNode], symbol_table: &mut SymbolTable) {
    for s in statements {
        //symbol_table.add_symbol(spec.name);
    }
}

pub fn typecheck_function_declaration(name: &String,
                                       params: &[Field],
                                       return_kind: &Option<Box<AstKindNode>>,
                                       body: &mut [StatementNode],
                                       line: u32,
                                       table: &mut SymbolTable) {
    panic!("unimplemented");
}

pub fn typecheck_statement(stmt: &mut StatementNode,
                           symbol_table: &mut SymbolTable) {
    match stmt.statement {
        Statement::Empty => panic!("unimplemented"),
        Statement::Break => panic!("unimplemented"),
        Statement::Continue => panic!("unimplemented"),
        Statement::Expression(ref mut exp) => {
            typecheck_expression(exp, symbol_table);
        }
        Statement::Return(ref mut exp) => {
            // We know that return statements only happen inside functions
            let maybe_actual_kind =
                if let &mut Some(ref mut exp) = exp {
                    Some(typecheck_expression(&mut **exp, symbol_table))
                } else {
                    None
                };

            match (maybe_actual_kind, &symbol_table.return_type)  {
                (Some( ref actual_kind ), &Some(ref required_kind)) => {
                    if !are_identical(actual_kind, required_kind) {
                        eprintln!("Error: line {}: invalid return type {}. \
                            Type used in function header is {}.",
                                 stmt.line_number, actual_kind, required_kind);
                        exit(1);
                    }
                },
                (None, &None) => {},
                (Some(_), &None) => {
                        eprintln!("Error: line {}: trying to return something from void function.",
                                 stmt.line_number);
                        exit(1);

                },
                (None, &Some(ref k)) => {
                        eprintln!("Error: line {}: must return a value of type {}", stmt.line_number, k);
                        exit(1);
                },
            }

        },
        Statement::ShortVariableDeclaration { ref identifier_list, ref expression_list } => {
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

        Statement::VarDeclarations { ref mut declarations } => {
            typecheck_variable_declarations(declarations, symbol_table);
        }

        Statement::TypeDeclarations { ref mut declarations } => {
            typecheck_type_declarations(declarations, symbol_table);
        }

        Statement::Assignment { ref mut lhs, ref mut rhs } => {
            panic!("unimplemented");

            for i in 0..lhs.len() {
                let lhs_exp = &mut lhs[i];
                let rhs_exp = &mut rhs[i];
                if !is_addressable(lhs_exp) {
                     println!("Error: line {}: lvalue {} in list is not addressable.", 
                              stmt.line_number,
                              i + 1);
                     exit(1);
                }
                let lhs_kind = typecheck_expression(lhs_exp, symbol_table);
                let rhs_kind = typecheck_expression(rhs_exp, symbol_table);

                if !are_identical(&lhs_kind, &rhs_kind) {
                    println!("Error: line {}: In position {} of assignment list, \
                    trying to assign a value of type {} \
                    to an expression expression {}", 
                    stmt.line_number,
                    i + 1,
                    rhs_kind,
                    lhs_kind);
                    exit(1);
                }

            }
        }

        Statement::OpAssignment { ref mut lhs, ref mut rhs, ref mut operator } => {
            panic!("unimplemented");
            /*
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
            */
        }

        Statement::Block(ref mut statements) => {
            let new_scope = &mut symbol_table.new_scope();
            typecheck_statements(statements, new_scope);
        }
        Statement::Print { ref mut exprs } |
        Statement::Println { ref mut exprs } => {
            for expr in exprs {
                let kind = typecheck_expression(expr, symbol_table);
            }
        }

        Statement::For { ref mut init, ref mut condition, ref mut post, ref mut body } => {
            let init_scope = &mut symbol_table.new_scope();

            // init, condition and post are in the same scope; body is in a different one
            typecheck_statement(init, init_scope);
            let exp_type = 
                if let Some(ref mut condition) = *condition {
                    typecheck_expression(condition, init_scope)
                } else {
                    Kind::Basic(BasicKind::Bool)
                };

            typecheck_statement(post, init_scope);
            
            if !are_identical(&exp_type, &Kind::Basic(BasicKind::Bool)) {
                println!("Error: line {}: condition must be of type bool.", 
                         stmt.line_number);
                exit(1);
            }

            let new_scope = &mut init_scope.new_scope();
            typecheck_statements(body, new_scope);
        }

        Statement::If { ref mut init, ref mut condition, ref mut if_branch, ref mut else_branch } => {
            let init_scope = &mut symbol_table.new_scope();
            typecheck_statement(init, init_scope);
            let exp_type = typecheck_expression(condition, init_scope);

            if !are_identical(&exp_type, &Kind::Basic(BasicKind::Bool)) {
                println!("Error: line {}: condition must be of type bool.", 
                         stmt.line_number);
                exit(1);
            }

            {
                let new_scope = &mut init_scope.new_scope();
                typecheck_statements(if_branch, new_scope);
            }

            match *else_branch {
                Some(ref mut stmt) => typecheck_statement(stmt, init_scope),
                None => {},
            }
        }

        Statement::Switch { ref mut init, ref mut expr, ref mut body } => {
            let init_scope = &mut symbol_table.new_scope();
            typecheck_statement(init, init_scope);
            let exp_type = 
                if let Some(ref mut expr) = *expr {
                    typecheck_expression(expr, init_scope)
                } else {
                    Kind::Basic(BasicKind::Bool)
                };

            for cc in body {
                
                match cc.switch_case {
                    SwitchCase::Cases(ref mut cases) => {
                        for case in cases {
                            let cc_type = typecheck_expression(case, init_scope);
                            if !are_identical(&cc_type, &exp_type) {
                                // TODO: also must be comparabel I believe
                                eprintln!("Error: line {}: mismatched case type {}; \
                                         expected {}.", 
                                         cc.line_number, cc_type, exp_type);
                                exit(1);
                            }
                        }
                    }
                    SwitchCase::Default => {},
                }
                

                for mut stmt in &mut cc.statements {
                    let new_scope = &mut init_scope.new_scope();
                    typecheck_statement(&mut stmt, new_scope);
                }
            }

        }
        Statement::IncDec { ref is_dec, ref mut expr } => {
            // TODO: check if is addressable
            let exp_type = typecheck_expression(expr, symbol_table);
            let base = exp_type.resolve();

            // Resolve base type or something
            panic!("unimplemented");
        }
    }
}

pub fn typecheck_kind(ast: &mut AstKindNode, 
                      symbol_table: &mut SymbolTable, 
                      top_name: Option<&str>) -> Kind { 
                    // top_name is to prevent recursive definitions in structs
    match ast.ast_kind {
        AstKind::Identifier { ref name } => {
            match top_name {
                Some(ref top_name) => {
                    if name == top_name {
                        //error recursive def
                    }
                }
                None => {},
            }
            if let Declaration::Type(ref kind) = symbol_table.get_symbol(name, ast.line_number).declaration {
                return kind.clone();
            } else {
                //error
                exit(1);
            }
        },
        AstKind::Slice { ref mut base } => {
            return Kind::Slice(Box::new(typecheck_kind(base, symbol_table, top_name)))
        },
        AstKind::Array { ref mut base, ref size } => {
            return Kind::Array(Box::new(typecheck_kind(base, symbol_table, top_name)), 0) // CHANGE
        },
        AstKind::Struct { ref mut fields } => {
            let mut kind_fields = Vec::new();  
            for field in fields {
                let field_kind = typecheck_kind(&mut field.kind, symbol_table, top_name);
                for id in &field.identifiers {
                    kind_fields.push(kind::Field{name: id.clone(), kind: field_kind.clone()});
                }
            }
            return Kind::Struct(kind_fields)
        }
    }
}


pub fn typecheck_expression(exp: &mut ExpressionNode, symbol_table: &mut SymbolTable) -> Kind {
    match exp.expression {
        Expression::RawLiteral{..} => {
            return exp.kind.clone()
        }

        Expression::Identifier { ref name } => {
            let symbol = symbol_table.get_symbol(name, exp.line_number);
            match symbol.declaration {
                Declaration::Variable(ref kind) | Declaration::Constant(ref kind) => {
                    exp.kind = kind.clone();
                    return kind.clone()
                }
                _ => {
                    eprintln!("Error: line {}: `{}` is not a variable or a constant.", 
                              exp.line_number, name);
                    exit(1);
                }
            }
        }

        Expression::UnaryOperation { ref op, ref mut rhs } => {
            let kind = typecheck_expression(rhs, symbol_table);
            let op_kind = get_kind_unary_op(&kind, op.clone(), exp.line_number);
            exp.kind = op_kind;
            return exp.kind.clone()
        }

        Expression::BinaryOperation { ref op, ref mut lhs, ref mut rhs } => {
            let lhs_kind = typecheck_expression(lhs, symbol_table);
            let rhs_kind = typecheck_expression(rhs, symbol_table);
            let op_kind = get_kind_binary_op(&lhs_kind, &rhs_kind, op.clone(), exp.line_number);
            exp.kind = op_kind;
            return exp.kind.clone()
        }

        Expression::FunctionCall { ref mut primary, ref arguments } => {
            if let Expression::Identifier{ref name} = primary.expression {
                let symbol = symbol_table.get_symbol(name, exp.line_number);
                match symbol.declaration {
                    Declaration::Type(ref kind) => {
                        //asdfasdfas
                    },
                    Declaration::Function{..} => {
                        //asdfasdfas
                    },
                    _ => {
                        eprintln!("Error: line {}: `{}` is not a type of function.",
                                  exp.line_number, name);
                    }
                }
            } else {
                eprintln!("Error: line {}: primary epression for function call or \
                type cast must be an identifier.", exp.line_number);
            }
        }

        Expression::Index { ref mut primary, ref mut index } => {
            let primary_kind = typecheck_expression(primary, symbol_table);
            let index_kind = typecheck_expression(index, symbol_table);
            match primary_kind.resolve() {
                &Kind::Array(ref a_kind, ..) | &Kind::Slice(ref a_kind) => {
                    if let &Kind::Basic(ref kind)=index_kind.resolve()  {
                        if let &BasicKind::Int = kind {
                            exp.kind = *a_kind.clone();
                            return *a_kind.clone()
                        } else {
                            eprintln!("Error: line {}: index expression does not resolve \
                            to int", exp.line_number);
                        }
                    } else {
                        eprintln!("Error: line {}: index expression does not resolve to \
                        Basic type", exp.line_number);
                    }
                }
                _ => eprintln!("Error: line {}: primary expression does not resolve to \
                     Slice or Array type", exp.line_number),
            }
        }

        Expression::Selector { ref mut primary, ref name } => {
            let kind = typecheck_expression(primary, symbol_table);
            if let &Kind::Struct(ref fields) = kind.resolve() {
                for field in fields {
                    if field.name == *name {
                        return field.kind.clone()
                    }
                }
                eprintln!("Error: line {}: unknown field \"{}\"", exp.line_number, name);
            }
            eprintln!("Error: line {}: primary expression does not resolve to \
                      Struct type", exp.line_number);
        }

        Expression::Append { ref mut lhs, ref mut rhs } => {
            let s_kind = typecheck_expression(lhs, symbol_table);
            let kind = typecheck_expression(rhs, symbol_table);

            if let &Kind::Slice(ref t_kind) = s_kind.resolve() {
                if are_identical(t_kind, &kind) {
                    exp.kind = s_kind.clone();
                    return s_kind.clone()
                } else {
                    eprintln!("Error: line {}: mismatched types in \
                    append expression", exp.line_number);
                }
            } else {
                eprintln!("Error: line {}: lhs does not resolve to Slice \
                in append expression", exp.line_number);
            }
        }

        Expression::TypeCast { ref expr } => {
            // We need to remove this
        }
    } 
    return Kind::Undefined;
}

// Checks if a type is addressable
// Question: isn't it an expression that is addressable or not?
pub fn is_addressable(exp: &ExpressionNode) -> bool {
    true
}

// Need also to check if kinds are valid for op
pub fn get_kind_binary_op(a: &Kind, b: &Kind, op: BinaryOperator, line_number: u32) -> Kind {
    Kind::Undefined
}

pub fn get_kind_unary_op(a: &Kind, op: UnaryOperator, line_number: u32) -> Kind {
    Kind::Undefined
}
