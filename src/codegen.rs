use ast::*;

struct CodeGenVisitor {
    indent: u32,
    id_counter: u32,
    init_functions: Vec<String>
}

impl CodeGenVisitor{

    fn visit_top_level_declaration(&mut self, decl: &TopLevelDeclarationNode){
        match decl {
            TopLevelDeclaration::VarDeclarations { ref declarations } => {
                for d in declarations.iter(){
                    self.visit_top_level_var_spec(&d)
                }
            }

            TopLevelDeclaration::FunctionDeclaration { ref name, ref parameters, ref return_kind, ref body} => {

                let fun_name: String = name + self.create_id();

                if name == "init" {
                    self.init_functions.append(fun_name)
                }

                let params_string = parameterrs.join(" , ");

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
                let post_string = "let " + name + " = " ;

                for (name, rhs) in var_spec.names.iter().zip(values.iter_mut()) {
                    self.visit_expression(&rhs, &pre_string, &post_string)
                }
            }
            &None => {

            }
        }


    }


    fn visit_variable_declarations(&mut self, declarations: &mut [VarSpec]) {
        for spec in declarations {
        }
    }

    fn visit_statements(&mut self, statements: &mut [StatementNode]) {
        for s in statements {
            self.visit_statement(s);
        }
    }

    fn visit_function_declaration(&mut self,
                                  name: &str,
                                  params: &mut [Field],
                                  body: &mut [StatementNode]) {

    }

    fn visit_statement(&mut self, stmt: &mut StatementNode) {
        match stmt.statement {
            Statement::Empty => {},
            Statement::Break => {},
            Statement::Continue => {},
            Statement::Expression(ref mut exp) => {
            },
            Statement::Return(ref mut exp) => {
            },
            Statement::ShortVariableDeclaration { ref identifier_list, ref mut expression_list } => {
            },
            Statement::VarDeclarations { ref mut declarations } => {
            },
            Statement::TypeDeclarations { ref mut declarations } => {
            },
            Statement::Assignment { ref mut lhs, ref mut rhs } => {
            },
            Statement::OpAssignment { ref mut lhs, ref mut rhs, ref mut operator } => {
            },
            Statement::Block(ref mut statements) => {
            },
            Statement::Print { ref mut exprs } |
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

    fn visit_expression(exp: &mut ExpressionNode,
                            from_expression_statement: bool){

        match exp.expression {
            Expression::RawLiteral{..} => {
            }

            Expression::Identifier { ref name } => {
            }

            Expression::UnaryOperation { ref op, ref mut rhs } => {
            }

            Expression::BinaryOperation { ref op, ref mut lhs, ref mut rhs } => {
            }

            Expression::FunctionCall { .. } => {
            }

            Expression::Index { ref mut primary, ref mut index } => {
            }

            Expression::Selector { ref mut primary, ref name } => {
            }

            Expression::Append { ref mut lhs, ref mut rhs } => {
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

}
