use ast::*;


fn indent_print(text: String, indent: i32) {
    while indent > 0 {
        print!("    ");
        indent++;
    }
    print!("{}", text);
}

/*
PROGRAM PRETTY PRINTING
========================================= */
pub fn pretty_print_program(root: &Program){
    println!("package {}", root.package_name);

    for node in root.declarations.iter(){
        pretty_print_top_level_declaration(node, 0);
        println!()
    }
}

/*
TOP LEVEL DECLARATION PRETTY PRINTING
========================================= */
fn pretty_print_top_level_declaration(node: &TopLevelDeclarationNode, indent: i32){
    match node.top_level_declaration {
        TopLevelDeclaration::VarDeclarations { ref declarations } => {
            println!("var (");
            for var_spec in declarations.iter(){
                pretty_print_var_declaration(var_spec, indent+1);
                println!()
            }
            print!(")");
        },
        TopLevelDeclaration::TypeDeclarations { ref declarations } => {
            println!("type (");
            for type_spec in declarations.iter(){
                pretty_print_type_declaration(type_spec, indent+1);
                println!()
            }
            print!(")");
        },
        TopLevelDeclaration::FunctionDeclaration { ref name, ref parameters, ref return_kind, ref body} =>  {
            pretty_print_function_declaration(&name, &parameters, &return_kind, &body);
        }
    }
}

/*
DECLARATION PRETTY PRINTING
========================================= */

/// Pretty print variable declarations
fn pretty_print_var_declaration(var_spec: &VarSpec, indent: i32){

    let len = var_spec.names.len();
    let mut count = 0;

    for name in var_spec.names.iter() {
        indent_print(name, indent);

        if count < len - 1 {
            print!(", ")
        }

        count = count + 1
    }

    match var_spec.kind {
        Some(ref k) => {
            print!(" ");
            pretty_print_ast_kind(&k.ast_kind);
        }
        None => ()
    }

    match var_spec.rhs {
        Some(ref v) => {
            print!(" = ");
            for (count, expr) in v.iter().enumerate(){

                pretty_print_expression(expr);
                if count < v.len() - 1 {
                    print!(", ")
                }

            }
        },
        None => ()
    }
}


/// Pretty print type declarations
fn pretty_print_type_declaration(type_spec: &TypeSpec, indent: i32){
    indent_print(&type_spec.name, indent);
    pretty_print_ast_kind(&type_spec.kind.ast_kind)
}


/// Pretty print function declarations
fn pretty_print_function_declaration(name: &String,
                                     parameters: &Vec<Field>,
                                     return_kind: &Option<Box<AstKindNode>>,
                                     body: &Vec<StatementNode>, 
                                     indent: i32){

    indent_print("", indent);
    print!("func {} (", name);

    let len = parameters.len();
    let mut count = 0;

    for p in parameters.iter(){
        pretty_print_field(p, 0);

        if count < len - 1 {
            print!(", ")
        }

        count = count + 1;
    }

    print!(")");

    match return_kind {
        &Some(ref k) => {
            pretty_print_ast_kind(&k.ast_kind)
        },
        &None =>()
    }

    println!("{{");

    for stmt in body.iter() {
        pretty_print_statement(stmt, indent+1);
        println!();

    }

    print!("}}");

}

/// Pretty prints ast kinds such as identifier, slices, arrays, and structs
fn pretty_print_ast_kind(kind: &AstKind){
    match kind {
        &AstKind::Identifier { ref name} => print!("{}", &name),
        &AstKind::Slice { ref base } => {
            print!("[]");
            pretty_print_ast_kind(&base.ast_kind)
        },
        &AstKind::Array { ref base, ref size} => {
            print!("[{}]", &size);
            pretty_print_ast_kind(&base.ast_kind)
        },
        &AstKind::Struct { ref fields } => {
            println!("struct {{");
            for f in fields.iter(){
                pretty_print_field(f, indent+1);
                println!(";")
            }
            print!("}}");
        }
    }
}

/// Pretty prints basic types
fn pretty_print_kind(kind: Kind){
    match kind {
        Kind::Undefined => print!("undefined"),
        Kind::Basic(basic) => {
            match basic {
                BasicKind::Int => print!("int"),
                BasicKind::Float => print!("float64"),
                BasicKind::Rune => print!("rune"),
                BasicKind::String => print!("string")
            }
        }
    }
}

/// Pretty prints fields
fn pretty_print_field(field: &Field, indent: i32){

    let len = field.identifiers.len();
    let mut count = 0;

    for id in field.identifiers.iter() {
        print_indent(id, indent);

        if count < len - 1 {
            print!(", ");
        }

        count = count + 1;
    }
    print!(" ");

    pretty_print_ast_kind(&field.kind.ast_kind);
}


