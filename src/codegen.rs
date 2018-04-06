use ast::*;

struct CodeGenVisitor {
    indent: u32,

}

impl CodeGenVisitor{

    fn visit_top_level_declaration(&mut self, decl: &TopLevelDeclarationNode){
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
}

pub fn codegen(root: &Program) {

}
