use std::fmt::Write;
use util::*;
use ast::*;
use kind::*;
use std::fs::File;
use std::io::prelude::*;

struct CodeGenVisitor {
    indent: u32,
    id_counter: u32,
    init_functions: Vec<String>
}

impl CodeGenVisitor{
    fn visit_program(&mut self, root: &Program) {
        print_header();

        for decl in &root.declarations {
            self.visit_top_level_declaration(&decl);
        }

        for init_func_name in &self.init_functions {
            println!("{}();",init_func_name);
        }

        println!("main();");
    }


    fn visit_top_level_declaration(&mut self, decl: &TopLevelDeclarationNode) {
        match decl.top_level_declaration {
            TopLevelDeclaration::VarDeclarations { ref declarations } => {
                for d in declarations.iter(){
                    self.visit_var_spec(&d)
                }
            }

            TopLevelDeclaration::FunctionDeclaration 
                (Function { ref name, ref parameters, ref body, .. }) => {
                if name == "_" {
                    return;
                }

                let mut func_name = name.clone();

                if func_name == "init" {
                    func_name = format!("init_{}", self.create_id());
                    self.init_functions.push(func_name.clone());
                }

                let mut params_string = "".to_string();
                for (i,field) in parameters.iter().enumerate() {
                    for (j,id) in field.identifiers.iter().enumerate() {
                        write!(params_string, "{}", id).unwrap();
                        if i < parameters.len() - 1 || j < field.identifiers.len() - 1 {
                            write!(params_string, ", ").unwrap();
                        }
                    }
                }

                println!("function {} ( {} ) {{", func_name, params_string);

                self.indent += 1;
                self.visit_statements(&body);
                self.indent -= 1;

                println!("}}");
            },

            _ => {}
        }
    }

    fn visit_var_spec(&mut self, var_spec: &VarSpec){
        match var_spec.rhs {
            Some(ref values) => {
                let mut pre_string = "".to_string();
                let mut post_string = "".to_string();

                for (name, rhs) in var_spec.names.iter().zip(values.iter()) {
                    let mut new_post = String::new();
                    self.visit_expression(&rhs, &mut pre_string, &mut new_post);
                    if name != "_" {
                        writeln!(post_string, "{}var {} = deepCopy({});",
                            indent(self.indent), name, new_post).unwrap();
                    }
                }

                println!("{} \n {}", pre_string, post_string);
            }
            None => {
                for name in var_spec.names.iter() {
                    if name != "_" {
                        print!("{}var {} = ", indent(self.indent), name);
                        self.visit_var_initialization(&var_spec.evaluated_kind);
                        println!(";");
                    }
                }
            }
        }
    }

    fn visit_var_initialization(&mut self, var_kind: &Kind){
        match &var_kind.resolve() {
            &Kind::Basic(BasicKind::Int) | &Kind::Basic(BasicKind::Float)| &Kind::Basic(BasicKind::Rune) => print!("0"),
            &Kind::Basic(BasicKind::Bool) => print!("false"),
            &Kind::Basic(BasicKind::String) => print!("''"),
            &Kind::Array(ref kind, ref length) => {
                print!("makeArray({}, ",length);
                self.visit_var_initialization(&kind);
                print!(")");
            }
            &Kind::Slice(..) => {
                println!("{{", );

                self.indent+=1;
                println!("{} length: 0,", indent(self.indent));
                println!("{}capacity: 0,", indent(self.indent));
                println!("{} contents: []", indent(self.indent));
                self.indent-=1;
                print!("{}}}", indent(self.indent));
            }
            &Kind::Struct(ref fields) => {
                println!("{{");
                self.indent+=1;
                for field in fields.iter(){
                    print!("{}ㆭ{}:", indent(self.indent), field.name, );
                    self.visit_var_initialization(&field.kind);
                    println!(",");
                }
                self.indent-=1;
                print!("{}}}", indent(self.indent));
            }
            _ => {panic!("initializing value not supported")}
        }
    }


    fn visit_statements(&mut self, statements: &[StatementNode]) {
        for s in statements {
            self.visit_statement(s);
        }
    }

