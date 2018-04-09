use std::fmt::Write;
use ast;
use env;
use util::*;
use ast::*;
use kind;
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
                    self.visit_top_level_var_spec(&d)
                }
            }

            TopLevelDeclaration::FunctionDeclaration 
            { ref name, ref parameters, ref return_kind, ref body } => {

                let mut func_name = name.clone();

                if name == "init" {
                    func_name = format!("{}_{}", name, self.create_id());
                    self.init_functions.push(func_name.clone());
                }

                let mut params_string = "".to_string();
                for field in parameters {
                    for id in &field.identifiers {
                        write!(params_string, "{}, ", id);
                    }
                }


                println!("function {} ( {} ) {{", func_name, params_string);

                self.indent += 1;
                self.visit_statements(&body);
                self.indent -= 1;

                println!("}}");
            },

            _ => return
        }
    }

    fn visit_top_level_var_spec(&mut self, var_spec: &VarSpec){
        match var_spec.rhs {
            Some(ref values) => {
                let mut pre_string = "".to_string();
                let mut post_string = "".to_string();

                for (name, rhs) in var_spec.names.iter().zip(values.iter()) {
                    write!(post_string, "let {} = ", name);
                    self.visit_expression(&rhs, &mut pre_string, &mut post_string);
                    write!(post_string, "\n");
                }
            }
            None => {
                // TODO initialize to zero value or something
            }
        }


    }


    fn visit_variable_declarations(&mut self, declarations: &[VarSpec]) {
        for spec in declarations {

        }
    }

    fn visit_statements(&mut self, statements: &[StatementNode]) {
        for s in statements {
            self.visit_statement(s);
        }
    }

    fn visit_function_declaration(&mut self,
                                  name: &str,
                                  params: &[ast::Field],
                                  body: &[StatementNode]) {

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
                    &Some(..) => {
                        // DO SOMETHING
                    },
                    &None => {
                        print!("{}", indent(self.indent));
                        println!("return;");
                    }
                }
            },
            Statement::ShortVariableDeclaration { ref identifier_list, ref expression_list } => {
            },
            Statement::VarDeclarations { ref declarations } => {
            },
            Statement::TypeDeclarations { ref declarations } => {
                // Nothing
            },
            Statement::Assignment { ref lhs, ref rhs } => {
            },
            Statement::OpAssignment { ref lhs, ref rhs, ref operator } => {
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

                println!("{}while ({}) {{",indent(self.indent), condition_string);
                self.indent+=1;
                self.visit_statements(body);
                println!("{}// post:",indent(self.indent));
                self.visit_statement(post);
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
                // TODO write tests for this
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
                    (true, true) => "binary_Add_int",
                    (false, true) => "binary_Sub_int",
                    (true, false) => "binary_Add",
                    (false, false) => "binary_Sub",
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
        for expr in exprs {
            let function = 
                match expr.kind.resolve() {
                    Kind::Basic( BasicKind::Float) => "print_float",
                    _ => "print_not_float",
                };
            write!(post,"{}{}(", indent(self.indent), function);
            self.visit_expression(expr, &mut pre, &mut post);
            write!(post,");\n");
            if is_println { // TODO: probably not do this on the last space
                write!(post,"{}print_not_float(\" \");\n", indent(self.indent));
            }
        }
        if is_println {
            write!(post,"{}print_not_float(\"\\n\");\n", indent(self.indent));
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
            new_post);
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
                        write!(post_string, "{}", value);
                    },
                    Kind::Basic(BasicKind::Rune) => {
                        // TODO: parse rune and print out integer value
                    },
                    Kind::Basic(BasicKind::String) => {
                        // TODO: raw vs interpreted strings
                    }
                    _ => {
                        panic!("Invalid type of typecasted expression");
                    }
                }
            }

            Expression::Identifier { ref name, .. } => {
                write!(post_string, "{}", name);
            }

            Expression::UnaryOperation { ref op, ref rhs } => {
                write!(post_string, "{}(", print_unary_op(op));
                self.visit_expression(rhs, pre_string, post_string);
                write!(post_string, ")");
            }

            Expression::BinaryOperation { ref op, ref lhs, ref rhs } => {
                if *op == BinaryOperator::Or || *op == BinaryOperator::And {
                    write!(post_string, "(");
                    self.visit_expression(lhs, pre_string, post_string);
                    write!(post_string, " {} ",
                           if *op==BinaryOperator::Or { "||" } else { "&&" });
                    self.codegen_expression_iife(rhs,  post_string);
                    write!(post_string, ")");
                } else {
                    write!(post_string, "{}", print_binary_op(op));

                    if exp.kind.is_integer() {
                        match op {
                            &BinaryOperator::Add |
                                &BinaryOperator::Sub |
                                &BinaryOperator::Mul |
                                &BinaryOperator::Div => {
                                    write!(post_string, "_int");
                                }
                            _ => {}
                        }
                    }

                    write!(post_string, "(");
                    self.visit_expression(lhs, pre_string, post_string);
                    write!(post_string, ",");
                    self.visit_expression(rhs, pre_string, post_string);
                    write!(post_string, ")");
                }
            }

            Expression::FunctionCall { ref primary, ref arguments } => {
                let tmp_id = self.create_id();
                let mut new_pre_string = "".to_string();
                let mut new_post_string = "".to_string();

                // Print the name of the temp variable in the post_string
                write!(post_string, "ⴵ_{}", tmp_id);

                // Execute function call outside using different post/prestrings
                write!(new_post_string, "var ⴵ_{} = ", tmp_id);

                // Print primary to new_post_string
                self.visit_expression(primary, &mut new_pre_string, &mut new_post_string);

                // Print arguments to new_post_string
                write!(new_post_string, "(");
                for arg in arguments {
                    write!(new_post_string, "deepCopy(");
                    self.visit_expression(arg, &mut new_pre_string, &mut new_post_string);
                    write!(new_post_string, "), ");
                }
                write!(new_post_string, ");");

                // Add all hoisted calls, and the new func call to pre_string.
                write!(pre_string, 
                       "{}{}{}\n", 
                       &mut new_pre_string, 
                       indent(self.indent), 
                       &mut new_post_string);
            }

            Expression::Index { ref primary, ref index } => {

                let mut primary_value = "".to_string();
                let mut index_value = "".to_string();
                self.visit_expression(primary, pre_string, &mut primary_value);
                self.visit_expression(index, pre_string, &mut index_value);

                write!(post_string, "{}[check_bounds({},{}.length,{})]",
                primary_value, index_value, primary_value, expr.line_number);
            }

            Expression::Selector { ref primary, ref name } => {
                self.visit_expression(primary, pre_string, post_string);
                write!(post_string, ".ㆭ{}", name);
            }

            Expression::Append { ref lhs, ref rhs } => {
                write!(post_string, "append(");
                self.visit_expression(lhs, pre_string, post_string);
                write!(post_string, ",");
                self.visit_expression(rhs, pre_string, post_string);
                write!(post_string, ")");
            }

            Expression::TypeCast { ref expr, .. } => {
                if exp.kind.is_string() && expr.kind.is_integer() {
                    write!(post_string, "String.fromCharCode(");
                    self.visit_expression(expr, pre_string, post_string);
                    write!(post_string, ")");
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


fn print_unary_op(op: &UnaryOperator) -> String {
    let mut op_name = "".to_string();
    write!(op_name, "unary_{:?}", op);
    op_name
}

fn print_binary_op(op: &BinaryOperator) -> String {
    let mut op_name = "".to_string();
    write!(op_name, "binary_{:?}", op);
    op_name
}
