use ast::*;
use std::mem;
use ast::Field;
use kind;
use kind::*;
use kind::Kind;
use kind::BasicKind;
use symbol_table::*;
use std::process::exit;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn typecheck(root: &mut Program, print_table: bool) {
    // Because of how we defined the back pointers for the symbol table, the parent should be
    let universe_block = create_root_symbol_table(print_table);
    let symbol_table = &mut universe_block.new_scope();

    for decl in &mut root.declarations {
        typecheck_top_level_declaration(decl, symbol_table);
    }
}

fn typecheck_top_level_declaration(decl: &mut TopLevelDeclarationNode, symbol_table: &mut SymbolTable) {
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


fn typecheck_variable_declarations(declarations: &mut [VarSpec], symbol_table: &mut SymbolTable) {

    for spec in declarations {

        let maybe_declared_kind = 
            match spec.kind {
                Some(ref mut k) => Some(typecheck_kind(k, symbol_table, None)),
                None => None,
            };

        let maybe_rhs_kinds = 
            match spec.rhs {
                Some(ref mut exprs) => {
                    Some(typecheck_expression_vec(exprs, symbol_table))
                },
                None => None,
            };


        for i in 0..spec.names.len() {
            match (&maybe_rhs_kinds, &maybe_declared_kind) {
                (&Some(ref rhs_kinds), &Some(ref declared_kind)) => {
                    let init_kind = &rhs_kinds[i];
                    if !kind::are_identical(&init_kind, &declared_kind) {
                        eprintln!("Error: line {}: trying to initialize variable `{}` \
                        of type {} with type {}.",
                                 spec.line_number, spec.names[i], declared_kind, init_kind);
                        exit(1);
                    }
                    symbol_table.add_declaration(spec.names[i].clone(),
                            spec.line_number,
                            Declaration::Variable(declared_kind.clone()),
                            /*inferred*/ false);
                },
                (&Some(ref rhs_kinds), &None) => {
                    let init_kind = &rhs_kinds[i];
                    symbol_table.add_declaration(spec.names[i].clone(),
                            spec.line_number,
                            Declaration::Variable(init_kind.clone()),
                            /*inferred*/ true);
                },
                (&None, &Some(ref declared_kind)) => {
                    symbol_table.add_declaration(spec.names[i].clone(),
                    spec.line_number,
                    Declaration::Variable(declared_kind.clone()),
                    /*inferred*/ false);
                },
                (&None, &None) => unreachable!()
            }
        }
    }
}

fn typecheck_type_declarations(declarations: &mut [TypeSpec], symbol_table: &mut SymbolTable) {

    for spec in declarations {
        let kind = typecheck_kind(&mut spec.kind, symbol_table, Some(&spec.name));
        symbol_table.add_declaration(spec.name.clone(),
                                     spec.line_number,
                                     Declaration::Type(kind),
                                     /*inferred*/ false);
    }

}

fn typecheck_statements(statements: &mut [StatementNode], symbol_table: &mut SymbolTable) {
    for s in statements {
        typecheck_statement(s, symbol_table);
    }
}

fn typecheck_function_declaration(name: &str,
                                   params: &mut [Field],
                                   return_kind: &mut Option<Box<AstKindNode>>,
                                   body: &mut [StatementNode],
                                   line: u32,
                                   symbol_table: &mut SymbolTable) {

    let mut param_tuples = Vec::new();
    for f in params.iter_mut() {
        let k = typecheck_kind(&mut f.kind, symbol_table, None);
        for i in 0..f.identifiers.len() {
            param_tuples.push((&f.identifiers[i], f.line_number, k.clone()))
        }
    }

    let real_return_kind = 
        match return_kind {
            &mut Some(ref mut ret) => Some(typecheck_kind(ret, symbol_table, None)),
            &mut None => None
        };

    symbol_table.add_declaration(name.to_string(),
                                 line,
                                 Declaration::Function{
                                     params: param_tuples.iter().map(|x| x.2.clone()).collect(),
                                     return_kind: real_return_kind.clone(),
                                 },
                                 false);

    let new_scope = &mut symbol_table.new_scope();
    new_scope.return_kind = real_return_kind;
    new_scope.in_function = true;

    for f in param_tuples {
        new_scope.add_declaration(f.0.to_string(), f.1,
                                  Declaration::Variable(f.2), false);
    }

    typecheck_statements(body, new_scope);
}

fn typecheck_statement(stmt: &mut StatementNode,
                           symbol_table: &mut SymbolTable) {
    match stmt.statement {
        Statement::Empty => {},
        Statement::Break => {},
        Statement::Continue => {},
        Statement::Expression(ref mut exp) => {
            typecheck_expression(exp, symbol_table);
        },
        Statement::Return(ref mut exp) => {
            // Since statements happen only inside functions, return only happens inside functions
            let maybe_actual_kind =
                if let &mut Some(ref mut exp) = exp {
                    Some(typecheck_expression(&mut **exp, symbol_table))
                } else {
                    None
                };

            match (maybe_actual_kind, &symbol_table.return_kind)  {
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
            // TODO
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
                let resolved_kind = kind.resolve();
                if let &Kind::Basic(..) = resolved_kind {
                } else {
                    eprintln!("Error: line {}: trying to print something that resolves \
                    to a {}", expr.line_number, resolved_kind);
                    exit(1);
                }
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
            
            if !are_identical(exp_type.resolve(), &Kind::Basic(BasicKind::Bool)) {
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

            if !are_identical(exp_type.resolve(), &Kind::Basic(BasicKind::Bool)) {
                println!("Error: line {}: condition must be of type bool.", 
                         stmt.line_number);
                exit(1);
            }

            {
                let new_scope = &mut init_scope.new_scope();
                typecheck_statements(if_branch, new_scope);
            }

            let else_scope = &mut init_scope.new_scope();
            match *else_branch {
                Some(ref mut stmt) => typecheck_statement(stmt, else_scope),
                None => {},
            }
        }

        Statement::Switch { ref mut init, ref mut expr, ref mut body } => {
            let init_scope = &mut symbol_table.new_scope();
            typecheck_statement(init, init_scope);
            let exp_type = 
                if let Some(ref mut expr) = *expr {
                    let exp_type = typecheck_expression(expr, init_scope);
                    if !exp_type.resolve().is_comparable() {
                        eprintln!("Error: line {}: type {} is not comparable",
                                  expr.line_number, exp_type);
                        exit(1);
                    }
                    exp_type
                } else {
                    Kind::Basic(BasicKind::Bool)
                };


            for cc in body {
                match cc.switch_case {
                    SwitchCase::Cases(ref mut cases) => {
                        for case in cases {
                            let cc_type = typecheck_expression(case, init_scope);
                            if !are_identical(&cc_type, &exp_type) {
                                eprintln!("Error: line {}: mismatched case type {}; \
                                         expected {}.", 
                                         cc.line_number, cc_type, exp_type);
                                exit(1);
                            }
                        }
                    }
                    SwitchCase::Default => {},
                }
                

                let new_scope = &mut init_scope.new_scope();
                for mut stmt in &mut cc.statements {
                    typecheck_statement(&mut stmt, new_scope);
                }
            }
        }
        Statement::IncDec { ref is_dec, ref mut expr } => {
            // TODO: check if is addressable
            // or do we need to do it? I beleive we did it in the weed phase
            let exp_type = typecheck_expression(expr, symbol_table);
            let base = exp_type.resolve();
            if !base.is_numeric() {
                eprintln!("Error: line {}: attempt to increment/decrement a non-numeric type \
                {},", expr.line_number, exp_type);
                exit(1);
            }
        }
    }
}

fn typecheck_kind(ast: &mut AstKindNode, 
                      symbol_table: &mut SymbolTable, 
                      top_name: Option<&str>) -> Kind { 
                    // top_name is to prevent recursive definitions in structs
    match ast.ast_kind {
        AstKind::Identifier { ref name } => {
            match top_name {
                Some(ref top_name) => {
                    if name == top_name {
                        eprintln!("Error: line {}: trying to recursively use {} \
                        in type definition:", ast.line_number, name);
                        exit(1);
                    }
                }
                None => {},
            }
            if let Declaration::Type(ref kind) = symbol_table.get_symbol(name, ast.line_number).declaration {
                return kind.clone();
            } else {
                eprintln!("Error: line {}: `{}` is not a type.", ast.line_number, name);
                exit(1);
            }
        },
        AstKind::Slice { ref mut base } => {
            return Kind::Slice(Box::new(typecheck_kind(base, symbol_table, top_name)))
        },
        AstKind::Array { ref mut base, ref size } => {
            // TODO: parse the size and replace the 0
            return Kind::Array(Box::new(typecheck_kind(base, symbol_table, top_name)), 0)
        },
        AstKind::Struct { ref mut fields } => {
            let mut kind_fields = Vec::new();
            let mut previous_names = HashSet::new();
            for field in fields {
                let field_kind = typecheck_kind(&mut field.kind, symbol_table, top_name);
                for id in &field.identifiers {
                    if (&*id != "_") {
                        if previous_names.contains(&*id) {
                            eprintln!("Error: line {}: duplicate struct field `{}`.", ast.line_number, id);
                            exit(1);
                        }
                        previous_names.insert(id.clone());
                    }
                    kind_fields.push(kind::Field{name: id.clone(), kind: field_kind.clone()});
                }
            }
            return Kind::Struct(kind_fields)
        }
    }
}


fn typecheck_expression(exp: &mut ExpressionNode, symbol_table: &mut SymbolTable) -> Kind {
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

        ref mut a@Expression::FunctionCall { .. } => {
            // Here I do this weird thing where I reassign the node to either a function call or a
            // type cast depending on what the primary expression is.
            
            let primary;
            let mut arguments;
            
            // To move things out of a borrowed value, I need to put something in its place
            if let Expression::FunctionCall{ primary: p, arguments: a } = mem::replace(
                a, Expression::Identifier{ name: "^ this is dumb ^".to_string()}) {
                primary = p;
                arguments = a;
            } else {
                unreachable!();
            }

            *a = 
            if let Some(ref name) =
                // fml
                if let Expression::Identifier{ref name} = primary.expression {
                    Some(name.clone())
                }
                else {
                    None
                }
            {
                // Note: we don't need to check if we're trying to call init because init is not
                // going to be in scope anyway.
                // And we can also have a type called `init`.

                let decl = symbol_table.get_symbol(&name, exp.line_number).declaration.clone();
                match decl { 
                    Declaration::Type(cast_kind) => {
                        if (arguments.len() != 1) {
                            eprintln!("Error: line {}: Type casts take exactly one parameter.",
                                      exp.line_number);
                            exit(1);
                        }
                        let mut inner_expr = arguments.drain(0..1).next().unwrap();
                        let expr_kind = typecheck_expression(&mut inner_expr, symbol_table);

                        let resolved_cast_kind = cast_kind.resolve();
                        let resolved_expr_kind = expr_kind.resolve();

                        if let &Kind::Basic(ref cast_basic) = resolved_cast_kind {
                            if are_identical(resolved_cast_kind, resolved_expr_kind) ||
                                (resolved_cast_kind.is_numeric() && resolved_expr_kind.is_numeric()) ||
                                    (cast_basic == &BasicKind::String && resolved_expr_kind.is_integer()) {

                            } else {
                                eprintln!("Error: line {}: Trying to cast expression of type {} \
                                to incompatible type {}.",
                                    exp.line_number, expr_kind, cast_kind);
                                exit(1);

                            }
                        } else {
                            eprintln!("Error: line {}: Cast type must resolve to a basic type; \
                                {} resolves to {} which is not a basic type.",
                                exp.line_number, cast_kind, resolved_cast_kind);
                            exit(1);
                        }

                        exp.kind = cast_kind.clone();
                        Expression::TypeCast{ expr: Box::new(inner_expr) }
                    },
                    Declaration::Function{ref params, ref return_kind} => {

                        if arguments.len() != params.len() {
                            eprintln!("Error: line {}: `{}` takes {} arguments but only {} were provided.",
                                      exp.line_number, &name, params.len(), arguments.len());
                            exit(1);
                        }

                        let argument_kinds = typecheck_expression_vec(&mut arguments,symbol_table);
                        for (i, (ref ak, ref pk)) in argument_kinds.iter().zip(params.iter()).enumerate() {
                            if !are_identical(&ak, &pk) {
                                eprintln!("Error: line {}: argment {} that was provided for function `{}` is of type {} \
                                but should be of type {}.",
                                          exp.line_number, i+1, &name, ak, pk);
                                exit(1);
                            }
                        }
                        
                        match return_kind {
                            &Some(ref r) => {
                                exp.kind = r.clone();
                            }
                            &None => {
                                exp.kind = Kind::Undefined.clone();
                            }
                        }
                        Expression::FunctionCall{ primary, arguments }
                    },
                    _ => {
                        eprintln!("Error: line {}: `{}` is not a type or a function.",
                                  exp.line_number, name);
                        exit(1);
                    }
                }
            } else {
                eprintln!("Error: line {}: primary expression for function call or \
                type cast must be an identifier.", exp.line_number);
                exit(1);
            };

            return exp.kind.clone()
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
            panic!("This should not happen at this phase.");
        }
    } 
    return Kind::Undefined;
}

fn typecheck_expression_vec(exprs: &mut [ExpressionNode], symbol_table: &mut SymbolTable) ->
Vec<Kind> {
    let mut ret = Vec::new();
    for e in exprs {
        ret.push(typecheck_expression(e, symbol_table));
    }
    ret
}

// Question: isn't it an expression that is addressable or not?
fn is_addressable(exp: &ExpressionNode) -> bool {
    // TODO: this
    true
}

// Need also to check if kinds are valid for op
fn get_kind_binary_op(a: &Kind, b: &Kind, op: BinaryOperator, line_number: u32) -> Kind {
    if !are_identical(a, b) {
        eprintln!("Error: line {}: trying to do operation {:?} on expressions \
                  of different types {} and {}", line_number, op, a, b);
        exit(1);
    }

    match op {
        BinaryOperator::Or | BinaryOperator::And => {
            if a.is_boolean() {
                return a.clone();
               //return Kind::Basic(BasicKind::Bool)
            } else {
               eprintln!("Error: line {}: trying to perform an invalid operation on a {}", line_number, a);
               exit(1)
            }
        },
        BinaryOperator::Eq | BinaryOperator::Neq => {
           if a.is_comparable() {
               return Kind::Basic(BasicKind::Bool)
           } else {
               eprintln!("Error: line {}: trying to perform an invalid operation on a non-comparable type {}", line_number, a);
               exit(1)
           }
        },
        BinaryOperator::Lt | BinaryOperator::Leq | BinaryOperator::Gt | BinaryOperator::Geq => {
           if a.is_ordered() {
               return Kind::Basic(BasicKind::Bool)
           } else {
               eprintln!("Error: line {}: trying to perform an invalid operation on a non-ordered type {}", line_number, a);
               exit(1)
           }
        },
        BinaryOperator::Sub | BinaryOperator::Mul | BinaryOperator::Div |  BinaryOperator::Add=> {
            let is_add: bool;

            match op {
                BinaryOperator::Add => is_add = true,
                _ => is_add = false
            }

            if a.is_numeric() || (is_add && a.is_string()) {
                return a.clone()
            } else {
                eprintln!("Error: line {}: trying to perform an arithmetic operation on non-numerical (or string) type {}", line_number, a);
                exit(1)
            }
        },
        BinaryOperator::BwXor | BinaryOperator::BwOr | BinaryOperator::Mod
        | BinaryOperator::BwAnd | BinaryOperator::BwAndNot | BinaryOperator::LShift
        | BinaryOperator::RShift => {
            if a.is_integer() {
                return a.clone();
            } else {
                eprintln!("Error: line {}: trying to perform a bitwise operation on non-integer type {}", line_number, a);
                exit(1)
            }
        },
   }
}

fn get_kind_unary_op(kind: &Kind, op: UnaryOperator, line_number: u32) -> Kind {
    match op {
        UnaryOperator::Plus | UnaryOperator::Neg =>  {
            match kind.resolve() {
                &Kind::Basic(BasicKind::Int) => Kind::Basic(BasicKind::Int),
                &Kind::Basic(BasicKind::Float) => Kind::Basic(BasicKind::Float),
                _ => {
                    eprintln!("Error: line {}: trying to perform an invalid operation on a {}", line_number, kind);
                    exit(1);
                }
            }
        },
        UnaryOperator::BwCompl => {
            match kind.resolve() {
                &Kind::Basic(BasicKind::Int) => Kind::Basic(BasicKind::Int),
                _ => {
                    eprintln!("Error: line {}: trying to perform an invalid operation on a {}", line_number, kind);
                    exit(1);
                }
            }
        }
        UnaryOperator::Not => {
            match kind.resolve() {
                &Kind::Basic(BasicKind::Bool) => Kind::Basic(BasicKind::Bool),
                _ => {
                    eprintln!("Error: line {}: trying to perform an invalid operation on a {}", line_number, kind);
                    exit(1);
                }
            }
        }
    }
}