    fn visit_statement(&mut self, stmt: &StatementNode) {
        match stmt.statement {
            Statement::Empty => {},
            Statement::Break => {
                print!("{}", indent(self.indent));
                println!("break;")
            },
            Statement::Continue => {
                print!("{}", indent(self.indent));
                println!("continue;")
            },
            Statement::Expression(ref exp) => {
                let mut pre = String::new();
                let mut post = String::new();
                self.visit_expression(exp, &mut pre, &mut post);
                print!("{}",pre);
                println!("{}{};", indent(self.indent), &mut post);
            },
            Statement::Return(ref exp) => {
                match exp {
                    &Some(ref e) => {
                        let mut pre = String::new();
                        let mut post = String::new();
                        self.visit_expression(e, &mut pre, &mut post);
                        print!("{}",pre);
                        println!("{}return {};", indent(self.indent), &mut post);
                    },
                    &None => {
                        print!("{}", indent(self.indent));
                        println!("return;");
                    }
                }
            },
            Statement::ShortVariableDeclaration { ref identifier_list, ref expression_list, ref is_assigning } => {
                let mut global_pre = String::new();
                let mut global_post = String::new();
                let mut temps = Vec::new();

                for expr in expression_list.iter() {
                    let temp_string = format!("ㄭ{}", self.create_id());
                    temps.push(temp_string.clone());

                    write!(global_post, "{}var {} = ",indent(self.indent), temp_string).unwrap();
                    self.visit_expression(&expr, &mut global_pre, &mut global_post);
                    write!(global_post, ";\n").unwrap();
                }

                for x in 0..identifier_list.len() {
                    if identifier_list[x] != "_" {
                        if is_assigning[x] {
                            write!(global_post, "{}{} = deepCopy({});\n", indent(self.indent), identifier_list[x], temps[x]).unwrap();
                        } else {
                            write!(global_post, "{}var {} = deepCopy({});\n", indent(self.indent), identifier_list[x], temps[x]).unwrap();
                        }
                    }
                }

                println!("{}{}", global_pre, global_post);
            },
            Statement::VarDeclarations { ref declarations } => {
                for decl in declarations.iter() {
                    self.visit_var_spec(decl);
                }
            },
            Statement::TypeDeclarations { .. } => {},
            Statement::Assignment { ref lhs, ref rhs } => {

                let mut global_pre = String::new();
                let mut global_post = String::new();
                let mut temps = Vec::new();
                let mut lhs_post_strings = Vec::new();

                for expr in lhs.iter() {
                    let mut post = String::new();
                    self.visit_expression(&expr, &mut global_pre, &mut post);
                    lhs_post_strings.push(post);
                }

                for expr in rhs.iter() {
                    let temp_string = format!("temp_{}", self.create_id());
                    temps.push(temp_string.clone());

                    write!(global_post, "{}var {} = ",indent(self.indent), temp_string).unwrap();
                    self.visit_expression(&expr, &mut global_pre, &mut global_post);
                    write!(global_post, ";\n").unwrap();
                }

                for x in 0..lhs_post_strings.len() {
                    write!(global_post, "{}{} = deepCopy({});\n", indent(self.indent), lhs_post_strings[x], temps[x]).unwrap();
                }

                println!("{}{}", global_pre, global_post);
            },
            Statement::OpAssignment { ref lhs, ref rhs, ref operator } => {
                let mut pre_lhs = String::new();
                let mut post_lhs = String::new();
                self.visit_expression(&lhs, &mut pre_lhs, &mut post_lhs);

                let mut pre_rhs = String::new();
                let mut post_rhs = String::new();
                self.visit_expression(&rhs, &mut pre_rhs, &mut post_rhs);

                println!("{}", pre_lhs);
                println!("{}", pre_rhs);
                print!("{}{} = ", indent(self.indent), post_lhs);
                print!("{}",generate_binary_op(&operator));
                println!("({}, {});", post_lhs, post_rhs);
            },
            Statement::Block(ref statements) => {
                for stmt in statements {
                    self.visit_statement(stmt);
                }
            },
            Statement::Print { ref exprs } => {
                self.codegen_print(exprs, false);
            },
            Statement::Println { ref exprs } => {
                self.codegen_print(exprs, true);
            },
            Statement::For { ref init, ref condition, ref post, ref body } => {
                self.visit_statement(init);
                let mut condition_string;
                if let &Some(ref condition) = condition {
                    condition_string = String::new();
                    self.codegen_expression_iife(condition, &mut condition_string);
                } else {
                    condition_string = "true".to_string();
                }

                println!("{}for (;{};(function() {{",indent(self.indent), condition_string);

                self.indent+=1;
                println!("{}// post:",indent(self.indent));
                self.visit_statement(post);
                self.indent-=1;

                println!("{}}}() ) ) {{", indent(self.indent));

                self.indent+=1;
                self.visit_statements(body);
                self.indent-=1;

                println!("{}}}",indent(self.indent));

            },
            Statement::If { ref init, ref condition, ref if_branch, ref else_branch } => {
                self.visit_statement(init);
                let mut pre = String::new();
                let mut post = String::new();
                self.visit_expression(condition, &mut pre, &mut post);
                print!("{}",pre);
                println!("{}if ({}) {{",indent(self.indent),post);
                self.indent+=1;
                self.visit_statements(if_branch);
                self.indent-=1;
                println!("{}}} else {{",indent(self.indent));
                if let &Some(ref else_branch) = else_branch {
                    self.indent+=1;
                    self.visit_statement(else_branch);
                    self.indent-=1;
                }
                println!("{}}}",indent(self.indent));

            },
            Statement::Switch { ref init, ref expr, ref body } => {
                self.visit_statement(init);

                    let mut pre;
                    let mut post;
                if let &Some(ref expr) = expr {
                    pre = String::new();
                    post = String::new();
                    self.visit_expression(expr, &mut pre, &mut post);
                    print!("{}",pre);
                } else {
                    post = "true".to_string();
                }
                println!("{}switch ({}) {{",indent(self.indent),post);
                self.indent+=1;
                for case_clause in body {
                    match &case_clause.switch_case {
                        &SwitchCase::Default => {
                            println!("{}default:", indent(self.indent));
                        }
                        &SwitchCase::Cases(ref cases) => {
                            for case in cases {
                                let mut case_code = String::new();
                                self.codegen_expression_iife(&case, &mut case_code);
                                println!("{}case {}:", indent(self.indent), case_code);
                            }
                        }
                    }
                    self.indent+=1;
                    self.visit_statements(&case_clause.statements);
                    println!("{}break;", indent(self.indent));
                    self.indent-=1;
                }
                self.indent-=1;
                println!("{}}}",indent(self.indent));
            },
            Statement::IncDec { ref expr, is_dec } => {
                let mut pre = String::new();
                let mut post = String::new();
                self.visit_expression(expr, &mut pre, &mut post);
                print!("{}",pre);
                let function = 
                match (is_dec, expr.kind.is_integer()) {
                    (false, true) => "binary_Add_int",
                    (true, true) => "binary_Sub_int",
                    (false, false) => "binary_Add",
                    (true, false) => "binary_Sub",
                };
                print!("{}{} = {}({},1);\n",
                        indent(self.indent),
                        post,
                        function,
                        post);

            }
        }
    }

