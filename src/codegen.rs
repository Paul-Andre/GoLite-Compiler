use ast::*;

struct CodeGenVisitor {
    indent: u32,

}

impl CodeGenVisitor{

    fn visitTopLevelDeclaration(&mut self, decl: &TopLevelDeclarationNode) {

    }

    fn visitStatement(&mut self, stmt: &StatementNode ){


    }

    fn visitExpression(&mut self, expr: &ExpressionNode){

    }
}

pub fn codegen(root: &Program) {

}