/*
STATEMENT PRETTY PRINTING
========================================= */
/// The convention is that statements do not put an newline after themselves
fn pretty_print_statement(stmt: &StatementNode, indent: i32) {
    match stmt.statement {
        Statement::Empty => (),
        Statement::Block(ref v) => {
            print_indent("", indent);
            println!("{{");

            for s in v.iter(){
                pretty_print_statement(s, indent+1);
                println!()
            }

            print_indent("}}", indent);
        },
        Statement::Expression(ref expr) => {
            pretty_print_expression(&*expr)
        },
        Statement::Assignment { ref lhs, ref rhs} => {
            print_indent("", indent);
            let mut count = 0;
            let len = lhs.len();
            for expr in lhs.iter() {
                pretty_print_expression(expr);

                if count < len - 1 {
                    print!(", ");
                }

                count = count + 1;
            }

            print!(" = ");

            let mut count = 0;
            let len = rhs.len();
            for expr in rhs.iter() {
                pretty_print_expression(expr);
                if count < len - 1 {
                    print!(", ");
                }

                count = count + 1;
            }
        },
        Statement::OpAssignment { ref lhs, ref rhs, operator } => {
            pretty_print_expression(&*lhs);
            print!(" ");
            pretty_print_binary_operator(operator);
            print!("= ");
            pretty_print_expression(&*rhs);
        },
        Statement::VarDeclarations { ref declarations } => {
            print_indent("", indent);
            println!("var (");
            for decl in declarations.iter() {
                pretty_print_var_declaration(decl, indent+1);
                println!();
            }
            print_indent(")", indent);
        },
        Statement::TypeDeclarations { ref declarations } => {
            print_indent("", indent);
            println!("type (");
            for decl in declarations.iter() {
                pretty_print_type_declaration(decl, indent+1);
                println!();
            }
            print_indent(")", indent);
        },
        Statement::ShortVariableDeclaration { ref identifier_list, ref expression_list } => {
            let len = identifier_list.len();
            let mut count = 0;

            print_indent("", indent);
            for id in identifier_list.iter() {
                print!("{}", id);

                if count < len - 1 {
                    print!(", ")
                }

                count = count + 1;
            }

            print!(" := ");

            let len = expression_list.len();
            count = 0;

            for expr in expression_list.iter() {
                pretty_print_expression(expr);

                if count < len - 1 {
                    print!(", ")
                }

                count = count + 1;
            }
        },
        Statement::IncDec { is_dec, ref expr } => {
            print_indent("", indent);
            pretty_print_expression(&*expr);

            if is_dec {
                print!("--")
            } else {
                print!("++")

            }
        },
        Statement::Print { ref exprs } => {
            print_indent("", indent);
            print!("print( ");

            for expr in exprs.iter() {
                pretty_print_expression(expr)
            }

            print!(")")
        },
        Statement::Println { ref exprs } => {
            print_indent("", indent);
            print!("println( ");

            for expr in exprs.iter() {
                pretty_print_expression(expr)
            }

            print!(")")
        },
        Statement::If { ref init, ref condition, ref if_branch, ref else_branch } => {
            print_indent("", indent);
            print!("if ");
            pretty_print_statement(&*init, 0);
            print!("; ");
            pretty_print_expression(&*condition);
            println!(" {{");

            for stmt in if_branch.iter(){
                pretty_print_statement(stmt, indent+1);
                println!();
            }

            print_indent("", indent);
            print!("}}");

            match else_branch{
                &Some(ref stmt) => {
                    print!(" else ");
                    pretty_print_statement(&*stmt, indent+1)
                },
                &None => println!()
            }
        },
        Statement::Loop { ref body} => {
            print_indent("", indent);
            println!("for {{");
            for stmt in body.iter(){
                pretty_print_statement(stmt, indent+1);
                println!();
            }
            print_indent("", indent);
            print!("}}");
        },
        Statement::While { ref condition, ref body } => {
            print_indent("", indent);
            print!("for ");
            pretty_print_expression(&*condition);
            println!(" {{");

            for stmt in body.iter(){
                pretty_print_statement(stmt, indent+1);
                println!();
            }

            print_indent("", indent);
            print!("}}");
        },
        Statement::For {ref init, ref condition, ref post, ref body } => {
            print_indent("", indent);
            print!("for ");
            pretty_print_statement(&*init, 0);
            print!("; ");
            pretty_print_expression(&*condition);
            print!("; ");
            pretty_print_statement(&*post, 0);
            println!(" {{");

            for stmt in body.iter(){
                pretty_print_statement(stmt. indent+1);
                println!();
            }

            print_indent("", indent);
            print!("}}");
        },
        Statement::Switch { ref init, ref expr, ref body } => {
            print_indent("", indent);
            print!("switch ");

            pretty_print_statement(&*init, 0);
            print!("; ");

            match expr {
                &Some(ref e) => pretty_print_expression(&*e),
                &None => ()
            }

            println!(" {{");

            for case_clause in body.iter(){
                pretty_print_case_clause(case_clause, indent+1);
            }

            print_indent("", indent);
            print!("}}");
        },
        Statement::Break => print_indent("break;", indent),
        Statement::Continue => print_indent("continue;", indent),
        Statement::Return(ref expr) => {
            print_indent("return ", indent);
            match expr {
                &Some(ref e) => pretty_print_expression(&*e),
                &None => ()
            }
        }
    }
}

