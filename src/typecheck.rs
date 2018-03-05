use ast::*;
use symbol_table::*;
use std::process::exit;
use std::collections::HashMap;

pub fn typecheck(root: &Program, symbol_table: &SymbolTable) {

}

pub fn typecheck_top_level_declaration(decl: &TopLevelDeclarationNode, symbol_table: &SymbolTable) {
    match decl.top_level_declaration {
        TopLevelDeclaration::VarDeclarations => {

        }
        TopLevelDeclaration::TypeDeclarations => {

        }
        TopLevelDeclaration::FunctionDeclaration => {

        }
    }
}

pub fn typecheck_statement(stmt: &StatementNode, symbol_table: &SymbolTable) {
    match stmt.statement {
        Statement::Empty => return;
        Statement::Break => return;
        Statement::Continue => return;
        Statement::Expression => {

        }
        Statement::Return => {
            
        }
        Statement::ShortVarDeclaration => {

        }
        Statement::VarDeclaration => {

        }
        Statement::Assignment => {

        }
        Statement::OpAssignment => {

        }
        Statement::Block => {

        }
        Statement::Print => {

        }
        Statement::Prinln => {

        }
        Statement::For => {

        }
        Statement::While => {

        }
        Statement::Loop => {

        }
        Statement::If => {

        }
        Statement::Switch => {

        }
        Statement::IncDec => {

        }
    }
}

pub fn typecheck_expression(exp: &ExpressionNode, symbol_table: &SymbolTable) -> Type {
    match exp.expression {
        Expression::RawLiteral => {

        }
        Expression::Identifier => {

        }
        Expression::UnaryOperation => {

        }
        Expression::BinaryOperation => {

        }
        Expression::FunctionCall => {

        }
        Expression::Index => {

        }
        Expression::Selector => {

        }
        Expression::Append => {

        }
        Expression::TypeCast => {

        }
    }
}

pub fn typecheck_expression_list(exprs: &Vec<ExpressionNode>, 
                                 symbol_table: &SymbolTable) -> Vec<Type> {
    for exp in exprs.iter() {
        // Do something
    }
}
