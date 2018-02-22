use std::env;
use ast::*;

fn exp_identifier(line: u32, str: String) -> Box<ExpressionNode> {
    Box::new(
        ExpressionNode {
            location: SourceLocation{ line_number: line }, 
            expression: Expression::Identifier{ name: str},
            kind: Kind::Undefined 
        }
    )
}

/*
fn exp_rawliteral(line: u32, str: String) -> Box<ExpressionNode> {
    Box::new(ExpressionNode {location: line, expression: Expression::RawLiteral{ value: str }})
}

fn exp_binoperation(line: u32, str: operator, left: Box<ExpressionNode>, right: Box<ExpressionNode>) -> Box<ExpressionNode> {
    Box::new(
        ExpressionNode {
            location: line, 
            expression: Expression::BinOperation{ op: str, lhs: left, rhs: right }
        }
    )
}

fn exp_unoperation(line: u32, str: operator, right: Box<ExpressionNode>) -> Box<ExpressionNode> {
    Box::new(
        ExpressionNode {
            location: line, 
            expression: Expression::UnOperation{ op: str, rhs: right }
        }
    )
}

fn exp_index(line: u32, p: Box<ExpressionNode>, i: Box<ExpressionNode>) -> Box<ExpressionNode> {
    Box::new(
        ExpressionNode {
            location: line, 
            expression: Expression::Index{ primary: p, index: i }
        }
    )
}

fn exp_selector(line: u32, p: Box<ExpressionNode>, str: String) -> Box<ExpressionNode> {
    Box::new(
        ExpressionNode {
            location: line, 
            expression: Expression::Selector{ primary: p, name: str }
        }
    )
}

fn exp_functioncall(line: u32, p: Box<ExpressionNode>, args: Vec<ExpressionNode>) -> Box<ExpressionNode> {
    Box::new(
        ExpressionNode {
            location: line, 
            expression: Expression::FunctionCall{ primary: p, arguments: args }
        }
    )
}

fn exp_append(line: u32, left: Box<ExpressionNode>, right: Box<ExpressionNode>) -> Box<ExpressionNode> {
    Box::new(
        ExpressionNode {
            location: line, 
            expression: Expression::Append{ lhs: left, rhs: right }
        }
    )
}

fn exp_typecast(line: u32, exp: Box<ExpressionNode>) -> Box<ExpressionNode> {
    Box::new(
        ExpressionNode {
            location: line, 
            expression: Expression::TypeCast{ expr: exp }
        }
    )
}
*/