/// Pretty prints case clause
fn pretty_print_case_clause(case_clause: &CaseClause, indent: i32){
    pretty_print_switch_case(&case_clause.switch_case, indent);
    println!();

    for stmt in case_clause.statements.iter(){
        pretty_print_statement(stmt, indent+1);
        println!();
    }

}


fn comma_separated_expressions(v: &Vec<ExpressionNode>, indent: i32) {
    for (count, expr) in v.iter().enumerate() {
        pretty_print_expression(expr);
        if count < v.len() - 1 {
            print!(", ");
        }
    }
}

/// Pretty prints switch case
fn pretty_print_switch_case(switch_case: &SwitchCase, indent: i32){
    print_indent("", indent);
    match switch_case {
        &SwitchCase::Default => println!("default: "),
        &SwitchCase::Cases(ref v) => {
            print!("case ");
            comma_separated_expressions(v);
            print!(": ");
        }
    }
}

/*
EXPRESSION PRETTY PRINTING
========================================= */

fn pretty_print_expression(expr: &ExpressionNode){
    match expr.expression {
        Expression::Identifier { ref name } => print!("{}", name),
        Expression::RawLiteral { ref value } => print!("{}", value),
        Expression::BinaryOperation { op, ref lhs, ref rhs} => {
            print!("( ");
            pretty_print_expression(&*lhs);
            print!(" ");
            pretty_print_binary_operator(op);
            print!(" ");
            pretty_print_expression(&*rhs);
            print!(" )");
        },
        Expression::UnaryOperation { op, ref rhs } => {
            pretty_print_unary_operator(op);
            pretty_print_expression(&*rhs);
        },
        Expression::Index { ref primary, ref index } => {
            pretty_print_expression(&*primary);
            print!("[");
            pretty_print_expression(&*index);
            print!("]");
        },
        Expression::Selector { ref primary, ref name} => {
            pretty_print_expression(&*primary);
            print!(".{}", name);
        },
        Expression::FunctionCall { ref primary, ref arguments } => {
            pretty_print_expression(&*primary);
            print!("(");

            let len = arguments.len();
            let mut count = 0;

            for arg in arguments.iter() {
                pretty_print_expression(arg);

                if count < len - 1 {
                    print!(" , ")
                }

                count = count + 1;
            }

            print!(" )");
        },
        Expression::Append { ref lhs, ref rhs } => {
            print!("append( ");
            pretty_print_expression(&*lhs);
            print!(" , ");
            pretty_print_expression(&*rhs);
            print!(" )");
        },
        Expression::TypeCast { ref expr } => {
            panic!("There should not be type casts in the AST at this point.")
        }
    }
}

/// Prints binary operator
fn pretty_print_binary_operator(bin: BinaryOperator){
    match bin {
        BinaryOperator::Or => print!("||"),
        BinaryOperator::And => print!("&&"),
        BinaryOperator::Eq => print!("=="),
        BinaryOperator::Neq => print!("!="),
        BinaryOperator::Lt => print!("<"),
        BinaryOperator::Leq => print!("<="),
        BinaryOperator::Gt => print!(">"),
        BinaryOperator::Geq => print!(">="),
        BinaryOperator::Add => print!("+"),
        BinaryOperator::Sub => print!("-"),
        BinaryOperator::BwOr => print!("|"),
        BinaryOperator::BwXor => print!("^"),
        BinaryOperator::Mul => print!("*"),
        BinaryOperator::Div => print!("/"),
        BinaryOperator::Mod => print!("%"),
        BinaryOperator::LShift => print!("<<"),
        BinaryOperator::RShift => print!(">>"),
        BinaryOperator::BwAnd => print!("&"),
        BinaryOperator::BwAndNot => print!("&^")
    }
}

/// Prints unary operator
fn pretty_print_unary_operator(un: UnaryOperator){
    match un {
        UnaryOperator::Plus => print!("+"),
        UnaryOperator::Neg => print!("-"),
        UnaryOperator::BwCompl => print!("~"),
        UnaryOperator::Not => print!("!"),
    }
}
