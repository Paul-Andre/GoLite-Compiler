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
        for decl in root.declarations {
            self.visit_top_level_declaration(&decl);
        }
        print_out_the_call_to_init_functions();

        println!("main();")
    }

    fn visit_top_level_declaration(&mut self, decl: &TopLevelDeclarationNode) {
        match decl.top_level_declaration {
            TopLevelDeclaration::VarDeclarations { ref declarations } => {
                for d in declarations.iter(){
                    self.visit_top_level_var_spec(&d)
                }
            }

            TopLevelDeclaration::FunctionDeclaration { ref name, ref parameters, ref return_kind, ref body} => {

                let func_name = name.clone()

                if name == "init" {
                    self.init_functions.push(fun_name.clone());
                    func_name = format!("{}_{}", name, self.create_id());
                }

                let params_string = "";
                for field in parameters {
                    for id in field.identifiers {
                        write!(params_string, "{}, ", id);
                    }
                }


                println!("function {} ( {} ) {{", fun_name, params_string);

                self.indent += 1;
                self.visit_statements(&body);
                self.indent -= 1;

                println!("}}");
            }

            _ => return
        }
    }

    fn visit_top_level_var_spec(&mut self, var_spec: &VarSpec){
        match var_spec.rhs {
            &Some(ref values) => {
                let pre_string = "";
                let post_string = "";

                for (name, rhs) in var_spec.names.iter().zip(values.iter_mut()) {
                    write!(post_string, "let {} = ", name);
                    self.visit_expression(&rhs, &pre_string, &post_string);
                    write!(post_string, "\n");
                }
            }
            &None => {
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

    fn visit_statement(&mut self, stmt: &mut StatementNode) {
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
            Statement::Expression(ref mut exp) => {
                
            },
            Statement::Return(ref mut exp) => {
                match exp {
                    &mut Some(..) => {
                        // DO SOMETHING
                    },
                    &mut None => {
                        indent(self.indent);
                        println!("return;");
                    }
                }
            },
            Statement::ShortVariableDeclaration { ref identifier_list, ref mut expression_list } => {
            },
            Statement::VarDeclarations { ref mut declarations } => {
            },
            Statement::TypeDeclarations { ref mut declarations } => {
                // Nothing
            },
            Statement::Assignment { ref mut lhs, ref mut rhs } => {
            },
            Statement::OpAssignment { ref mut lhs, ref mut rhs, ref mut operator } => {
            },
            Statement::Block(ref mut statements) => {
                for stmt in statements {
                    self.visit_statement(stmt);
                }
            },
            Statement::Print { ref mut exprs } => {

            },
            Statement::Println { ref mut exprs } => {

            },
            Statement::For { ref mut init, ref mut condition, ref mut post, ref mut body } => {
            },
            Statement::If { ref mut init, ref mut condition, ref mut if_branch, ref mut else_branch } => {
            },
            Statement::Switch { ref mut init, ref mut expr, ref mut body } => {
            },
            Statement::IncDec { ref mut expr, .. } => {
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
                match exp.kind.resolve() { // TODO: take care of && or ||
                    Kind::Basic(kind::BasicKind::Int) |
                    Kind::Basic(kind::BasicKind::Rune) => {
                        write!(post_string, "{}_int(", print_binary_op(op));
                        self.visit_expression(lhs, pre_string, post_string);
                        write!(post_string, ",");
                        self.visit_expression(lhs, pre_string, post_string);
                        write!(post_string, ")");
                    },
                    _ => {
                        write!(post_string, "{}(", print_binary_op(op));
                        self.visit_expression(lhs, pre_string, post_string);
                        write!(post_string, ",");
                        self.visit_expression(lhs, pre_string, post_string);
                        write!(post_string, ")");
                    }
                }
            }

            Expression::FunctionCall { .. } => {
                write!(post_string, "tmp_{}_", self.get_id());
                // Execute function call outside: append to prestring
                // print the name of the temp variable
                //
            }

            Expression::Index { ref primary, ref index } => {
                write!(post_string, "index("); // RENAME THIS MAYBE?
                self.visit_expression(primary, pre_string, post_string);
                write!(post_string, ",");
                self.visit_expression(index, pre_string, post_string);
                write!(post_string, ")");
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
    let visitor = CodeGenVisitor{ indent: 0 };

    visitor.visit_program(root);

}

fn print_header() {

}

fn indent(size: u32) {

}

fn print_unary_op(op: &UnaryOperator) -> String {

    "".to_string()
}

fn print_binary_op(op: &BinaryOperator) -> String {

    "".to_string()
}
