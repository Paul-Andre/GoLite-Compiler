use std::fmt::Write;
use ast;
use ast::*;
use kind;
use kind::*;

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
                    self.init_functions.push(func_name.clone());
                    func_name = format!("{}_{}", name, self.create_id());
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
                indent(self.indent);
                println!("break;")
            },
            Statement::Continue => {
                indent(self.indent);
                println!("continue;")
            },
            Statement::Expression(ref exp) => {
                
            },
            Statement::Return(ref exp) => {
                match exp {
                    &Some(..) => {
                        // DO SOMETHING
                    },
                    &None => {
                        indent(self.indent);
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

            },
            Statement::Println { ref exprs } => {

            },
            Statement::For { ref init, ref condition, ref post, ref body } => {
            },
            Statement::If { ref init, ref condition, ref if_branch, ref else_branch } => {
            },
            Statement::Switch { ref init, ref expr, ref body } => {
            },
            Statement::IncDec { ref expr, .. } => {
            }
        }
    }

    fn visit_expression(&mut self,
                        exp: &ExpressionNode,
                        pre_string: &mut String,
                        post_string: &mut String) {

        match exp.expression {
            Expression::RawLiteral{..} => {
            }

            Expression::Identifier { ref name } => {
                write!(post_string, "{}", name);
            }

            Expression::UnaryOperation { ref op, ref rhs } => {
                write!(post_string, "{}(", print_unary_op(op));
                self.visit_expression(rhs, pre_string, post_string);
                write!(post_string, ")");
            }

            Expression::BinaryOperation { ref op, ref lhs, ref rhs } => {
                write!(post_string, "{}", print_binary_op(op));

                if exp.kind.is_integer() { // TODO: take care of && or ||
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
                self.visit_expression(lhs, pre_string, post_string);
                write!(post_string, ")");
            }

            Expression::FunctionCall { ref primary, ref arguments } => {
                let tmp_id = self.create_id();
                let mut new_pre_string = "".to_string();
                let mut new_post_string = "".to_string();

                // TODO: take care of indentation when writing to pre_string
                // Print the name of the temp variable in the post_string
                write!(post_string, "tmp_{}_", tmp_id.clone());

                // Execute function call outside using different post/prestrings
                write!(new_post_string, "tmp_{}_ = ", tmp_id.clone());

                // Print primary to new_post_string
                self.visit_expression(primary, &mut new_pre_string, &mut new_post_string);

                // Print arguments to new_post_string
                write!(new_post_string, "(");
                for arg in arguments {
                    self.visit_expression(arg, &mut new_pre_string, &mut new_post_string);
                    write!(new_post_string, ", ");
                }
                write!(new_post_string, ")");

                // Add all hoisted calls, and the new func call to pre_string.
                write!(pre_string, "{}{}\n", &mut new_pre_string, &mut new_post_string);
            }

            Expression::Index { ref primary, ref index } => {

                let mut primary_value = "".to_string();
                let mut index_value = "".to_string();
                self.visit_expression(primary, pre_string, &mut primary_value);
                self.visit_expression(index, pre_string, &mut index_value);

                write!(post_string, "{}[check_bounds({},{}.lenght)]", primary_value, index_value, primary_value);
            }

            Expression::Selector { ref primary, ref name } => {
                self.visit_expression(primary, pre_string, post_string);
                write!(post_string, ".{}", name);
            }

            Expression::Append { ref lhs, ref rhs } => {
                write!(post_string, "append(");
                self.visit_expression(lhs, pre_string, post_string);
                write!(post_string, ",");
                self.visit_expression(rhs, pre_string, post_string);
                write!(post_string, ")");
            }

            Expression::TypeCast { .. } => {
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

}

fn indent(size: u32) {

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
