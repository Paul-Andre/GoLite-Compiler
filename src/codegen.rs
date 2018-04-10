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

        self.print_init_calls();

        println!("main();")
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

                if func_name == "init" {
                    func_name = format!("init_{}", self.create_id());
                    self.init_functions.push(func_name.clone());
                }

                let params_string = "".to_string();
                for field in parameters {
                    for id in field.identifiers {
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
            Some(mut values) => {
                let mut pre_string = "".to_string();
                let mut post_string = "".to_string();

                for (name, rhs) in var_spec.names.iter().zip(values.iter_mut()) {
                    write!(post_string, "let {} = ", name);
                    self.visit_expression(&rhs, &mut pre_string, &mut post_string);
                    write!(post_string, "\n");
                }

                println!("{} \n {}", pre_string, post_string);
            }
            None => {
                let mut pre_string = "".to_string();
                for (name, kind) in var_spec.names.iter().zip(evaluated_kinds.iter_mut()) {
                    self.visit_var_initialization(&name, &kind);
                }
            }
        }
    }

    fn visit_var_initialization(&mut self, name: &String, kind: &Kind){
        match kind {
            Kind::Basic(BasicKind::Int) => {}
            Kind::Basic(BasicKind::Float) => {}
            Kind::Array(ref kind, ref length) => {
                // TODO: add make array function in javascript
            }
            Kind::Slice(..) => {
                // TODO: create struct for
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
                write!(post_string, "tmp_{}_", self.create_id());
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

    fn print_init_calls(&mut self){
        for init in self.init_functions.iter(){
            println!(init + "();");
        }
    }

    fn create_id(&mut self) -> String{
        self.id_counter += 1;
        return self.id_counter.to_string()
    }
}

pub fn codegen(root: &Program) {
    let mut visitor = CodeGenVisitor{ indent: 0, id_counter: 0, init_functions: Vec::new() };
    visitor.visit_program(root);
}

fn print_header() {

}

fn indent(size: u32) {

}

fn print_unary_op(op: &UnaryOperator) -> String {
    match op {
        Plus => {
            return "unary_plus".to_string()
        },
        Neg => {
            return "unary_neg".to_string()
        },
        BwCompl => {
            return "unary_bwcompl".to_string()
        },
        Not => {
            return "unary_not".to_string()
        }
    }

}

fn print_binary_op(op: &BinaryOperator) -> String {
    match op {
        Or => {
            return "binary_or".to_string()
        },
        And => {
            return "binary_and".to_string()
        },

        Eq => {
            return "binary_eq".to_string()
        },
        Neq => {
            return "binary_neq".to_string()
        },
        Lt => {
            return "binary_lt".to_string()
        },
        Leq => {
            return "binary_leq".to_string()
        },
        Gt => {
            return "binary_gt".to_string()
        },
        Geq => {
            return "binary_geq".to_string()
        },
        Add => {
            return "binary_add".to_string()
        },
        Sub => {
            return "binary_sub".to_string()
        },
        Mul => {
            return "binary_mul".to_string()
        },
        Div => {
            return "binary_div".to_string()
        },



        BwOr => {
            return "binary_bwor".to_string()
        },
        BwXor => {
            return "binary_bwxor".to_string()
        },
        Mod => {
            return "binary_mod".to_string()
        },
        BwAnd => {
            return "binary_bwand".to_string()
        },
        BwAndNot => {
            return "binary_bwandnot".to_string()
        },
        LShift => {
            return "binary_lshift".to_string()
        },
        RShift => {
            return "binary_rshift".to_string()
        },
    }

}