    fn codegen_print(&mut self,
                     exprs: &Vec<ExpressionNode>,
                     is_println: bool) {
        let mut pre = String::new();
        let mut post = String::new();
        for (i,expr) in exprs.iter().enumerate() {
            let function = 
                match expr.kind.resolve() {
                    Kind::Basic( BasicKind::Float) => "print_float",
                    _ => "print_not_float",
                };
            write!(post,"{}{}(", indent(self.indent), function).unwrap();
            self.visit_expression(expr, &mut pre, &mut post);
            write!(post,");\n").unwrap();
            if is_println && i < exprs.len()-1 { 
                write!(post,"{}print_not_float(\" \");\n", indent(self.indent)).unwrap();
            }
        }
        if is_println {
            write!(post,"{}print_not_float(\"\\n\");\n", indent(self.indent)).unwrap();
        }
        println!("{}{}", pre, post);
    }

    fn codegen_expression_iife(&mut self,
                               exp: &ExpressionNode,
                               post_string: &mut String) {
        let mut new_pre = String::new();
        let mut new_post = String::new();
        self.indent+= 1;
        self.visit_expression(exp, &mut new_pre, &mut new_post);
        self.indent-= 1;
        write!(post_string, "(function() {{\n\
            {}\
            {}return {};}}())",
            new_pre,
            indent(self.indent + 1),
            new_post).unwrap();
    }

    // Convention:
    // each line in pre_string is indented and ends with a semicolon and a newline
    // post_string is not indented or anything
    fn visit_expression(&mut self,
                        exp: &ExpressionNode,
                        pre_string: &mut String,
                        post_string: &mut String) {

        match exp.expression {
            Expression::RawLiteral{ ref value } => {
                match exp.kind {
                    Kind::Basic(BasicKind::Int) | 
                    Kind::Basic(BasicKind::Float) => {
                        write!(post_string, "{}", value).unwrap();
                    },
                    Kind::Basic(BasicKind::Rune) => {
                        let code_no = parse_rune_literal(value);
                        write!(post_string, "{}", code_no).unwrap();
                    },
                    Kind::Basic(BasicKind::String) => {
                        let letter = &value[0..1];
                        match letter {
                            "`" => { // Raw
                                let mut new_string = String::new();
                                for c in value.chars() {
                                    if c == '`' {
                                        continue;
                                    }
                                    if c == '\\' || c == '"' {
                                        new_string = format!("{}{}", new_string, "\\");
                                    }
                                    new_string = format!("{}{}", new_string, c);
                                }
                                write!(post_string, "\"{}\"", new_string).unwrap();
                            },
                            "\"" => { // Interpreted
                                let mut new_string = String::new();
                                let char_vec: Vec<_> = value.chars().collect::<Vec<_>>();
                                let mut skip_next_char = false;
                                for (id, &c) in char_vec.iter().enumerate() {
                                    if skip_next_char {
                                        skip_next_char = false;
                                        continue;
                                    }
                                    if c == '\\' 
                                        && id < char_vec.len()-2
                                        && char_vec[id+1] == 'a' {
                                        new_string = format!("{}\\007", new_string);
                                        skip_next_char = true;
                                    } else {
                                        new_string = format!("{}{}", new_string, c);
                                    }
                                }
                                write!(post_string, "{}", new_string).unwrap();
                            }
                            _ => {
                                panic!("A string should be either interpreted or raw");
                            }
                        }
                    }
                    _ => {
                        panic!("Invalid type of typecasted expression");
                    }
                }
            }

            Expression::Identifier { ref name, .. } => {
                write!(post_string, "{}", name).unwrap();
            }

            Expression::UnaryOperation { ref op, ref rhs } => {
                write!(post_string, "{}(", generate_unary_op(op)).unwrap();
                self.visit_expression(rhs, pre_string, post_string);
                write!(post_string, ")").unwrap();
            }

            Expression::BinaryOperation { ref op, ref lhs, ref rhs } => {
                if *op == BinaryOperator::Or || *op == BinaryOperator::And {
                    write!(post_string, "(").unwrap();
                    self.visit_expression(lhs, pre_string, post_string);
                    write!(post_string, " {} ",
                           if *op==BinaryOperator::Or { "||" } else { "&&" }).unwrap();
                    self.codegen_expression_iife(rhs,  post_string);
                    write!(post_string, ")").unwrap();
                } else {
                    write!(post_string, "{}", generate_binary_op(op)).unwrap();

                    if exp.kind.is_integer() {
                        match op {
                            &BinaryOperator::Add |
                                &BinaryOperator::Sub |
                                &BinaryOperator::Mul |
                                &BinaryOperator::Div => {
                                    write!(post_string, "_int").unwrap();
                                }
                            _ => {}
                        }
                    }

                    write!(post_string, "(").unwrap();
                    self.visit_expression(lhs, pre_string, post_string);
                    write!(post_string, ", ").unwrap();
                    self.visit_expression(rhs, pre_string, post_string);
                    write!(post_string, ")").unwrap();
                }
            }

            Expression::FunctionCall { ref primary, ref arguments } => {
                let tmp_id = self.create_id();
                let mut new_pre_string = String::new();
                let mut new_post_string = String::new();

                // Print the name of the temp variable in the post_string
                write!(post_string, "ⴵ_{}", tmp_id).unwrap();

                // Execute function call outside using different post/prestrings
                write!(new_post_string, "var ⴵ_{} = ", tmp_id).unwrap();

                // Print primary to new_post_string
                self.visit_expression(primary, &mut new_pre_string, &mut new_post_string);

                // Print arguments to new_post_string
                write!(new_post_string, "(").unwrap();
                for (i,arg) in arguments.iter().enumerate() {
                    write!(new_post_string, "deepCopy(").unwrap();
                    self.visit_expression(arg, &mut new_pre_string, &mut new_post_string);
                    write!(new_post_string, ")").unwrap();
                    if i < arguments.len() - 1 {
                        write!(new_post_string, ", ").unwrap();
                    }
                }
                write!(new_post_string, ");").unwrap();

                // Add all hoisted calls, and the new func call to pre_string.
                write!(pre_string, 
                       "{}{}{}\n", 
                       &mut new_pre_string, 
                       indent(self.indent), 
                       &mut new_post_string).unwrap();
            }

            Expression::Index { ref primary, ref index } => {

                let mut primary_value = "".to_string();
                let mut index_value = "".to_string();
                self.visit_expression(primary, pre_string, &mut primary_value);
                self.visit_expression(index, pre_string, &mut index_value);

                match primary.kind {
                    Kind::Slice(..) =>  {
                        write!(post_string, "{}.contents", primary_value).unwrap();
                    },
                    Kind::Array(..) =>  {
                        write!(post_string, "{}", primary_value).unwrap();
                    },
                    _ => panic!("codegening index of something other than slice or array")
                };

                write!(post_string, "[check_bounds({}, {}.length, {})]",
                index_value, primary_value, exp.line_number).unwrap();
            }

            Expression::Selector { ref primary, ref name } => {
                self.visit_expression(primary, pre_string, post_string);
                write!(post_string, ".ㆭ{}", name).unwrap();
            }

            Expression::Append { ref lhs, ref rhs } => {
                write!(post_string, "append(").unwrap();
                self.visit_expression(lhs, pre_string, post_string);
                write!(post_string, ", ").unwrap();
                self.visit_expression(rhs, pre_string, post_string);
                write!(post_string, ")").unwrap();
            }

            Expression::TypeCast { ref expr, .. } => {
                if exp.kind.is_string() && expr.kind.is_integer() {
                    write!(post_string, "String.fromCharCode(").unwrap();
                    self.visit_expression(expr, pre_string, post_string);
                    write!(post_string, ")").unwrap();
                } else if exp.kind.is_integer() && expr.kind.is_floating_point(){
                    write!(post_string, "Math.trunc(").unwrap();
                    self.visit_expression(expr, pre_string, post_string);
                    write!(post_string, ")").unwrap();
                } else {
                    // Do nothing at all
                    self.visit_expression(expr, pre_string, post_string);
                }
            }
        }
    }

    fn create_id(&mut self) -> String{
        self.id_counter += 1;
        return self.id_counter.to_string()
    }
}

pub fn codegen(root: &Program) {
    let mut visitor = CodeGenVisitor{ indent: 0, id_counter: 0, init_functions: Vec::new()  };

    visitor.visit_program(root);
}

fn print_header() {
    let mut f = File::open("src/header.js").expect("Header file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Error when reading header file");

    println!("{}", contents);
}


fn generate_unary_op(op: &UnaryOperator) -> String {
    let mut op_name = "".to_string();
    write!(op_name, "unary_{:?}", op).unwrap();
    op_name
}

fn generate_binary_op(op: &BinaryOperator) -> String {
    let mut op_name = "".to_string();
    write!(op_name, "binary_{:?}", op).unwrap();
    op_name
}
